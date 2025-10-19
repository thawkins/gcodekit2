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

use theme::ThemeManager;
use ui_theme::UIThemeProvider;
use anyhow::Result;
use std::sync::Arc;

// Include Slint modules
slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting GCodeKit2 v0.2.0-alpha");

    // Initialize theme system with system theme detection
    let theme_manager = Arc::new(ThemeManager::new().await?);
    let _ui_theme_provider =
        UIThemeProvider::new(Arc::clone(&theme_manager)).await?;

    let current_theme = theme_manager.get_theme();
    tracing::info!(
        "System theme detected: {:?}",
        current_theme
    );

    // Create UI
    let ui = AppWindow::new()?;

    // Log successful initialization
    tracing::info!("UI window created successfully");

    ui.run()?;

    Ok(())
}
