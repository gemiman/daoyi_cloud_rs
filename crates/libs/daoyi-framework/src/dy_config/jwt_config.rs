use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: i64,
}