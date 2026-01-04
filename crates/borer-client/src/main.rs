mod tunnel;
mod proxy;
mod cli;
mod agent;

#[tokio::main]
async fn main() {
    tunnel::run("ws://localhost:3002/ws", "http://localhost:3001")
        .await
        .unwrap();
}
