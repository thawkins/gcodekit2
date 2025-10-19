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

    /// Filter ports to only include valid GRBL device ports
    /// Valid ports: COM followed by number, /dev/ttyUSB followed by number, /dev/ttyACM followed by number
    fn filter_valid_ports(ports: Vec<String>) -> Vec<String> {
        ports.into_iter()
            .filter(|port| {
                // Windows: COM1, COM2, COM3, etc.
                if port.starts_with("COM") {
                    return port[3..].chars().all(|c| c.is_numeric());
                }
                
                // Linux USB: /dev/ttyUSB0, /dev/ttyUSB1, etc.
                if port.starts_with("/dev/ttyUSB") {
                    return port[11..].chars().all(|c| c.is_numeric());
                }
                
                // Linux ACM: /dev/ttyACM0, /dev/ttyACM1, etc.
                if port.starts_with("/dev/ttyACM") {
                    return port[11..].chars().all(|c| c.is_numeric());
                }
                
                // macOS: /dev/tty.usbserial- or /dev/cu.usbserial-
                if port.starts_with("/dev/tty.usbserial-") || port.starts_with("/dev/cu.usbserial-") {
                    return true;
                }
                
                false
            })
            .collect()
    }

    /// Refresh available ports from the system
    pub fn refresh_ports(&mut self) -> Result<()> {
        info!("Refreshing available COM ports");
        match SerialConnection::list_ports() {
            Ok(ports) => {
                // Filter to only valid GRBL device ports
                let filtered_ports = Self::filter_valid_ports(ports);
                info!("Found {} valid GRBL ports: {:?}", filtered_ports.len(), filtered_ports);
                self.available_ports = filtered_ports;
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
