use crate::theme::{Theme, ThemeMode, get_theme_by_name, get_default_theme};
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme_name")]
    pub theme_name: String,
    
    #[serde(default = "default_theme_mode")]
    pub theme_mode: String,
}

fn default_theme_name() -> String {
    "default".to_string()
}

fn default_theme_mode() -> String {
    "light".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme_name: default_theme_name(),
            theme_mode: default_theme_mode(),
        }
    }
}

impl Config {
    /// Load config from standard locations
    pub fn load() -> Self {
        let config_paths = Self::get_config_paths();
        
        for path in config_paths {
            if path.exists() {
                match Self::load_from_file(&path) {
                    Ok(config) => {
                        eprintln!("Loaded config from: {}", path.display());
                        return config;
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load config from {}: {}", path.display(), e);
                        eprintln!("Using default configuration.");
                    }
                }
            }
        }
        
        // No config file found, use defaults
        Self::default()
    }
    
    /// Get list of config file paths in priority order
    fn get_config_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // 1. ~/.config/t-meter/config.json (Linux/macOS)
        if let Some(proj_dirs) = ProjectDirs::from("", "", "t-meter") {
            paths.push(proj_dirs.config_dir().join("config.json"));
        }
        
        // 2. ~/Library/Application Support/t-meter/config.json (macOS alternative)
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join("Library/Application Support/t-meter/config.json"));
        }
        
        // 3. Current directory ./t-meter.json (for testing)
        paths.push(PathBuf::from("./t-meter.json"));
        
        paths
    }
    
    /// Load config from a specific file
    fn load_from_file(path: &PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let config: Config = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        
        Ok(config)
    }
    
    /// Get the theme based on config
    pub fn get_theme(&self) -> Theme {
        // Try to get the theme by name
        if let Some(theme) = get_theme_by_name(&self.theme_name) {
            theme
        } else {
            eprintln!("Warning: Theme '{}' not found, using default theme", self.theme_name);
            get_default_theme()
        }
    }
    
    /// Get the theme mode
    pub fn get_theme_mode(&self) -> ThemeMode {
        self.theme_mode.parse().unwrap_or_else(|_| {
            eprintln!("Warning: Invalid theme mode '{}', using Light mode", self.theme_mode);
            ThemeMode::Light
        })
    }
    
    /// Save config to the primary config location
    pub fn save(&self) -> Result<()> {
        let config_paths = Self::get_config_paths();
        
        if let Some(path) = config_paths.first() {
            // Create parent directory if it doesn't exist
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
            }
            
            let json = serde_json::to_string_pretty(self)
                .context("Failed to serialize config")?;
            
            fs::write(path, json)
                .with_context(|| format!("Failed to write config file: {}", path.display()))?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No valid config path found"))
        }
    }
}

// Helper function to get home directory (for compatibility)
mod dirs {
    use std::path::PathBuf;
    
    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}
