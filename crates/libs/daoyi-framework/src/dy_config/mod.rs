use std::sync::OnceLock;

mod log_config;
pub use log_config::LogConfig;
pub mod db_config;
pub mod jwt_config;
pub mod rs_server_config;
pub mod spring;
pub mod tls_config;

use crate::dy_config::spring::DyConfiguration;
pub use db_config::DbConfig;

pub static CONFIG: OnceLock<DyConfiguration> = OnceLock::new();

pub async fn init() {
    let configuration = DyConfiguration::load(Option::<&str>::None, None, Option::<&str>::None)
        .await
        .expect("Failed to load config");
    println!("-------------Configuration s -----------------------");
    println!("{configuration:#?}");
    println!("-------------Configuration e -----------------------");
    let rs_config = configuration.get_rs();
    if let Some(log) = rs_config.log.as_ref() {
        let _guard = log.guard();
        tracing::info!("log level: {}", log.filter_level);
    }
    CONFIG.set(configuration).expect("dy_config should be set");
}
pub fn get() -> &'static DyConfiguration {
    CONFIG.get().expect("dy_config should be set")
}

#[allow(dead_code)]
pub fn default_false() -> bool {
    false
}
#[allow(dead_code)]
pub fn default_true() -> bool {
    true
}
