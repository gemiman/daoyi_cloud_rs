use argon2::password_hash::phc::Salt;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use config::{Config, ConfigError, File};
use nacos_sdk::api::{config::ConfigServiceBuilder, error as nacos_error, props::ClientProps};
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use serde_yaml::Deserializer as YamlDeserializer;
use std::iter;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
    iter::repeat(())
        .map(|_| rand::rng().sample(Alphanumeric))
        .map(char::from)
        .take(limit)
        .collect()
}

pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<()> {
    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("invalid password")),
    }
}

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = Salt::generate(); // Note: needs the `getrandom` feature of `argon2` enabled
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}

/// Load a YAML configuration file from a `resources` directory into the provided struct type.
///
/// `base_name` is the file stem (for example, `application` to read `application.yaml`), and
/// `profile` can override the active profile. If `profile` is `None`, the function will look for
/// `DAOYI_PROFILE` or `SPRING_PROFILES_ACTIVE` environment variables. The base YAML file is
/// required; `application-{profile}.yaml` and `application-local.yaml` are loaded when present.
pub async fn load_yaml_config<T>(
    resources_dir: Option<impl AsRef<Path>>,
    base_name: Option<&str>,
    profile: Option<impl AsRef<str>>,
) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
{
    let resources_dir = match resources_dir {
        Some(p) => p.as_ref().to_path_buf(),
        None => PathBuf::from("resources"),
    };
    let base_name = base_name.unwrap_or("application");
    let active_profile = profile
        .map(|p| p.as_ref().to_string())
        .or_else(|| std::env::var("DAOYI_PROFILE").ok())
        .or_else(|| std::env::var("SPRING_PROFILES_ACTIVE").ok());

    let mut builder = Config::builder()
        .add_source(File::from(resources_dir.join("application.yaml")).required(false));
    if !base_name.eq_ignore_ascii_case("application") {
        builder = builder.add_source(
            File::from(resources_dir.join(format!("{base_name}.yaml"))).required(false),
        );
    }

    if let Some(profile_name) = &active_profile {
        builder = builder.set_override("spring.profiles.active", profile_name.clone())?;
    } else {
        builder = builder.set_default("spring.profiles.active", "local")?;
    }

    let built = builder.build()?;
    let mut value = built.try_deserialize::<Value>()?;
    let original = value.clone();
    resolve_placeholders(&mut value, &original);
    merge_imports(&mut value, resources_dir.as_path()).await?;
    let resolved_root = value.clone();
    resolve_placeholders(&mut value, &resolved_root);
    serde_json::from_value(value).map_err(|err| ConfigError::Message(err.to_string()))
}

