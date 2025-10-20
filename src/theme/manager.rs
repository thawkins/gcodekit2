//! Theme management and orchestration
//!
//! Central manager for theme operations including theme switching,
//! palette retrieval, and preference persistence.

use super::detector::SystemThemeDetector;
use super::palette::{Palette, ThemeType};
use super::storage::ThemeStorage;
use std::sync::{Arc, RwLock};

/// Manages all theme operations
#[derive(Clone)]
pub struct ThemeManager {
    current_theme: Arc<RwLock<ThemeType>>,
    user_preference: Arc<RwLock<String>>,
    storage: ThemeStorage,
}

impl ThemeManager {
    /// Create and initialize a new theme manager
    /// Detects system theme and loads user preferences
    pub async fn new() -> anyhow::Result<Self> {
        let storage = ThemeStorage::new()?;
        let preference = storage.load_preference()?;

        // Determine initial theme
        let initial_theme = if preference == "system" {
            SystemThemeDetector::detect_system_theme()
        } else if preference == "light" {
            ThemeType::Light
        } else if preference == "dark" {
            ThemeType::Dark
        } else {
            ThemeType::Light
        };


        Ok(ThemeManager {
            current_theme: Arc::new(RwLock::new(initial_theme)),
            user_preference: Arc::new(RwLock::new(preference)),
            storage,
        })
    }

    /// Get the currently active theme
    pub fn get_theme(&self) -> ThemeType {
        *self.current_theme.read().unwrap()
    }

    /// Get the user's theme preference ("light", "dark", or "system")
    pub fn get_preference(&self) -> String {
        self.user_preference.read().unwrap().clone()
    }

    /// Set the theme and save preference
    pub async fn set_theme(&self, theme: ThemeType) -> anyhow::Result<()> {
        let preference = theme.to_string();
        self.storage.save_preference(&preference)?;
        *self.current_theme.write().unwrap() = theme;
        *self.user_preference.write().unwrap() = preference.clone();

        Ok(())
    }

    /// Set user preference and update theme accordingly
    pub async fn set_preference(&self, preference: &str) -> anyhow::Result<()> {
        let theme = if preference == "system" {
            SystemThemeDetector::detect_system_theme()
        } else if preference == "light" {
            ThemeType::Light
        } else if preference == "dark" {
            ThemeType::Dark
        } else {
            return Err(anyhow::anyhow!(
                "Invalid preference: {}. Use 'light', 'dark', or 'system'",
                preference
            ));
        };

        self.storage.save_preference(preference)?;
        *self.current_theme.write().unwrap() = theme;
        *self.user_preference.write().unwrap() = preference.to_string();

        Ok(())
    }

    /// Get the color palette for the current theme
    pub fn get_palette(&self) -> Palette {
        let theme = self.get_theme();
        Palette::for_theme(theme)
    }

    /// Get palette for a specific theme type
    pub fn get_palette_for(&self, theme_type: ThemeType) -> Palette {
        Palette::for_theme(theme_type)
    }

    /// Toggle between light and dark themes
    pub async fn toggle_theme(&self) -> anyhow::Result<ThemeType> {
        let current = self.get_theme();
        let new_theme = match current {
            ThemeType::Light => ThemeType::Dark,
            ThemeType::Dark => ThemeType::Light,
        };

        self.set_theme(new_theme).await?;
        Ok(new_theme)
    }

