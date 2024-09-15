use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub uri: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Watchdog {
    pub api_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub server: Server,
    pub watchdog: Watchdog,
}

pub fn load_config() -> Result<Config, figment::Error> {
    Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("STREAM_"))
        .extract()
}