async fn merge_imports(root: &mut Value, resources_dir: &Path) -> Result<(), ConfigError> {
    let imports: Vec<String> = root
        .get("spring")
        .and_then(|s| s.get("config"))
        .and_then(|c| c.get("import"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    for import_str in imports {
        let optional = import_str.starts_with("optional:");
        let trimmed = import_str.trim_start_matches("optional:");

        if let Some(classpath) = trimmed.strip_prefix("classpath:") {
            let path = resources_dir.join(classpath);
            match load_value_from_file(&path) {
                Ok(mut imported) => {
                    resolve_placeholders(&mut imported, root);
                    merge_values(root, &imported);
                }
                Err(e) if optional && matches!(e, ConfigError::NotFound(_)) => {
                    eprintln!("optional classpath config not found: {}", classpath);
                }
                Err(e) => return Err(e),
            }
        } else if let Some(data_id) = trimmed.strip_prefix("nacos:") {
            match load_value_from_nacos(root, data_id).await {
                Ok(Some(mut imported)) => {
                    resolve_placeholders(&mut imported, root);
                    merge_values(root, &imported);
                }
                Ok(None) if optional => {
                    eprintln!("optional nacos config not found, skipping: {}", data_id);
                }
                Ok(None) => {
                    return Err(ConfigError::NotFound(format!(
                        "nacos config {} not found",
                        data_id
                    )));
                }
                Err(err) if optional => {
                    eprintln!(
                        "error = {err:#?}, optional nacos config failed to load: {}",
                        data_id
                    );
                }
                Err(err) => return Err(err),
            }
        }
    }
    Ok(())
}

async fn load_value_from_nacos(root: &Value, data_id: &str) -> Result<Option<Value>, ConfigError> {
    let Some(nacos_conf) = root
        .get("spring")
        .and_then(|s| s.get("cloud"))
        .and_then(|c| c.get("nacos"))
    else {
        return Err(ConfigError::Message(
            "spring.cloud.nacos is required for nacos imports".to_string(),
        ));
    };

    let server_addr = nacos_conf
        .get("server-addr")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| ConfigError::Message("spring.cloud.nacos.server-addr is missing".into()))?;

    let config_namespace = nacos_conf
        .get("config")
        .and_then(|v| v.get("namespace"))
        .and_then(|v| v.as_str())
        .or_else(|| {
            nacos_conf
                .get("discovery")
                .and_then(|v| v.get("namespace"))
                .and_then(|v| v.as_str())
        })
        .unwrap_or("")
        .to_string();

    let config_group = nacos_conf
        .get("config")
        .and_then(|v| v.get("group"))
        .and_then(|v| v.as_str())
        .unwrap_or("DEFAULT_GROUP")
        .to_string();

    let username = nacos_conf
        .get("username")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    let password = nacos_conf
        .get("password")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    let app_name = root
        .get("spring")
        .and_then(|s| s.get("application"))
        .and_then(|a| a.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("daoyi-cloud-rs")
        .to_string();

    let mut client_props = ClientProps::new()
        .server_addr(server_addr)
        .namespace(config_namespace)
        .app_name(app_name)
        .env_first(false);

    if let Some(u) = username {
        client_props = client_props.auth_username(u);
    }
    if let Some(p) = password {
        client_props = client_props.auth_password(p);
    }

    let config_service = ConfigServiceBuilder::new(client_props)
        .enable_auth_plugin_http()
        .build()
        .map_err(|e| ConfigError::Message(e.to_string()))?;

    match config_service
        .get_config(data_id.to_string(), config_group.clone())
        .await
    {
        Ok(resp) => load_value_from_str(resp.content()).map(Some),
        Err(nacos_error::Error::ConfigNotFound(_)) => Ok(None),
        Err(err) => Err(ConfigError::Message(err.to_string())),
    }
}

fn load_value_from_str(content: &str) -> Result<Value, ConfigError> {
    let mut acc = Value::Null;
    for doc in YamlDeserializer::from_str(content) {
        let yaml_val: serde_yaml::Value =
            serde_yaml::Value::deserialize(doc).map_err(|e| ConfigError::Message(e.to_string()))?;
        let json_val =
            serde_json::to_value(yaml_val).map_err(|e| ConfigError::Message(e.to_string()))?;
        if acc.is_null() {
            acc = json_val;
        } else {
            merge_values(&mut acc, &json_val);
        }
    }

    if acc.is_null() {
        Err(ConfigError::Message(
            "empty yaml content from nacos".to_string(),
        ))
    } else {
        Ok(acc)
    }
}

fn load_value_from_file(path: &Path) -> Result<Value, ConfigError> {
    let config = Config::builder().add_source(File::from(path)).build()?;
    config.try_deserialize::<Value>()
}

fn merge_values(base: &mut Value, overlay: &Value) {
    if overlay.is_null() {
        // Skip nulls to avoid wiping existing values
        return;
    }
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (k, v) in overlay_map {
                match base_map.get_mut(k) {
                    Some(base_val) => merge_values(base_val, v),
                    None => {
                        base_map.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        (base_slot, overlay_val) => {
            *base_slot = overlay_val.clone();
        }
    }
}

fn resolve_placeholders(value: &mut Value, root: &Value) {
    match value {
        Value::String(s) => {
            let replaced = replace_placeholders(s, root);
            *s = replaced;
        }
        Value::Array(items) => {
            for item in items {
                resolve_placeholders(item, root);
            }
        }
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                resolve_placeholders(v, root);
            }
        }
        _ => {}
    }
}

fn replace_placeholders(input: &str, root: &Value) -> String {
    let mut output = String::new();
    let mut idx = 0usize;
    while let Some(start) = input[idx..].find("${") {
        let abs_start = idx + start;
        output.push_str(&input[idx..abs_start]);
        if let Some(end_rel) = input[abs_start + 2..].find('}') {
            let abs_end = abs_start + 2 + end_rel;
            let key = &input[abs_start + 2..abs_end];
            let replacement = lookup_path(root, key)
                .and_then(value_as_string)
                .unwrap_or_else(|| format!("${{{}}}", key));
            output.push_str(&replacement);
            idx = abs_end + 1;
        } else {
            output.push_str(&input[abs_start..]);
            return output;
        }
    }
    output.push_str(&input[idx..]);
    output
}

fn lookup_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for part in path.split('.') {
        match current {
            Value::Object(map) => {
                current = map.get(part)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

fn value_as_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => Some(String::new()),
        _ => None,
    }
}
