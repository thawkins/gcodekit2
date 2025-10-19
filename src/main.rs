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
use widgets::ConnectionWidget;
use communication::GrblController;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    // Setup event handlers for UI interaction
    setup_ui_handlers(&ui).await?;

    ui.run()?;

    Ok(())
}

/// Setup all UI event handlers and callbacks
async fn setup_ui_handlers(ui: &AppWindow) -> Result<()> {
    tracing::debug!("Setting up UI event handlers");

    // Initialize connection widget and controller
    let connection_widget = Arc::new(Mutex::new(ConnectionWidget::new()));
    let grbl_controller = Arc::new(GrblController::new());

    // Set up connection widget UI bindings
    {
        let connection_widget = Arc::clone(&connection_widget);
        let grbl_controller = Arc::clone(&grbl_controller);

        // Refresh ports callback
        ui.on_refresh_ports({
            let connection_widget = Arc::clone(&connection_widget);
            move || {
                let connection_widget = connection_widget.clone();
                let _ = slint::spawn_local(async move {
                    let mut widget = connection_widget.lock().await;
                    if let Err(e) = widget.refresh_ports() {
                        tracing::error!("Failed to refresh ports: {}", e);
                    }
                });
            }
        });

        // Connect callback
        ui.on_connect({
            let connection_widget = Arc::clone(&connection_widget);
            let grbl_controller = Arc::clone(&grbl_controller);
            move |port: slint::SharedString| {
                let connection_widget = connection_widget.clone();
                let grbl_controller = grbl_controller.clone();
                let _ = slint::spawn_local(async move {
                    let mut widget = connection_widget.lock().await;
                    match widget
                        .connect(&grbl_controller, port.to_string())
                        .await
                    {
                        Ok(_) => {
                            tracing::info!("Connected to device");
                        }
                        Err(e) => {
                            tracing::error!("Connection failed: {}", e);
                        }
                    }
                });
            }
        });

        // Disconnect callback
        ui.on_disconnect({
            let connection_widget = Arc::clone(&connection_widget);
            let grbl_controller = Arc::clone(&grbl_controller);
            move || {
                let connection_widget = connection_widget.clone();
                let grbl_controller = grbl_controller.clone();
                let _ = slint::spawn_local(async move {
                    let mut widget = connection_widget.lock().await;
                    match widget.disconnect(&grbl_controller).await {
                        Ok(_) => {
                            tracing::info!("Disconnected from device");
                        }
                        Err(e) => {
                            tracing::error!("Disconnection failed: {}", e);
                        }
                    }
                });
            }
        });
    }

    // Initial port refresh
    {
        let connection_widget = Arc::clone(&connection_widget);
        let mut widget = connection_widget.lock().await;
        if let Err(e) = widget.refresh_ports() {
            tracing::warn!("Initial port refresh failed: {}", e);
        }
    }

    tracing::debug!("UI event handlers setup complete");
    Ok(())
}

