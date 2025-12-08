use crate::utils::load_yaml_config;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct DyConfiguration {
    pub spring: Option<SpringConfig>,
    pub logging: Option<LoggingConfig>,
    pub server: Option<ServerConfig>,
    pub knife4j: Option<KfConfig>,
    pub yudao: Option<YuDaoConfig>,
}

impl DyConfiguration {
    pub async fn load(
        resources_dir: Option<impl AsRef<Path>>,
        base_name: Option<&str>,
        profile: Option<impl AsRef<str>>,
    ) -> anyhow::Result<Self> {
        load_yaml_config(resources_dir, base_name, profile)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpringConfig {
    pub profiles: Option<SpringProfileConfig>,
    pub application: Option<ApplicationConfig>,
    pub http: Option<HttpConfig>,
    pub jackson: Option<JacksonConfig>,
    pub main: Option<MainConfig>,
    pub config: Option<SpringImportConfig>,
    pub cloud: Option<CloudConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct GatewayCloudConfig {
    pub server: Option<GatewayServerConfig>,
    #[serde(rename = "x-forwarded")]
    pub x_forwarded: Option<XForwardedConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct GatewayServerConfig {
    pub webflux: Option<WebfluxConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct WebfluxConfig {
    pub routes: Option<Vec<RouteDefinition>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct RouteDefinition {
    pub id: Option<String>,
    pub uri: Option<String>,
    pub predicates: Option<Vec<String>>,
    pub filters: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct XForwardedConfig {
    #[serde(rename = "prefix-enabled")]
    pub prefix_enabled: Option<bool>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ProfilesConfig {
    pub active: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub codecs: Option<HttpCodecConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct HttpCodecConfig {
    #[serde(rename = "max-in-memory-size")]
    pub max_in_memory_size: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct JacksonConfig {
    pub serialization: Option<JacksonSerializationConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct JacksonSerializationConfig {
    #[serde(rename = "write-dates-as-timestamps")]
    pub write_dates_as_timestamps: Option<bool>,
    #[serde(rename = "write-date-timestamps-as-nanoseconds")]
    pub write_date_timestamps_as_nanoseconds: Option<bool>,
    #[serde(rename = "write-durations-as-timestamps")]
    pub write_durations_as_timestamps: Option<bool>,
    #[serde(rename = "fail-on-empty-beans")]
    pub fail_on_empty_beans: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct MainConfig {
    #[serde(rename = "allow-circular-references")]
    pub allow_circular_references: Option<bool>,
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

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingFileConfig {
    pub name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct KfConfig {
    pub gateway: Option<KfGatewayConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct KfGatewayConfig {
    pub enabled: Option<bool>,
    pub routes: Option<Vec<KfGatewayRoute>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct KfGatewayRoute {
    pub name: Option<String>,
    #[serde(rename = "service-name")]
    pub service_name: Option<String>,
    pub url: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct YuDaoConfig {
    pub info: Option<YuDaoInfo>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct YuDaoInfo {
    pub version: Option<String>,
}
