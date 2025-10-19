//! GCodeKit2 - Desktop application for GRBL laser engravers and CNC machines
//!
//! Provides comprehensive machine control, CAM functions, and error recovery
//! for GRBL v1.1+ compatible devices across Linux, Windows, and macOS.

mod communication;
mod designer;
mod jobs;
mod materials;
mod widgets;

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

    let ui = AppWindow::new()?;
    ui.run()?;

    Ok(())
}
