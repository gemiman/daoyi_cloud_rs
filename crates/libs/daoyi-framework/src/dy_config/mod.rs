use std::sync::OnceLock;

use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

mod log_config;
pub use log_config::LogConfig;
pub mod db_config;
pub mod jwt_config;
pub mod nacos_config;
pub mod tls_config;

use crate::dy_config::jwt_config::JwtConfig;
use crate::dy_config::tls_config::TlsConfig;
pub use db_config::DbConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn init(config_file: &str) {
    let raw_config = Figment::new()
        .merge(Toml::string(config_file))
        .merge(Env::prefixed("APP_").global());

    let mut config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your dy_config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    if config.db.url.is_empty() {
        config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
    }
    if config.db.url.is_empty() {
        eprintln!("DATABASE_URL is not set");
        std::process::exit(1);
    }
    CONFIG.set(config).expect("dy_config should be set");
}
pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("dy_config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    pub db: DbConfig,
    pub log: LogConfig,
    pub jwt: JwtConfig,
    pub tls: Option<TlsConfig>,
}

#[allow(dead_code)]
pub fn default_false() -> bool {
    false
}
#[allow(dead_code)]
pub fn default_true() -> bool {
    true
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}
