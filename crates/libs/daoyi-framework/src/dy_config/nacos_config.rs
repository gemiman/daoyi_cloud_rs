use crate::dy_config::default_false;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NacosConfig {
    #[serde(default = "default_false")]
    enable: bool,
    server_addr: String,
    namespace: String,
    app_name: String,
    group: String,
    auth_username: String,
    auth_password: String,
}
