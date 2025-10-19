//! Theme preference persistence to configuration files
//!
//! Stores and retrieves user theme preferences across application sessions.
//! Uses JSON format for easy configuration and platform-specific config directories.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Theme preference configuration stored in JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// User's theme preference: "light", "dark", or "system"
    pub theme: String,
    /// Whether to auto-follow system theme changes
    pub auto_follow_system: bool,
    /// Last update timestamp (ISO 8601)
    pub last_updated: String,
    /// Reserved for future custom color overrides
    #[serde(default)]
    pub custom_overrides: serde_json::Value,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            theme: "system".to_string(),
            auto_follow_system: true,
            last_updated: chrono::Utc::now().to_rfc3339(),
            custom_overrides: serde_json::json!({}),
        }
    }
}

/// Manages theme preference storage and retrieval
/// Persistent theme preference storage
#[derive(Clone)]
pub struct ThemeStorage {
    config_path: PathBuf,
}

impl ThemeStorage {
    /// Create a new theme storage instance
    /// Creates config directory if it doesn't exist
    pub fn new() -> anyhow::Result<Self> {
        let config_path = Self::get_config_path()?;

        // Create config directory if needed
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(ThemeStorage { config_path })
    }

    /// Get platform-specific config directory path
    fn get_config_path() -> anyhow::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")?;
            Ok(PathBuf::from(appdata)
                .join("gcodekit2")
                .join("theme.json"))
        }

        #[cfg(target_os = "macos")]
        {
            let home = dirs::home_dir().ok_or_else(|| {
                anyhow::anyhow!("Cannot determine home directory")
            })?;
            Ok(home
                .join("Library/Application Support/gcodekit2")
                .join("theme.json"))
        }

        #[cfg(target_os = "linux")]
        {
            let config_dir = dirs::config_dir().ok_or_else(|| {
                anyhow::anyhow!("Cannot determine config directory")
            })?;
            Ok(config_dir.join("gcodekit2").join("theme.json"))
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(anyhow::anyhow!("Unsupported platform"))
        }
    }

    /// Save theme preference to disk
    pub fn save_preference(&self, preference: &str) -> anyhow::Result<()> {
        let config = ThemeConfig {
            theme: preference.to_string(),
            auto_follow_system: preference == "system",
            last_updated: chrono::Utc::now().to_rfc3339(),
            custom_overrides: serde_json::json!({}),
        };

        let json = serde_json::to_string_pretty(&config)?;
        std::fs::write(&self.config_path, json)?;

        tracing::debug!("Theme preference saved: {} to {}", preference, self.config_path.display());
        Ok(())
    }

    /// Load theme preference from disk
    pub fn load_preference(&self) -> anyhow::Result<String> {
        if !self.config_path.exists() {
            tracing::debug!("No theme config found, using default: system");
            return Ok("system".to_string());
        }

        let json = std::fs::read_to_string(&self.config_path)?;
        if json.trim().is_empty() {
            tracing::debug!("Theme config is empty, using default: system");
            return Ok("system".to_string());
        }

        match serde_json::from_str::<ThemeConfig>(&json) {
            Ok(config) => {
                tracing::debug!("Theme preference loaded: {}", config.theme);
                Ok(config.theme)
            }
            Err(_) => {
                tracing::debug!("Failed to parse theme config, using default: system");
                Ok("system".to_string())
            }
        }
    }

    /// Reset preferences to defaults
    pub fn reset_to_default(&self) -> anyhow::Result<()> {
        let default_config = ThemeConfig::default();
        let json = serde_json::to_string_pretty(&default_config)?;
        std::fs::write(&self.config_path, json)?;

        tracing::info!("Theme preferences reset to defaults");
        Ok(())
    }

    /// Load full configuration
    pub fn load_config(&self) -> anyhow::Result<ThemeConfig> {
        if !self.config_path.exists() {
            return Ok(ThemeConfig::default());
        }

        let json = std::fs::read_to_string(&self.config_path)?;
        let config: ThemeConfig = serde_json::from_str(&json)?;
        Ok(config)
    }

    /// Get config file path (for testing/debugging)
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

impl Default for ThemeStorage {
    fn default() -> Self {
        // In tests or default initialization, use a temp path
        Self {
            config_path: PathBuf::from(".gcodekit2_theme_default.json"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_storage_creation() {
        let _storage = ThemeStorage::default();
    }

    #[test]
    fn test_default_config() {
        let config = ThemeConfig::default();
        assert_eq!(config.theme, "system");
        assert!(config.auto_follow_system);
    }

    #[test]
    fn test_theme_storage_new() {
        // This will use actual directories - just verify it doesn't panic
        let _result = ThemeStorage::new();
    }

    #[test]
    fn test_save_and_load_preference() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let config_file = temp_dir.path().join("theme.json");

        let mut storage = ThemeStorage::default();
        storage.config_path = config_file.clone();

        // Save preference
        storage.save_preference("dark")?;

        // Load and verify
        let preference = storage.load_preference()?;
        assert_eq!(preference, "dark");

        Ok(())
    }

    #[test]
    fn test_save_multiple_preferences() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let config_file = temp_dir.path().join("theme.json");

        let mut storage = ThemeStorage::default();
        storage.config_path = config_file;

        // Save and verify multiple times
        storage.save_preference("light")?;
        assert_eq!(storage.load_preference()?, "light");

        storage.save_preference("dark")?;
        assert_eq!(storage.load_preference()?, "dark");

        storage.save_preference("system")?;
        assert_eq!(storage.load_preference()?, "system");

        Ok(())
    }

    #[test]
    fn test_reset_to_default() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let config_file = temp_dir.path().join("theme.json");

        let mut storage = ThemeStorage::default();
        storage.config_path = config_file;

        // Save custom preference
        storage.save_preference("dark")?;
        assert_eq!(storage.load_preference()?, "dark");

        // Reset to default
        storage.reset_to_default()?;
        let config = storage.load_config()?;
        assert_eq!(config.theme, "system");
        assert!(config.auto_follow_system);

        Ok(())
    }

    #[test]
    fn test_config_serialization() {
        let config = ThemeConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();

        assert!(json.contains("\"theme\""));
        assert!(json.contains("\"system\""));
        assert!(json.contains("\"auto_follow_system\""));
        assert!(json.contains("\"last_updated\""));
    }

    #[test]
    fn test_config_deserialization() -> anyhow::Result<()> {
        let json = r#"{
  "theme": "dark",
  "auto_follow_system": false,
  "last_updated": "2025-10-19T09:00:00+00:00",
  "custom_overrides": {}
}"#;

        let config: ThemeConfig = serde_json::from_str(json)?;
        assert_eq!(config.theme, "dark");
        assert!(!config.auto_follow_system);

        Ok(())
    }

    #[test]
    fn test_nonexistent_config_returns_default() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let config_file = temp_dir.path().join("nonexistent").join("theme.json");

        let mut storage = ThemeStorage::default();
        storage.config_path = config_file;

        let preference = storage.load_preference()?;
        assert_eq!(preference, "system");

        Ok(())
    }
}
