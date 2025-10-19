//! Slint UI theme integration
//!
//! Bridges between Rust theme system and Slint UI components.
//! Provides theme colors and synchronization between Rust backend
//! and Slint UI frontend.

use crate::theme::{Palette, ThemeManager, ThemeType};
use slint::Color;
use std::sync::Arc;

/// Represents a color suitable for Slint rendering
#[derive(Debug, Clone, Copy)]
pub struct UIColor {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
}

impl UIColor {
    /// Convert to Slint Color
    pub fn to_slint(&self) -> Color {
        Color::from_rgb_u8(self.r, self.g, self.b)
    }

    /// Create from hex string (e.g., "#FFFFFF")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some(UIColor { r, g, b })
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// UI-specific theme palette
#[derive(Debug, Clone)]
pub struct UIThemePalette {
    pub background: UIColor,
    pub text_primary: UIColor,
    pub text_secondary: UIColor,
    pub panel: UIColor,
    pub button: UIColor,
    pub accent: UIColor,
    pub status_green: UIColor,
    pub status_blue: UIColor,
    pub status_red: UIColor,
    pub status_yellow: UIColor,
}

impl UIThemePalette {
    /// Create palette from Theme palette
    pub fn from_palette(palette: &Palette) -> Self {
        UIThemePalette {
            background: UIColor {
                r: palette.background.r,
                g: palette.background.g,
                b: palette.background.b,
            },
            text_primary: UIColor {
                r: palette.text_primary.r,
                g: palette.text_primary.g,
                b: palette.text_primary.b,
            },
            text_secondary: UIColor {
                r: palette.text_secondary.r,
                g: palette.text_secondary.g,
                b: palette.text_secondary.b,
            },
            panel: UIColor {
                r: palette.panel.r,
                g: palette.panel.g,
                b: palette.panel.b,
            },
            button: UIColor {
                r: palette.button.r,
                g: palette.button.g,
                b: palette.button.b,
            },
            accent: UIColor {
                r: palette.accent.r,
                g: palette.accent.g,
                b: palette.accent.b,
            },
            status_green: UIColor {
                r: palette.status_green.r,
                g: palette.status_green.g,
                b: palette.status_green.b,
            },
            status_blue: UIColor {
                r: palette.status_blue.r,
                g: palette.status_blue.g,
                b: palette.status_blue.b,
            },
            status_red: UIColor {
                r: palette.status_red.r,
                g: palette.status_red.g,
                b: palette.status_red.b,
            },
            status_yellow: UIColor {
                r: palette.status_yellow.r,
                g: palette.status_yellow.g,
                b: palette.status_yellow.b,
            },
        }
    }

    /// Get palette for a specific theme type
    pub fn for_theme(theme: ThemeType) -> Self {
        let palette = Palette::for_theme(theme);
        Self::from_palette(&palette)
    }

    /// Create light palette
    pub fn light() -> Self {
        Self::for_theme(ThemeType::Light)
    }

    /// Create dark palette
    pub fn dark() -> Self {
        Self::for_theme(ThemeType::Dark)
    }
}

/// Manages theme integration between Rust and Slint
#[derive(Clone)]
pub struct UIThemeProvider {
    manager: Arc<ThemeManager>,
    current_palette: Arc<tokio::sync::RwLock<UIThemePalette>>,
}

impl UIThemeProvider {
    /// Create a new UI theme provider
    pub async fn new(manager: Arc<ThemeManager>) -> anyhow::Result<Self> {
        let palette = manager.get_palette();
        let ui_palette = UIThemePalette::from_palette(&palette);

        Ok(UIThemeProvider {
            manager,
            current_palette: Arc::new(tokio::sync::RwLock::new(ui_palette)),
        })
    }

    /// Get current UI palette
    pub async fn get_palette(&self) -> UIThemePalette {
        self.current_palette.read().await.clone()
    }

    /// Get current theme type
    pub fn get_theme_type(&self) -> ThemeType {
        self.manager.get_theme()
    }

    /// Get current preference
    pub fn get_preference(&self) -> String {
        self.manager.get_preference()
    }

    /// Set theme and update UI palette
    pub async fn set_theme(&self, theme: ThemeType) -> anyhow::Result<()> {
        self.manager.set_theme(theme).await?;
        let palette = self.manager.get_palette();
        let ui_palette = UIThemePalette::from_palette(&palette);
        *self.current_palette.write().await = ui_palette;
        Ok(())
    }

    /// Set preference and update UI palette
    pub async fn set_preference(&self, preference: &str) -> anyhow::Result<()> {
        self.manager.set_preference(preference).await?;
        let palette = self.manager.get_palette();
        let ui_palette = UIThemePalette::from_palette(&palette);
        *self.current_palette.write().await = ui_palette;
        Ok(())
    }

    /// Toggle between light and dark themes
    pub async fn toggle_theme(&self) -> anyhow::Result<ThemeType> {
        let new_theme = self.manager.toggle_theme().await?;
        let palette = self.manager.get_palette();
        let ui_palette = UIThemePalette::from_palette(&palette);
        *self.current_palette.write().await = ui_palette;
        Ok(new_theme)
    }

