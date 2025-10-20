//! Serial port communication handler for GRBL devices
//!
//! Provides asynchronous serial port management, command sending/receiving,
//! and real-time status monitoring with error recovery.

use anyhow::{anyhow, Context, Result};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Serial port configuration
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub baud_rate: u32,
    pub data_bits: serialport::DataBits,
    pub stop_bits: serialport::StopBits,
    pub parity: serialport::Parity,
    pub flow_control: serialport::FlowControl,
    pub timeout: Duration,
}

impl Default for SerialConfig {
    fn default() -> Self {
        SerialConfig {
            baud_rate: 115200,
            data_bits: serialport::DataBits::Eight,
            stop_bits: serialport::StopBits::One,
            parity: serialport::Parity::None,
            flow_control: serialport::FlowControl::None,
            timeout: Duration::from_millis(500),
        }
    }
}

/// Serial port connection handler
pub struct SerialConnection {
    port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    config: SerialConfig,
    port_name: Arc<Mutex<String>>,
}

impl SerialConnection {
    /// Create a new serial connection handler
    pub fn new(config: SerialConfig) -> Self {
        SerialConnection {
            port: Arc::new(Mutex::new(None)),
            config,
            port_name: Arc::new(Mutex::new(String::new())),
        }
    }

    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(SerialConfig::default())
    }

    /// Connect to a serial port
    pub async fn connect(&self, port_name: &str) -> Result<()> {

        // Try to open the serial port
        let serial_port = serialport::new(port_name, self.config.baud_rate)
            .data_bits(self.config.data_bits)
            .stop_bits(self.config.stop_bits)
            .parity(self.config.parity)
            .flow_control(self.config.flow_control)
            .timeout(self.config.timeout)
            .open()
            .context(format!(
                "Failed to open serial port: {}",
                port_name
            ))?;

        let mut port = self.port.lock().await;
        *port = Some(serial_port);

        let mut stored_port_name = self.port_name.lock().await;
        *stored_port_name = port_name.to_string();

        Ok(())
    }

    /// Disconnect from the serial port
    pub async fn disconnect(&self) -> Result<()> {
        let mut port = self.port.lock().await;
        *port = None;
        Ok(())
    }

    /// Send raw bytes to the device
    pub async fn send_bytes(&self, data: &[u8]) -> Result<usize> {
        let mut port = self.port.lock().await;
        let port_ref = port
            .as_mut()
            .ok_or_else(|| anyhow!("Serial port not connected"))?;

        let bytes_written = port_ref
            .write(data)
            .context("Failed to write to serial port")?;

        Ok(bytes_written)
    }

    /// Send a string command (with newline)
    pub async fn send_command(&self, command: &str) -> Result<()> {
        let command_with_newline = format!("{}\n", command);
        self.send_bytes(command_with_newline.as_bytes()).await?;
        Ok(())
    }

    /// Read response from device (with timeout)
    pub async fn read_response(&self, max_size: usize) -> Result<String> {
        let mut buffer = vec![0u8; max_size];

        let mut port = self.port.lock().await;
        let port_ref = port
            .as_mut()
            .ok_or_else(|| anyhow!("Serial port not connected"))?;

        match port_ref.read(&mut buffer) {
            Ok(n) if n > 0 => {
                buffer.truncate(n);
                let response = String::from_utf8_lossy(&buffer).to_string();
                Ok(response)
            }
            Ok(_) => Err(anyhow!("No data received from serial port")),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                Err(anyhow!("Serial port read timeout"))
            }
            Err(e) => Err(anyhow!("Serial port read error: {}", e)),
        }
    }

    /// Read response with custom timeout
    pub async fn read_response_timeout(
        &self,
        max_size: usize,
        timeout: Duration,
    ) -> Result<String> {
        let result = tokio::time::timeout(timeout, self.read_response(max_size)).await;

        match result {
            Ok(response_result) => response_result,
            Err(_) => Err(anyhow!("Read operation timed out")),
        }
    }

    /// Check if port is connected
    pub async fn is_connected(&self) -> bool {
        let port = self.port.lock().await;
        port.is_some()
    }

    /// Get the connected port name
    pub async fn get_port_name(&self) -> String {
        let port_name = self.port_name.lock().await;
        port_name.clone()
    }

    /// List available serial ports
    pub fn list_ports() -> Result<Vec<String>> {
        let ports = serialport::available_ports()
            .context("Failed to enumerate serial ports")?;

        let port_names: Vec<String> = ports
            .iter()
            .filter_map(|port| match port {
                serialport::SerialPortInfo {
                    port_name,
                    port_type: _,
                } => Some(port_name.clone()),
            })
            .collect();

        Ok(port_names)
    }
}

impl Drop for SerialConnection {
    fn drop(&mut self) {
        // Ensure port is closed when the connection is dropped
        if let Ok(mut port_guard) = self.port.try_lock() {
            *port_guard = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_config_defaults() {
        let config = SerialConfig::default();
        assert_eq!(config.baud_rate, 115200);
        assert_eq!(config.timeout, Duration::from_millis(500));
    }

    #[tokio::test]
    async fn test_serial_connection_creation() {
        let conn = SerialConnection::default_config();
        assert!(!conn.is_connected().await);
    }

    #[tokio::test]
    async fn test_serial_connection_port_name() {
        let conn = SerialConnection::default_config();
        assert_eq!(conn.get_port_name().await, "");
    }

    #[test]
    fn test_list_available_ports() {
        // This test may fail if no serial ports are available
        let result = SerialConnection::list_ports();
        assert!(result.is_ok());
    }
}
