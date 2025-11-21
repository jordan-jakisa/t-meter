use crate::theme::{Theme, ThemeMode, get_theme_by_name, get_default_theme};
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressBarStyle {
    Gradient,
    Grainy,
    Analog,
}

impl ProgressBarStyle {
    pub fn cycle(&self) -> Self {
        match self {
            ProgressBarStyle::Gradient => ProgressBarStyle::Grainy,
            ProgressBarStyle::Grainy => ProgressBarStyle::Analog,
            ProgressBarStyle::Analog => ProgressBarStyle::Gradient,
        }
    }
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        ProgressBarStyle::Analog
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme_name")]
    pub theme_name: String,
    
    #[serde(default = "default_theme_mode")]
    pub theme_mode: String,

    #[serde(default)]
    pub progress_bar_style: ProgressBarStyle,

    #[serde(default = "default_wake_up_time")]
    pub wake_up_time: String,

    #[serde(default = "default_bed_time")]
    pub bed_time: String,

    #[serde(default)]
    pub markers: Vec<MarkerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerConfig {
    pub label: String,
    pub time: String,
}

fn default_theme_name() -> String {
    "default".to_string()
}

fn default_theme_mode() -> String {
    "light".to_string()
}

fn default_wake_up_time() -> String {
    "07:00".to_string()
}

fn default_bed_time() -> String {
    "23:00".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme_name: default_theme_name(),
            theme_mode: default_theme_mode(),
            progress_bar_style: ProgressBarStyle::default(),
            wake_up_time: default_wake_up_time(),
            bed_time: default_bed_time(),
            markers: Vec::new(),
        }
    }
}

impl Config {
    /// Load config from standard locations
    pub fn load() -> Self {
        let config_paths = Self::get_config_paths();
        
        // Check if any config file exists
        let config_exists = config_paths.iter().any(|p| p.exists());
        
        // If no config exists, generate the default one
        if !config_exists {
            if let Err(e) = Self::generate_default_config_file() {
                eprintln!("Warning: Failed to generate default config: {}", e);
            }
        }
        
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
        
        // 1. ~/.config/t-meter/config.toml (Linux/macOS)
        if let Some(proj_dirs) = ProjectDirs::from("", "", "t-meter") {
            paths.push(proj_dirs.config_dir().join("config.toml"));
        }
        
        // 2. ~/Library/Application Support/t-meter/config.toml (macOS alternative)
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join("Library/Application Support/t-meter/config.toml"));
        }
        
        // 3. Current directory ./t-meter.toml (for testing)
        paths.push(PathBuf::from("./t-meter.toml"));
        
        paths
    }
    
    /// Load config from a specific file
    fn load_from_file(path: &PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let config: Config = toml::from_str(&contents)
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
            
            let toml_str = toml::to_string_pretty(self)
                .context("Failed to serialize config")?;
            
            fs::write(path, toml_str)
                .with_context(|| format!("Failed to write config file: {}", path.display()))?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No valid config path found"))
        }
    }
    
    /// Generate a comprehensive default config file with all options documented
    pub fn generate_default_config_file() -> Result<()> {
        let config_paths = Self::get_config_paths();
        
        if let Some(path) = config_paths.first() {
            // Don't overwrite existing config
            if path.exists() {
                return Ok(());
            }
            
            // Create parent directory if it doesn't exist
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
            }
            
            // Create comprehensive config template with TOML comments
            let config_template = r#"# t-meter Configuration File
# Customize your t-meter experience by editing the values below

# =============================================================================
# KEYBOARD SHORTCUTS
# =============================================================================
# Available keyboard shortcuts while running t-meter:
#   q or Ctrl+C  - Quit the application
#   t            - Cycle through available themes
#   q or Ctrl+C  - Quit the application
#   t            - Cycle through available themes
#   d            - Toggle between light and dark mode
#   s            - Cycle through progress bar styles

# =============================================================================
# THEME CONFIGURATION
# =============================================================================

# Theme name - Choose from the following available themes:
#   "default"    - Clean monochrome theme with subtle colors
#   "ocean"      - Calming blue ocean-inspired theme
#   "forest"     - Natural green forest theme
#   "sunset"     - Warm orange and red sunset theme
#   "monochrome" - Pure black and white theme
theme_name = "default"

# Theme mode - Each theme has two modes:
#   "light" - Light background optimized theme
#   "dark"  - Dark background optimized theme
#   "dark"  - Dark background optimized theme
theme_mode = "light"

# =============================================================================
# PROGRESS BAR CONFIGURATION
# =============================================================================

# Style of the progress bar:
#   "Gradient" - Smooth gradient transition (Premium look)
#   "Grainy"   - Retro segmented look
#   "Analog"   - Vertical bars simulating an analog meter
progress_bar_style = "Gradient"

# =============================================================================
# SLEEP TRACKING
# =============================================================================

# Time you wake up (24-hour format HH:MM)
wake_up_time = "07:00"

# Time you go to bed (24-hour format HH:MM)
bed_time = "23:00"

# =============================================================================
# CUSTOMIZATION GUIDE
# =============================================================================
# 1. Edit the 'theme_name' value above to one of the available themes
# 2. Edit the 'theme_mode' value to either 'light' or 'dark'
# 3. Save this file and restart t-meter to see your changes
# 4. Press 't' while running to cycle through themes interactively
# 4. Press 't' while running to cycle through themes interactively
# 5. Press 'd' while running to toggle between light and dark modes
# 6. Press 's' while running to cycle through progress bar styles
"#;
            
            fs::write(path, config_template)
                .with_context(|| format!("Failed to write config file: {}", path.display()))?;
            
            eprintln!("✓ Generated config file at: {}", path.display());
            eprintln!("  You can customize your theme by editing this file.");
            
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
