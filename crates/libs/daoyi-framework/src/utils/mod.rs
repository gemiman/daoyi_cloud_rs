use argon2::{
    Argon2, PasswordHash,
    password_hash::{SaltString, rand_core::OsRng},
};
use config::{Config, ConfigError, File};
use rand::Rng;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::iter;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
    iter::repeat(())
        .map(|_| rand::rng().sample(rand::distr::Alphanumeric))
        .map(char::from)
        .take(limit)
        .collect()
}

pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<()> {
    let hash = PasswordHash::new(&password_hash)
        .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    let result = hash.verify_password(&[&Argon2::default()], password);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("invalid password")),
    }
}

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(PasswordHash::generate(Argon2::default(), password, &salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}

/// Load a YAML configuration file from a `resources` directory into the provided struct type.
///
/// `base_name` is the file stem (for example, `application` to read `application.yaml`), and
/// `profile` can override the active profile. If `profile` is `None`, the function will look for
/// `DAOYI_PROFILE` or `SPRING_PROFILES_ACTIVE` environment variables. The base YAML file is
/// required; `application-{profile}.yaml` and `application-local.yaml` are loaded when present.
pub fn load_yaml_config<T>(
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
        builder = builder.add_source(
            File::from(resources_dir.join(format!("application-{profile_name}.yaml")))
                .required(false),
        );
        if !base_name.eq_ignore_ascii_case("application") {
            builder = builder.add_source(
                File::from(resources_dir.join(format!("{base_name}-{profile_name}.yaml")))
                    .required(false),
            );
        }
        builder = builder.set_override("spring.profiles.active", profile_name.clone())?;
    } else {
        builder = builder
            .add_source(File::from(resources_dir.join("application-local.yaml")).required(false));
        if !base_name.eq_ignore_ascii_case("application") {
            builder = builder.add_source(
                File::from(resources_dir.join(format!("{base_name}-local.yaml"))).required(false),
            );
        }
        builder = builder.set_default("spring.profiles.active", "local")?;
    }

    let built = builder.build()?;
    let mut value = built.try_deserialize::<Value>()?;
    let original = value.clone();
    resolve_placeholders(&mut value, &original);
    serde_json::from_value(value).map_err(|err| ConfigError::Message(err.to_string()))
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
