//! Theme management system for light/dark mode support
//!
//! Provides theme detection, palette management, and preference persistence
//! for dynamic UI adaptation to system theme preferences.
//!
//! # Features
//! - Automatic OS theme detection (Windows, macOS, Linux)
//! - Light and Dark theme color palettes
//! - Real-time theme switching
//! - User preference persistence
//! - WCAG AA accessibility compliance
//!
//! # Example
//! ```ignore
//! use gcodekit2::theme::{ThemeManager, ThemeType};
//!
//! let manager = ThemeManager::new().await?;
//! let palette = manager.get_palette();
//! manager.set_theme(ThemeType::Dark).await?;
//! ```

pub mod palette;
pub mod manager;
pub mod detector;
pub mod storage;

pub use manager::ThemeManager;
pub use palette::{Palette, ThemeType};
pub use detector::SystemThemeDetector;
pub use storage::ThemeStorage;
