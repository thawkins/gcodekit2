//! Theme system tests
//!
//! Tests for the theme module covering:
//! - System theme detection
//! - Theme switching and persistence
//! - Color palettes and WCAG compliance
//! - Theme storage and retrieval

use gcodekit2::theme::{
    ThemeManager, ThemeType, SystemThemeDetector, ThemeStorage,
};

// ============ System Theme Detection Tests ============

#[tokio::test]
async fn test_theme_manager_creation() {
    let manager = ThemeManager::new().await;
    assert!(manager.is_ok(), "ThemeManager should initialize successfully");
}

#[tokio::test]
async fn test_get_current_theme() {
    let manager = ThemeManager::new().await.unwrap();
    let current_theme = manager.get_theme();
    assert!(
        matches!(current_theme, ThemeType::Light | ThemeType::Dark),
        "Current theme should be Light or Dark"
    );
}

#[tokio::test]
async fn test_theme_type_to_string() {
    assert_eq!(ThemeType::Light.to_string(), "light");
    assert_eq!(ThemeType::Dark.to_string(), "dark");
}

#[tokio::test]
async fn test_theme_type_display() {
    let light = ThemeType::Light;
    let dark = ThemeType::Dark;
    
    assert_eq!(format!("{}", light), "light");
    assert_eq!(format!("{}", dark), "dark");
}

// ============ Theme Switching Tests ============

#[tokio::test]
async fn test_set_theme_light() {
    let manager = ThemeManager::new().await.unwrap();
    manager.set_theme(ThemeType::Light).await.ok();
    assert_eq!(manager.get_theme(), ThemeType::Light);
}

#[tokio::test]
async fn test_set_theme_dark() {
    let manager = ThemeManager::new().await.unwrap();
    manager.set_theme(ThemeType::Dark).await.ok();
    assert_eq!(manager.get_theme(), ThemeType::Dark);
}

