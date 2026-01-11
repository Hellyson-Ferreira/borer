use crate::tunnel;

pub async fn run_agent(port: u16) {
    tunnel::run(
        "ws://localhost:3002/ws",
        &format!("http://localhost:{}", port),
    )
    .await
    .unwrap();
}
