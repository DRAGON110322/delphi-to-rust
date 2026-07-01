use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// Получение имени
fn get_machine_name() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "Unknown-Machine".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    pub autoload: bool,
    pub allow_load: bool,
    pub available_for_clients: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub main_port: u16,
    pub additional_port: u16,
    pub ping_enabled: bool,
    pub ping_interval: u32,
    pub autoload_windows: bool,
    pub machine_name: String,
    pub chbase_path: String,
    pub projects: Vec<ProjectSettings>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            main_port: 47073,
            additional_port: 4707,
            ping_enabled: true,
            ping_interval: 10,
            autoload_windows: false,
            machine_name: get_machine_name(),
            chbase_path: "Z:\\Monitor\\chbase".to_string(),
            projects: Vec::new(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    exe_dir.join("server_config.json")
}

pub fn load_config() -> ServerConfig {
    let config_path = get_config_path();

    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_json::from_str::<ServerConfig>(&content) {
                    Ok(config) => {
                        return config;
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    ServerConfig::default()
}

pub fn save_config(config: &ServerConfig) -> Result<(), String> {
    let config_path = get_config_path();

    let json = serde_json::to_string_pretty(config)
        .map_err(|_| "Error")?;

    fs::write(&config_path, json)
        .map_err(|_| "Error")?;

    Ok(())
}