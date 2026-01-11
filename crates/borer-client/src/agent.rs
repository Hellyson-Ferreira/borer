use crate::config::AppConfig;
use crate::tunnel;

pub async fn run_agent(port: u16) {
    let config =
        AppConfig::load_config("config.toml").expect("You must be logged in to run the agent.");

    tunnel::run(
        &config.host,
        &config.token,
        &format!("http://localhost:{}", port),
    )
    .await
    .unwrap();
}
