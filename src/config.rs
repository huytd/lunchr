use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigEntry {
    pub name: String,
    pub cwd: String,
    pub start_command: String,
    pub kill_command: String,
    pub health_check_pattern: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub commands: Vec<ConfigEntry>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().ok_or("Could not get home directory")?;
        let config_path = home_dir.join(".lunchr.toml");
        if !config_path.exists() {
            return Err(format!("Config file not found").into());
        }
        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
