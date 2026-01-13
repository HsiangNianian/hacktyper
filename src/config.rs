use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_endpoint: String,
    pub api_key: String,
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            api_key: "YOUR_API_KEY_HERE".to_string(),
            model: "gpt-3.5-turbo".to_string(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let proj_dirs = ProjectDirs::from("com", "HsiangNianian", "hacktyper")
        .context("Could not determine config directory")?;
    let config_dir = proj_dirs.config_dir();
    
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    
    let config_path = config_dir.join("config.toml");
    
    if !config_path.exists() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, toml_str)?;
        eprintln!("Created default config file at {:?}", config_path);
        return Ok(config);
    }
    
    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config file")?;
        
    Ok(config)
}
