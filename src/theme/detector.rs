//! System theme detection for light/dark mode preference
//!
//! Detects the operating system's theme preference on Windows, macOS, and Linux.
//! Falls back to Light theme if detection fails.

use super::palette::ThemeType;

/// Detects and monitors system theme preference
#[derive(Debug, Clone)]
pub struct SystemThemeDetector;

impl SystemThemeDetector {
    /// Create a new system theme detector
    pub fn new() -> Self {
        SystemThemeDetector
    }

    /// Detect the current system theme preference
    pub fn detect_system_theme() -> ThemeType {
        #[cfg(target_os = "windows")]
        {
            Self::detect_windows_theme().unwrap_or(ThemeType::Light)
        }

        #[cfg(target_os = "macos")]
        {
            Self::detect_macos_theme().unwrap_or(ThemeType::Light)
        }

        #[cfg(target_os = "linux")]
        {
            Self::detect_linux_theme().unwrap_or(ThemeType::Light)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            ThemeType::Light
        }
    }

    /// Detect Windows theme from registry
    #[cfg(target_os = "windows")]
    fn detect_windows_theme() -> Option<ThemeType> {
        use winreg::RegKey;
        use winreg::enums::HKEY_CURRENT_USER;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";

        if let Ok(key) = hkcu.open_subkey(path) {
            if let Ok(value) = key.get_value::<u32, &str>("AppsUseLightTheme") {
                return Some(if value == 0 { ThemeType::Dark } else { ThemeType::Light });
            }
        }

        None
    }

    /// Detect Windows theme (stub for non-Windows)
    #[cfg(not(target_os = "windows"))]
    fn detect_windows_theme() -> Option<ThemeType> {
        None
    }

    /// Detect macOS theme from system preferences
    #[cfg(target_os = "macos")]
    fn detect_macos_theme() -> Option<ThemeType> {
        use std::process::Command;

        let output = Command::new("defaults")
            .args(&["read", "-g", "AppleInterfaceStyle"])
            .output()
            .ok()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        // If "Dark" is in the output, it's dark mode
        if stdout.contains("Dark") {
            Some(ThemeType::Dark)
        } else {
            Some(ThemeType::Light)
        }
    }

    /// Detect macOS theme (stub for non-macOS)
    #[cfg(not(target_os = "macos"))]
    fn detect_macos_theme() -> Option<ThemeType> {
        None
    }

    /// Detect Linux theme from GTK settings
    #[cfg(target_os = "linux")]
    fn detect_linux_theme() -> Option<ThemeType> {
        use std::path::Path;

        // Try common GTK config locations
        let config_paths = vec![
            dirs::config_dir().map(|p| p.join("gtk-3.0/settings.ini")),
            dirs::home_dir().map(|p| p.join(".config/gtk-3.0/settings.ini")),
        ];

        for path_opt in config_paths {
            if let Some(path) = path_opt {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if content.contains("gtk-application-prefer-dark-theme=true") {
                        return Some(ThemeType::Dark);
                    } else if content.contains("gtk-application-prefer-dark-theme=false") {
                        return Some(ThemeType::Light);
                    }
                }
            }
        }

        None
    }

    /// Detect Linux theme (stub for non-Linux)
    #[cfg(not(target_os = "linux"))]
    fn detect_linux_theme() -> Option<ThemeType> {
        None
    }
}

impl Default for SystemThemeDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = SystemThemeDetector::new();
        assert_eq!(std::mem::size_of_val(&detector), 0);
    }

    #[test]
    fn test_detector_default() {
        let _detector = SystemThemeDetector::default();
    }

    #[test]
    fn test_system_theme_detection_returns_theme() {
        let theme = SystemThemeDetector::detect_system_theme();
        // Should always return a theme (Light is default fallback)
        match theme {
            ThemeType::Light | ThemeType::Dark => {
                // Valid theme detected
            }
        }
    }

    #[test]
    fn test_multiple_detections_consistent() {
        let theme1 = SystemThemeDetector::detect_system_theme();
        let theme2 = SystemThemeDetector::detect_system_theme();
        // Should be consistent within same session
        assert_eq!(theme1, theme2);
    }
}
