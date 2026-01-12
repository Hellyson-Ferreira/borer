use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BorerServerConfig {
    pub database_url: String,
    pub master_token: String,
}

pub fn load_config() -> BorerServerConfig {
    Config::builder()
        .add_source(
            Environment::default(), /*.prefix("BORER").separator("__")*/
        )
        .build()
        .expect("Cannot build config")
        .try_deserialize()
        .expect("Cannot deserialize config")
}
