use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProjectConfig {
    pub project_name: String,
    pub reference_directory: PathBuf,
    pub file_extensions: Vec<String>,
    pub ollama_model: String,
    pub setup_complete: bool,
}

impl Default for UserProjectConfig {
    fn default() -> Self {
        Self {
            project_name: "My Project".to_string(),
            reference_directory: PathBuf::from("."),
            file_extensions: vec![
                "rs".to_string(),
                "toml".to_string(),
                "md".to_string(),
                "txt".to_string(),
            ],
            ollama_model: "qwen2.5-coder:7b".to_string(),
            setup_complete: false,
        }
    }
}

pub fn load_user_config() -> Result<UserProjectConfig> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
        .join("local-doc-assistant");
    
    let config_file = config_dir.join("config.json");
    
    if config_file.exists() {
        let content = std::fs::read_to_string(&config_file)?;
        let config: UserProjectConfig = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        Ok(UserProjectConfig::default())
    }
}

pub fn save_user_config(config: &UserProjectConfig) -> Result<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
        .join("local-doc-assistant");
    
    std::fs::create_dir_all(&config_dir)?;
    
    let config_file = config_dir.join("config.json");
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(&config_file, content)?;
    
    Ok(())
}
