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
mod console_logger;

use theme::ThemeManager;
use ui_theme::UIThemeProvider;
use widgets::ConnectionWidget;
use communication::GrblController;
use console_logger::{init_console_logging, get_console_logs, add_console_message};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

// Include Slint modules
slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize console logging (captures tracing output from gcodekit2 only, not dependencies)
    let console_buffer = init_console_logging();

    // Create UI
    let ui = AppWindow::new()?;

    // Setup event handlers for UI interaction
    setup_ui_handlers(&ui, console_buffer).await?;

    ui.run()?;

    Ok(())
}

/// Helper function to update console display with filtered and formatted logs
fn update_console_display(
    ui: &AppWindow,
    console_buffer: &console_logger::ConsoleBuffer,
) {
    let filtered = console_logger::filter_console_logs(
        console_buffer,
        ui.get_show_info(),
        ui.get_show_debug(),
        ui.get_show_warn(),
        ui.get_show_error(),
        ui.get_show_trace(),
        ui.get_show_other(),
    );
    // Format logs with level at the start
    let formatted: Vec<String> = filtered
        .iter()
        .map(|line| console_logger::format_log_line(line))
        .collect();
    ui.set_console_content(formatted.join("\n").into());
}

/// Setup all UI event handlers and callbacks
async fn setup_ui_handlers(ui: &AppWindow, console_buffer: console_logger::ConsoleBuffer) -> Result<()> {
    tracing::debug!("Setting up UI event handlers");

    // Initialize connection widget and controller
    let connection_widget = Arc::new(Mutex::new(ConnectionWidget::new()));
    let grbl_controller = Arc::new(GrblController::new());

    // Initial port refresh
    {
        let connection_widget = Arc::clone(&connection_widget);
        let mut widget = connection_widget.lock().await;
        if let Err(e) = widget.refresh_ports() {
            tracing::warn!("Initial port refresh failed: {}", e);
        }
        
        // Update UI with initial port list
        let available_ports = widget.available_ports.iter()
            .map(|p| slint::SharedString::from(p.clone()))
            .collect::<Vec<_>>();
        ui.set_available_ports(slint::ModelRc::new(slint::VecModel::from(available_ports)));
    }

    // Set up connection widget UI bindings
    {
        let connection_widget = Arc::clone(&connection_widget);
        let grbl_controller = Arc::clone(&grbl_controller);
        let ui_handle = ui.as_weak();

        // Refresh ports callback
        ui.on_refresh_ports({
            let connection_widget = Arc::clone(&connection_widget);
            let ui_handle = ui_handle.clone();
            move || {
                let connection_widget = connection_widget.clone();
                let ui_handle = ui_handle.clone();
                let _ = slint::spawn_local(async move {
                    let mut widget = connection_widget.lock().await;
                    if let Err(e) = widget.refresh_ports() {
                        tracing::error!("Failed to refresh ports: {}", e);
                    } else {
                        // Update UI with new port list
                        if let Some(ui) = ui_handle.upgrade() {
                            let available_ports = widget.available_ports.iter()
                                .map(|p| slint::SharedString::from(p.clone()))
                                .collect::<Vec<_>>();
                            ui.set_available_ports(slint::ModelRc::new(slint::VecModel::from(available_ports)));
                            tracing::info!("Ports refreshed: {:?}", widget.available_ports);
                        }
                    }
                });
            }
        });

        // Connect callback
        ui.on_connect({
            let connection_widget = Arc::clone(&connection_widget);
            let grbl_controller = Arc::clone(&grbl_controller);
            let ui_handle = ui_handle.clone();
            move |port: slint::SharedString| {
                let connection_widget = connection_widget.clone();
                let grbl_controller = grbl_controller.clone();
                let ui_handle = ui_handle.clone();
                let port_str = port.to_string();
                let _ = slint::spawn_local(async move {
                    let mut widget = connection_widget.lock().await;
                    match widget
                        .connect(&grbl_controller, port_str.clone())
                        .await
                    {
                        Ok(_) => {
                            tracing::info!("Connected to device on {}", port_str);
                            if let Some(ui) = ui_handle.upgrade() {
                                ui.set_is_connected(true);
                                ui.set_selected_port(port_str.clone().into());
                                ui.set_connection_status(format!("Connected - {}", port_str).into());
                            }
                        }
                        Err(e) => {
                            tracing::error!("Connection failed: {}", e);
                            if let Some(ui) = ui_handle.upgrade() {
                                ui.set_is_connected(false);
                                ui.set_connection_status(format!("Failed: {}", e).into());
                            }
                        }
                    }
                });
            }
        });

        // Disconnect callback
        ui.on_disconnect({
            let connection_widget = Arc::clone(&connection_widget);
            let grbl_controller = Arc::clone(&grbl_controller);
            let ui_handle = ui_handle.clone();
            move || {
                let connection_widget = connection_widget.clone();
                let grbl_controller = grbl_controller.clone();
                let ui_handle = ui_handle.clone();
                let _ = slint::spawn_local(async move {
                    let mut widget = connection_widget.lock().await;
                    match widget.disconnect(&grbl_controller).await {
                        Ok(_) => {
                            tracing::info!("Disconnected from device");
                            if let Some(ui) = ui_handle.upgrade() {
                                ui.set_is_connected(false);
                                ui.set_connection_status("Disconnected".into());
                                ui.set_selected_port("".into());
                            }
                        }
                        Err(e) => {
                            tracing::error!("Disconnection failed: {}", e);
                            if let Some(ui) = ui_handle.upgrade() {
                                ui.set_connection_status(format!("Disconnect error: {}", e).into());
                            }
                        }
                    }
                });
            }
        });
    }

    // Set up console handlers
    {
        let console_buffer = console_buffer.clone();
        let ui_handle = ui.as_weak();
        
        // Send command
        ui.on_send_command({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |cmd: slint::SharedString| {
                let cmd_str = cmd.to_string();
                tracing::info!("User command: {}", cmd_str);
                add_console_message(&console_buffer, format!("TX: {}", cmd_str));
                
                // Update UI
                if let Some(ui) = ui_handle.upgrade() {
                    let content = console_logger::get_console_as_string(&console_buffer);
                    ui.set_console_content(content.into());
                }
            }
        });

        // Clear console
        ui.on_clear_console({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move || {
                console_logger::clear_console_logs(&console_buffer);
                
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_console_content("[System] Console cleared".into());
                }
            }
        });
        
        // Copy console
        ui.on_copy_console({
            let console_buffer = console_buffer.clone();
            move || {
                let content = console_logger::get_console_as_string(&console_buffer);
                tracing::info!("Console copied to clipboard ({} chars)", content.len());
                // TODO: Implement clipboard copy using clipboard crate or similar
            }
        });
        
        // Save console
        ui.on_save_console({
            let console_buffer = console_buffer.clone();
            move || {
                let content = console_logger::get_console_as_string(&console_buffer);
                
                // Show file save dialog
                if let Some(file) = rfd::FileDialog::new()
                    .set_title("Save Console Output")
                    .add_filter("Text files", &["txt"])
                    .add_filter("All files", &["*"])
                    .save_file() 
                {
                    let file_path = file.to_string_lossy().to_string();
                    
                    // Write file
                    match std::fs::write(&file_path, &content) {
                        Ok(_) => {
                            info!("Console output saved to: {} ({} chars)", file_path, content.len());
                        }
                        Err(e) => {
                            info!("Failed to save console output to {}: {}", file_path, e);
                        }
                    }
                }
            }
        });
        
        // Toggle filters
        ui.on_toggle_info({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |val: bool| {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_show_info(val);
                    update_console_display(&ui, &console_buffer);
                }
            }
        });
        
        ui.on_toggle_debug({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |val: bool| {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_show_debug(val);
                    update_console_display(&ui, &console_buffer);
                }
            }
        });
        
        ui.on_toggle_warn({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |val: bool| {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_show_warn(val);
                    update_console_display(&ui, &console_buffer);
                }
            }
        });
        
        ui.on_toggle_error({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |val: bool| {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_show_error(val);
                    update_console_display(&ui, &console_buffer);
                }
            }
        });
        
        ui.on_toggle_trace({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |val: bool| {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_show_trace(val);
                    update_console_display(&ui, &console_buffer);
                }
            }
        });
        
        ui.on_toggle_other({
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            move |val: bool| {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_show_other(val);
                    update_console_display(&ui, &console_buffer);
                }
            }
        });
        
        // Periodic update to show new console messages
        {
            let console_buffer = console_buffer.clone();
            let ui_handle = ui_handle.clone();
            let _ = slint::spawn_local(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    if let Some(ui) = ui_handle.upgrade() {
                        update_console_display(&ui, &console_buffer);
                    }
                }
            });
        }
    }

    tracing::debug!("UI event handlers setup complete");
    Ok(())
}

