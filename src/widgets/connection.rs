//! Connection Widget - Device selection and connection management

use serde::{Deserialize, Serialize};
use tracing::info;

/// Connection widget state
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
            available_ports: vec![
                "/dev/ttyACM0".to_string(),
                "/dev/ttyUSB0".to_string(),
                "COM3".to_string(),
                "COM4".to_string(),
            ],
            baud_rate: 115200,
            status_message: "Disconnected".to_string(),
        }
    }

    /// Refresh available ports
    pub fn refresh_ports(&mut self) {
        info!("Refreshing available COM ports");
        // Simulate port detection
        self.available_ports = vec![
            "/dev/ttyACM0".to_string(),
            "/dev/ttyUSB0".to_string(),
        ];
    }

    /// Connect to selected port
    pub fn connect(&mut self, port: String) -> Result<(), String> {
        if port.is_empty() {
            return Err("No port selected".to_string());
        }
        info!("Connecting to port: {}", port);
        self.port = port;
        self.connected = true;
        self.status_message = format!("Connected to {}", self.port);
        Ok(())
    }

    /// Disconnect from device
    pub fn disconnect(&mut self) -> Result<(), String> {
        info!("Disconnecting from port: {}", self.port);
        self.connected = false;
        self.status_message = "Disconnected".to_string();
        Ok(())
    }

    /// Set baud rate
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
}

impl Default for ConnectionWidget {
    fn default() -> Self {
        Self::new()
    }
}
