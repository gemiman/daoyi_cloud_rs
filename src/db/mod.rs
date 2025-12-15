use std::sync::OnceLock;

use rbatis::RBatis;

use crate::config::DbConfig;

pub static RBATIS_ENGINE: OnceLock<RBatis> = OnceLock::new();

pub async fn init(config: &DbConfig) {
    let rb = RBatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, &config.url)
        .unwrap();
    let sql_file = "./data/init.sql";

    if sql_file != "" {
        let sql = std::fs::read_to_string(sql_file).unwrap();
        let _ = rb.exec(&sql, vec![]).await;
    }
    RBATIS_ENGINE.set(rb).expect("rbatis should be set");
}

pub fn engine() -> &'static RBatis {
    RBATIS_ENGINE.get().expect("rbatis should be initialized")
}