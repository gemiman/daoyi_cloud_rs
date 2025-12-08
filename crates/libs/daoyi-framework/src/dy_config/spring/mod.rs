use crate::utils::load_yaml_config;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct DyConfiguration {
    pub spring: Option<SpringConfig>,
    pub logging: Option<LoggingConfig>,
}

impl DyConfiguration {
    pub fn load(
        resources_dir: Option<impl AsRef<Path>>,
        base_name: Option<&str>,
        profile: Option<impl AsRef<str>>,
    ) -> anyhow::Result<Self> {
        load_yaml_config(resources_dir, base_name, profile).map_err(|e| anyhow::anyhow!(e))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpringConfig {
    pub profiles: Option<SpringProfileConfig>,
    pub application: Option<ApplicationConfig>,
    pub config: Option<SpringImportConfig>,
    pub cloud: Option<CloudConfig>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct SpringProfileConfig {
    pub active: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationConfig {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpringImportConfig {
    pub import: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudConfig {
    pub nacos: Option<NacosConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NacosConfig {
    pub server_addr: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub discovery: Option<NacosNamespace>,
    pub config: Option<NacosNamespace>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NacosNamespace {
    pub namespace: Option<String>,
    pub group: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: Option<HashMap<String, String>>,
}