    /// Get shared theme colors for Slint callback
    pub async fn get_theme_colors(&self) -> ThemeColors {
        let palette = self.get_palette().await;
        ThemeColors {
            background: palette.background.to_slint(),
            text_primary: palette.text_primary.to_slint(),
            text_secondary: palette.text_secondary.to_slint(),
            panel: palette.panel.to_slint(),
            button: palette.button.to_slint(),
            accent: palette.accent.to_slint(),
            status_green: palette.status_green.to_slint(),
            status_blue: palette.status_blue.to_slint(),
            status_red: palette.status_red.to_slint(),
            status_yellow: palette.status_yellow.to_slint(),
        }
    }
}

/// Theme colors for Slint export
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub background: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub panel: Color,
    pub button: Color,
    pub accent: Color,
    pub status_green: Color,
    pub status_blue: Color,
    pub status_red: Color,
    pub status_yellow: Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_color_creation() {
        let color = UIColor { r: 255, g: 0, b: 0 };
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_ui_color_to_hex() {
        let color = UIColor {
            r: 255,
            g: 107,
            b: 53,
        };
        assert_eq!(color.to_hex(), "#FF6B35");
    }

    #[test]
    fn test_ui_color_from_hex() {
        let color = UIColor::from_hex("#FF6B35").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 107);
        assert_eq!(color.b, 53);
    }

    #[test]
    fn test_ui_color_from_hex_invalid() {
        assert!(UIColor::from_hex("INVALID").is_none());
        assert!(UIColor::from_hex("#GGGGGG").is_none());
    }

    #[test]
    fn test_ui_color_to_slint() {
        let color = UIColor {
            r: 255,
            g: 0,
            b: 0,
        };
        let slint_color = color.to_slint();
        assert_eq!(slint_color.red(), 255);
        assert_eq!(slint_color.green(), 0);
        assert_eq!(slint_color.blue(), 0);
    }

    #[test]
    fn test_ui_theme_palette_light() {
        let palette = UIThemePalette::light();
        assert_eq!(palette.background.r, 255);
        assert_eq!(palette.background.g, 255);
        assert_eq!(palette.background.b, 255);
    }

    #[test]
    fn test_ui_theme_palette_dark() {
        let palette = UIThemePalette::dark();
        assert_eq!(palette.background.r, 30);
        assert_eq!(palette.background.g, 30);
        assert_eq!(palette.background.b, 30);
    }

    #[test]
    fn test_ui_theme_palette_from_palette() {
        let theme_palette = Palette::light();
        let ui_palette = UIThemePalette::from_palette(&theme_palette);
        assert_eq!(ui_palette.background.r, theme_palette.background.r);
        assert_eq!(ui_palette.background.g, theme_palette.background.g);
        assert_eq!(ui_palette.background.b, theme_palette.background.b);
    }

    #[tokio::test]
    async fn test_ui_theme_provider_creation() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await;
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_ui_theme_provider_get_palette() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await.unwrap();
        provider.set_theme(ThemeType::Light).await.unwrap();
        let palette = provider.get_palette().await;
        assert_eq!(palette.background.r, 255);
    }

    #[tokio::test]
    async fn test_ui_theme_provider_set_theme() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await.unwrap();
        provider.set_theme(ThemeType::Dark).await.unwrap();
        let palette = provider.get_palette().await;
        assert_eq!(palette.background.r, 30);
    }

    #[tokio::test]
    async fn test_ui_theme_provider_toggle_theme() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await.unwrap();
        provider.set_theme(ThemeType::Light).await.unwrap();
        let new_theme = provider.toggle_theme().await.unwrap();
        assert_eq!(new_theme, ThemeType::Dark);
        let palette = provider.get_palette().await;
        assert_eq!(palette.background.r, 30);
    }

    #[tokio::test]
    async fn test_ui_theme_provider_set_preference() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await.unwrap();
        provider.set_preference("dark").await.unwrap();
        assert_eq!(provider.get_preference(), "dark");
    }

    #[tokio::test]
    async fn test_ui_theme_provider_get_theme_type() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await.unwrap();
        provider.set_theme(ThemeType::Light).await.unwrap();
        assert_eq!(provider.get_theme_type(), ThemeType::Light);
    }

    #[tokio::test]
    async fn test_ui_theme_provider_get_theme_colors() {
        let manager = Arc::new(ThemeManager::new().await.unwrap());
        let provider = UIThemeProvider::new(manager).await.unwrap();
        provider.set_theme(ThemeType::Light).await.unwrap();
        let colors = provider.get_theme_colors().await;
        assert_eq!(colors.background.red(), 255);
        assert_eq!(colors.background.green(), 255);
        assert_eq!(colors.background.blue(), 255);
    }

    #[test]
    fn test_ui_color_hex_roundtrip() {
        let original = UIColor {
            r: 100,
            g: 150,
            b: 200,
        };
        let hex = original.to_hex();
        let recovered = UIColor::from_hex(&hex).unwrap();
        assert_eq!(original.r, recovered.r);
        assert_eq!(original.g, recovered.g);
        assert_eq!(original.b, recovered.b);
    }
}