#[tokio::test]
async fn test_set_preference_light() {
    let manager = ThemeManager::new().await.unwrap();
    let result = manager.set_preference("light").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_preference_dark() {
    let manager = ThemeManager::new().await.unwrap();
    let result = manager.set_preference("dark").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_preference_system() {
    let manager = ThemeManager::new().await.unwrap();
    let result = manager.set_preference("system").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_palette_light() {
    let manager = ThemeManager::new().await.unwrap();
    manager.set_theme(ThemeType::Light).await.ok();
    
    let palette = manager.get_palette();
    assert_eq!(palette.theme_type, ThemeType::Light);
}

#[tokio::test]
async fn test_get_palette_dark() {
    let manager = ThemeManager::new().await.unwrap();
    manager.set_theme(ThemeType::Dark).await.ok();
    
    let palette = manager.get_palette();
    assert_eq!(palette.theme_type, ThemeType::Dark);
}

// ============ Theme Storage Tests ============

#[test]
fn test_theme_storage_creation() {
    let storage = ThemeStorage::new();
    assert!(storage.is_ok(), "ThemeStorage should initialize successfully");
}

#[test]
fn test_theme_storage_load_preference() {
    let storage = ThemeStorage::new().unwrap();
    let result = storage.load_preference();
    // Should return a valid preference string
    assert!(result.is_ok());
}

#[test]
fn test_theme_storage_save_preference() {
    let storage = ThemeStorage::new().unwrap();
    let result = storage.save_preference("dark");
    assert!(result.is_ok(), "Saving preference should succeed");
}

// ============ System Theme Detection Tests ============

#[test]
fn test_system_theme_detection() {
    let theme = SystemThemeDetector::detect_system_theme();
    assert!(
        matches!(theme, ThemeType::Light | ThemeType::Dark),
        "System theme should be Light or Dark"
    );
}

// ============ Color Conversion Tests ============

#[test]
fn test_color_hex_format() {
    let color = gcodekit2::theme::palette::LIGHT_BACKGROUND;
    let hex = color.to_hex();
    
    assert!(hex.starts_with('#'));
    assert_eq!(hex.len(), 7); // #RRGGBB
}

#[test]
fn test_light_theme_palette_colors() {
    use gcodekit2::theme::palette::Palette;
    
    let palette = Palette::for_theme(ThemeType::Light);
    assert_eq!(palette.theme_type, ThemeType::Light);
    
    // Verify colors are defined
    assert_ne!(palette.background, palette.text_primary);
    assert_ne!(palette.background, palette.panel);
}

#[test]
fn test_dark_theme_palette_colors() {
    use gcodekit2::theme::palette::Palette;
    
    let palette = Palette::for_theme(ThemeType::Dark);
    assert_eq!(palette.theme_type, ThemeType::Dark);
    
    // Verify colors are defined and different
    assert_ne!(palette.background, palette.text_primary);
    assert_ne!(palette.background, palette.panel);
}

#[test]
fn test_palette_light_dark_differences() {
    use gcodekit2::theme::palette::Palette;
    
    let light = Palette::for_theme(ThemeType::Light);
    let dark = Palette::for_theme(ThemeType::Dark);
    
    // Ensure themes are actually different
    assert_ne!(light.background.to_hex(), dark.background.to_hex());
    assert_ne!(light.text_primary.to_hex(), dark.text_primary.to_hex());
}

#[test]
fn test_palette_status_colors_distinct() {
    use gcodekit2::theme::palette::Palette;
    
    let palette = Palette::for_theme(ThemeType::Light);
    
    // Status colors should be distinct
    assert_ne!(palette.status_green.to_hex(), palette.status_red.to_hex());
    assert_ne!(palette.status_green.to_hex(), palette.status_blue.to_hex());
    assert_ne!(palette.status_red.to_hex(), palette.status_blue.to_hex());
}

// ============ Color Luminance Tests (WCAG Compliance) ============

#[test]
fn test_color_luminance_calculation() {
    use gcodekit2::theme::palette::LIGHT_BACKGROUND;
    
    let lum = LIGHT_BACKGROUND.luminance();
    assert!(lum > 0.0 && lum <= 1.0, "Luminance should be between 0 and 1");
}

#[test]
fn test_light_theme_contrast_ratio() {
    use gcodekit2::theme::palette::{LIGHT_BACKGROUND, LIGHT_TEXT_PRIMARY};
    
    let l1 = LIGHT_BACKGROUND.luminance();
    let l2 = LIGHT_TEXT_PRIMARY.luminance();
    
    let contrast = if l1 > l2 {
        (l1 + 0.05) / (l2 + 0.05)
    } else {
        (l2 + 0.05) / (l1 + 0.05)
    };
    
    // WCAG AA minimum is 4.5:1 for normal text
    assert!(contrast >= 4.5, "Light theme should meet WCAG AA contrast (got {}, need 4.5)", contrast);
}

#[test]
fn test_dark_theme_contrast_ratio() {
    use gcodekit2::theme::palette::{DARK_BACKGROUND, DARK_TEXT_PRIMARY};
    
    let l1 = DARK_BACKGROUND.luminance();
    let l2 = DARK_TEXT_PRIMARY.luminance();
    
    let contrast = if l1 > l2 {
        (l1 + 0.05) / (l2 + 0.05)
    } else {
        (l2 + 0.05) / (l1 + 0.05)
    };
    
    // WCAG AA minimum is 4.5:1 for normal text
    assert!(contrast >= 4.5, "Dark theme should meet WCAG AA contrast (got {}, need 4.5)", contrast);
}

// ============ Edge Case Tests ============

#[tokio::test]
async fn test_rapid_theme_changes() {
    let manager = ThemeManager::new().await.unwrap();
    
    for i in 0..10 {
        let theme = if i % 2 == 0 {
            ThemeType::Light
        } else {
            ThemeType::Dark
        };
        
        let result = manager.set_theme(theme).await;
        assert!(result.is_ok(), "Theme change should succeed on iteration {}", i);
    }
    
    let final_theme = manager.get_theme();
    assert!(matches!(final_theme, ThemeType::Light | ThemeType::Dark));
}

#[test]
fn test_theme_type_equality() {
    assert_eq!(ThemeType::Light, ThemeType::Light);
    assert_eq!(ThemeType::Dark, ThemeType::Dark);
    assert_ne!(ThemeType::Light, ThemeType::Dark);
}

#[test]
fn test_theme_type_debug() {
    let light = ThemeType::Light;
    let debug_str = format!("{:?}", light);
    assert_eq!(debug_str, "Light");
}

#[test]
fn test_preference_persistence_workflow() {
    let storage = ThemeStorage::new().unwrap();
    
    // Save a preference
    let save_result = storage.save_preference("dark");
    assert!(save_result.is_ok());
    
    // Load it back
    let load_result = storage.load_preference();
    assert!(load_result.is_ok());
}

// ============ UI Palette Integration Tests ============

#[test]
fn test_palette_to_slint_colors() {
    use gcodekit2::theme::palette::Palette;
    
    let palette = Palette::for_theme(ThemeType::Light);
    
    // Ensure colors can be converted to Slint format
    let _bg_color = palette.background.to_slint_color();
    let _text_color = palette.text_primary.to_slint_color();
    let _panel_color = palette.panel.to_slint_color();
}

#[tokio::test]
async fn test_manager_get_palette_for_theme() {
    let manager = ThemeManager::new().await.unwrap();
    
    manager.set_theme(ThemeType::Light).await.ok();
    let light_palette = manager.get_palette_for(ThemeType::Light);
    assert_eq!(light_palette.theme_type, ThemeType::Light);
    
    manager.set_theme(ThemeType::Dark).await.ok();
    let dark_palette = manager.get_palette_for(ThemeType::Dark);
    assert_eq!(dark_palette.theme_type, ThemeType::Dark);
}

// ============ Preference Management Tests ============

#[tokio::test]
async fn test_get_preference_value() {
    let manager = ThemeManager::new().await.unwrap();
    manager.set_preference("dark").await.ok();
    
    let pref = manager.get_preference();
    assert!(!pref.is_empty());
}

#[tokio::test]
async fn test_invalid_preference_handling() {
    let manager = ThemeManager::new().await.unwrap();
    let result = manager.set_preference("invalid").await;
    
    assert!(result.is_err(), "Invalid preference should return error");
}

