//! GCodeKit - Desktop GUI for GRBL laser engravers and CNC machines
//!
//! This application provides comprehensive machine control, CAM functions,
//! and error recovery for GRBL-compatible devices.

mod communication;
mod designer;
mod jobs;
mod materials;
mod widgets;

use anyhow::Result;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting GCodeKit application");

    let ui = AppWindow::new()?;
    ui.run()?;

    Ok(())
}
