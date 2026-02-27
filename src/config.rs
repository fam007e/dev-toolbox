use dirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub unicode_data_path: String,
    pub blocks_path: String,
    pub cache_db_path: String,
    pub github_api_base_url: String,
}

impl Config {
    pub fn default_with_paths() -> Self {
        let mut data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push("dev-toolbox");
        let _ = fs::create_dir_all(&data_dir);

        Self {
            unicode_data_path: "UnicodeData.txt".to_string(),
            blocks_path: "Blocks.txt".to_string(),
            cache_db_path: data_dir.join("cache.db").to_string_lossy().to_string(),
            github_api_base_url: "https://api.github.com".to_string(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.push("dev-toolbox");
        let _ = fs::create_dir_all(&config_dir);
        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default_with_paths();
            let content = toml::to_string(&config)?;
            fs::write(config_path, content)?;
            Ok(config)
        }
    }
}
