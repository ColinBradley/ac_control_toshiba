use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub username: String,
    pub password: String,
}

const CONFIG_PATH: &str = "./config.toml";

pub fn get_config() -> Result<Config> {
    let config_value = std::fs::read_to_string(CONFIG_PATH)
        .with_context(|| format!("Couldn't read the file \"{}\"", CONFIG_PATH))?;

    let config: Config = toml::from_str(&config_value).with_context(|| {
        format!(
            "Invalid config \"{}\". It should look something like: \n{}",
            CONFIG_PATH,
            toml::to_string_pretty(&Config {
                username: "dave".to_string(),
                password: "dave!".to_string()
            })
            .unwrap()
        )
    })?;

    Ok(config)
}
