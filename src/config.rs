use serde::Deserialize;
use config::{Config as RawConfig, ConfigError, Environment};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = RawConfig::builder()
            .add_source(Environment::default().separator("__"))
            .build()?;

        cfg.try_deserialize()
    }
}
