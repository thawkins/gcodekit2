//! Connection Widget - Device selection and connection management
//!
//! Manages GRBL device connection through serial ports, providing port detection,
//! connection/disconnection, and status monitoring integrated with GrblController.

use crate::communication::{GrblController, SerialConnection};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// Connection widget state synchronized with GrblController
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionWidget {
    pub port: String,
    pub connected: bool,
    pub available_ports: Vec<String>,
    pub baud_rate: u32,
    pub status_message: String,
}

impl ConnectionWidget {
    /// Create a new connection widget
    pub fn new() -> Self {
        ConnectionWidget {
            port: String::new(),
            connected: false,
            available_ports: Vec::new(),
            baud_rate: 115200,
            status_message: "Disconnected".to_string(),
        }
    }

    /// Refresh available ports from the system
    pub fn refresh_ports(&mut self) -> Result<()> {
        info!("Refreshing available COM ports");
        match SerialConnection::list_ports() {
            Ok(ports) => {
                self.available_ports = ports;
                Ok(())
            }
            Err(e) => {
                warn!("Failed to refresh ports: {}", e);
                self.available_ports.clear();
                Err(e)
            }
        }
    }

    /// Connect to selected port using GrblController
    pub async fn connect(&mut self, controller: &GrblController, port: String) -> Result<()> {
        if port.is_empty() {
            return Err(anyhow::anyhow!("No port selected"));
        }
        info!("Connecting to port: {}", port);

        match controller.connect(&port).await {
            Ok(_) => {
                self.port = port;
                self.connected = true;
                self.status_message = format!("Connected to {}", self.port);
                info!("Successfully connected to {}", self.port);
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to port: {}", e);
                self.connected = false;
                self.status_message = format!("Failed: {}", e);
                Err(e)
            }
        }
    }

    /// Disconnect from device using GrblController
    pub async fn disconnect(&mut self, controller: &GrblController) -> Result<()> {
        if self.port.is_empty() {
            return Err(anyhow::anyhow!("No port connected"));
        }
        info!("Disconnecting from port: {}", self.port);

        match controller.disconnect().await {
            Ok(_) => {
                self.connected = false;
                self.status_message = "Disconnected".to_string();
                self.port.clear();
                info!("Successfully disconnected");
                Ok(())
            }
            Err(e) => {
                error!("Failed to disconnect: {}", e);
                Err(e)
            }
        }
    }

    /// Set baud rate (for configuration before connection)
    pub fn set_baud_rate(&mut self, rate: u32) {
        self.baud_rate = rate;
        info!("Baud rate set to: {}", rate);
    }

    /// Get connection status as string
    pub fn get_status(&self) -> String {
        if self.connected {
            format!("Connected - {} @ {}", self.port, self.baud_rate)
        } else {
            "Disconnected".to_string()
        }
    }

    /// Sync widget state with controller status
    pub async fn sync_with_controller(&mut self, controller: &GrblController) {
        let is_connected = controller.is_connected().await;
        if is_connected != self.connected {
            self.connected = is_connected;
            self.status_message = if is_connected {
                format!("Connected - {} @ {}", self.port, self.baud_rate)
            } else {
                "Disconnected".to_string()
            };
        }
    }
}

impl Default for ConnectionWidget {
    fn default() -> Self {
        Self::new()
    }
}