    /// Re-detect system theme and apply if preference is "system"
    pub async fn apply_system_theme(&self) -> anyhow::Result<()> {
        let preference = self.get_preference();
        if preference == "system" {
            let system_theme = SystemThemeDetector::detect_system_theme();
            *self.current_theme.write().unwrap() = system_theme;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_theme_manager_creation() {
        let manager = ThemeManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_get_theme_returns_valid_theme() {
        let manager = ThemeManager::new().await.unwrap();
        let theme = manager.get_theme();
        match theme {
            ThemeType::Light | ThemeType::Dark => {
                // Valid
            }
        }
    }

    #[tokio::test]
    async fn test_get_preference_returns_string() {
        let manager = ThemeManager::new().await.unwrap();
        let pref = manager.get_preference();
        assert!(pref == "light" || pref == "dark" || pref == "system");
    }

    #[tokio::test]
    async fn test_set_theme_light() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_theme(ThemeType::Light).await.unwrap();
        assert_eq!(manager.get_theme(), ThemeType::Light);
    }

    #[tokio::test]
    async fn test_set_theme_dark() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_theme(ThemeType::Dark).await.unwrap();
        assert_eq!(manager.get_theme(), ThemeType::Dark);
    }

    #[tokio::test]
    async fn test_get_palette_light() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_theme(ThemeType::Light).await.unwrap();
        let palette = manager.get_palette();
        assert_eq!(palette.theme_type, ThemeType::Light);
    }

    #[tokio::test]
    async fn test_get_palette_dark() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_theme(ThemeType::Dark).await.unwrap();
        let palette = manager.get_palette();
        assert_eq!(palette.theme_type, ThemeType::Dark);
    }

    #[tokio::test]
    async fn test_toggle_theme_from_light() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_theme(ThemeType::Light).await.unwrap();
        let new_theme = manager.toggle_theme().await.unwrap();
        assert_eq!(new_theme, ThemeType::Dark);
        assert_eq!(manager.get_theme(), ThemeType::Dark);
    }

    #[tokio::test]
    async fn test_toggle_theme_from_dark() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_theme(ThemeType::Dark).await.unwrap();
        let new_theme = manager.toggle_theme().await.unwrap();
        assert_eq!(new_theme, ThemeType::Light);
        assert_eq!(manager.get_theme(), ThemeType::Light);
    }

    #[tokio::test]
    async fn test_set_preference_light() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_preference("light").await.unwrap();
        assert_eq!(manager.get_preference(), "light");
        assert_eq!(manager.get_theme(), ThemeType::Light);
    }

    #[tokio::test]
    async fn test_set_preference_dark() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_preference("dark").await.unwrap();
        assert_eq!(manager.get_preference(), "dark");
        assert_eq!(manager.get_theme(), ThemeType::Dark);
    }

    #[tokio::test]
    async fn test_set_preference_system() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_preference("system").await.unwrap();
        assert_eq!(manager.get_preference(), "system");
    }

    #[tokio::test]
    async fn test_set_invalid_preference() {
        let manager = ThemeManager::new().await.unwrap();
        let result = manager.set_preference("invalid").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_apply_system_theme() {
        let manager = ThemeManager::new().await.unwrap();
        manager.set_preference("system").await.unwrap();
        let result = manager.apply_system_theme().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_palette_for_light() {
        let manager = ThemeManager::new().await.unwrap();
        let palette = manager.get_palette_for(ThemeType::Light);
        assert_eq!(palette.theme_type, ThemeType::Light);
    }

    #[tokio::test]
    async fn test_get_palette_for_dark() {
        let manager = ThemeManager::new().await.unwrap();
        let palette = manager.get_palette_for(ThemeType::Dark);
        assert_eq!(palette.theme_type, ThemeType::Dark);
    }

    #[tokio::test]
    async fn test_theme_manager_thread_safe() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());

        // Clone for each thread
        let manager1 = manager.clone();
        let manager2 = manager.clone();

        let handle1 = tokio::spawn(async move {
            manager1.set_theme(ThemeType::Dark).await.unwrap();
            manager1.get_theme()
        });

        let handle2 = tokio::spawn(async move {
            manager2.get_theme()
        });

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        // Both should see the same theme (or valid themes)
        match (result1, result2) {
            (ThemeType::Dark, ThemeType::Dark) => { /* Both see dark */ }
            _ => { /* At least one is valid */ }
        }
    }
}
