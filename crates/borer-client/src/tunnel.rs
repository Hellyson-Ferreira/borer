use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use borer_core::protocol::{TunnelHttpRequest, TunnelHttpResponse, TunnelMessage};

use reqwest::Client;

async fn handle_http_request(
    client: &Client,
    base: &str,
    req: TunnelHttpRequest,
) -> TunnelHttpResponse {
    let url = match &req.query {
        Some(q) => format!("{}{}?{}", base, req.path, q),
        None => format!("{}{}", base, req.path),
    };

    println!("Received HTTP request: {base} {} {}", req.method, req.path);

    let method = req.method.parse().unwrap_or(reqwest::Method::GET);

    let mut request = client.request(method, url);

    for (k, v) in req.headers {
        request = request.header(&k, &v);
    }

    let res = match request.body(req.body).send().await {
        Ok(r) => r,
        Err(e) => {
            println!("Error sending HTTP request: {}", e);

            return TunnelHttpResponse {
                id: req.id,
                status: 502,
                headers: vec![],
                body: format!("upstream error: {e}").into_bytes(),
            };
        }
    };

    let status = res.status().as_u16();

    let headers = res
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let body = res.bytes().await.unwrap_or_default().to_vec();

    let result = TunnelHttpResponse {
        id: req.id,
        status,
        headers,
        body,
    };
    println!(
        "Handled request: {} {}, response status: {}",
        req.method, req.path, status
    );

    result
}

pub async fn run(ws_url: &str, token: &str, local_base: &str) -> anyhow::Result<()> {
    // TODO: change this to headers for better security
    // TODO: add support ws when in development mode
    let ws_url = format!("ws://{}/ws?token={}", ws_url, token);

    println!("Connecting to {}", ws_url);

    let (ws, _) = connect_async(ws_url).await?;
    println!("Connected to server");

    let (mut sender, mut receiver) = ws.split();
    let client = Client::new();

    while let Some(msg) = receiver.next().await {
        let msg = msg?;

        match msg {
            Message::Binary(bytes) => {
                let tunnel_msg = TunnelMessage::from_bytes(&bytes)?;

                match tunnel_msg {
                    TunnelMessage::HttpRequest(req) => {
                        let resp = handle_http_request(&client, local_base, req).await;

                        let out = TunnelMessage::HttpResponse(resp);
                        let bytes = out.to_bytes()?;

                        sender.send(Message::Binary(bytes.into())).await?;
                    }
                    _ => {
                        println!("Unexpected message: {:?}", tunnel_msg);
                    }
                }
            }
            Message::Close(_) => {
                println!("Server closed the connection");
                break;
            }
            _ => {
                println!("Ignoring non-binary message");
            }
        }
    }

    println!("Disconnected");
    Ok(())
}
