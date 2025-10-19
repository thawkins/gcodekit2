//! GCodeKit2 - Desktop application for GRBL laser engravers and CNC machines
//!
//! Provides comprehensive machine control, CAM functions, and error recovery
//! for GRBL v1.1+ compatible devices across Linux, Windows, and macOS.

mod communication;
mod designer;
mod jobs;
mod materials;
mod theme;
mod ui_theme;
mod widgets;

use theme::{ThemeManager, ThemeType};
use ui_theme::UIThemeProvider;
use anyhow::Result;

// Include Slint modules
slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting GCodeKit2 v0.1.0-alpha");

    // Initialize theme system with system theme detection
    let theme_manager = ThemeManager::new().await?;
    let ui_theme_provider = UIThemeProvider::new(
        std::sync::Arc::new(theme_manager),
    )
    .await?;

    tracing::info!(
        "System theme detected: {:?}",
        ui_theme_provider.get_theme_type()
    );

    // Create UI
    let ui = AppWindow::new()?;

    // Log successful initialization
    tracing::info!("UI window created successfully");

    ui.run()?;

    Ok(())
}
