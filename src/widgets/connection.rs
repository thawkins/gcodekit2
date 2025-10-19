//! Connection widget for device connection and status display

/// Device connection widget
pub struct ConnectionWidget {
    pub port: String,
    pub connected: bool,
    pub status: String,
}

impl ConnectionWidget {
    /// Create a new connection widget
    pub fn new() -> Self {
        Self {
            port: String::new(),
            connected: false,
            status: "Disconnected".to_string(),
        }
    }

    /// Get available serial ports
    pub fn available_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .iter()
            .filter_map(|port| {
                if let serialport::SerialPortType::UsbPort(info) = &port.port_type {
                    Some(port.port_name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Update connection status
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }
}

impl Default for ConnectionWidget {
    fn default() -> Self {
        Self::new()
    }
}
