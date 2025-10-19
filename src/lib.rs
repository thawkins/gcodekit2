//! GCodeKit - Desktop application for GRBL laser engravers and CNC machines
//!
//! Provides comprehensive machine control, CAM functions, and error recovery
//! for GRBL v1.1+ compatible devices across Linux, Windows, and macOS.

pub mod communication;
pub mod designer;
pub mod jobs;
pub mod materials;
pub mod widgets;
pub mod theme;
pub mod ui_theme;
pub mod pendant;

pub use communication::GrblController;
pub use designer::Designer;
pub use jobs::JobManager;
pub use materials::MaterialDatabase;
pub use theme::ThemeManager;
pub use ui_theme::{UIThemeProvider, UIThemePalette, UIColor};
pub use pendant::{PendantServer, PendantConfig};
