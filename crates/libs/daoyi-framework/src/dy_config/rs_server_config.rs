use crate::dy_config::jwt_config::JwtConfig;
use crate::dy_config::tls_config::TlsConfig;
use crate::dy_config::{DbConfig, LogConfig};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct RsServerConfig {
    pub db: Option<DbConfig>,
    pub log: Option<LogConfig>,
    pub jwt: Option<JwtConfig>,
    pub tls: Option<TlsConfig>,
}

impl RsServerConfig {
    pub fn get_jwt(&self) -> &JwtConfig {
        self.jwt.as_ref().expect("should be set")
    }
}
