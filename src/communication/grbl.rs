//! GRBL protocol communication
//!
//! Manages serial port communication with GRBL firmware controllers,
//! including version detection, status queries, and real-time control.

use anyhow::{Result, Context};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

/// Represents a connection to a GRBL device
pub struct GrblController {
    port: Box<dyn SerialPort>,
    version: String,
}

impl GrblController {
    /// Connect to a GRBL device on the specified port
    pub fn connect(port_name: &str) -> Result<Self> {
        let port = serialport::new(port_name, 115200)
            .timeout(Duration::from_secs(2))
            .open()
            .context(format!("Failed to open serial port: {}", port_name))?;

        let mut controller = Self {
            port,
            version: String::new(),
        };

        // Request version from GRBL
        controller.detect_version()?;

        Ok(controller)
    }

    /// Detect GRBL version by sending the version command
    fn detect_version(&mut self) -> Result<()> {
        self.send_command("$I")?;
        let response = self.read_response()?;
        self.version = response;
        tracing::info!("GRBL version detected: {}", self.version);
        Ok(())
    }

    /// Send a command to the GRBL device
    pub fn send_command(&mut self, cmd: &str) -> Result<()> {
        self.port.write_all(cmd.as_bytes())?;
        self.port.write_all(b"\n")?;
        self.port.flush()?;
        Ok(())
    }

    /// Read a response from the device
    pub fn read_response(&mut self) -> Result<String> {
        let mut buffer = [0u8; 1024];
        let n = self.port.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }

    /// Get the detected GRBL version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Request current machine status
    pub fn get_status(&mut self) -> Result<String> {
        self.send_command("?")?;
        self.read_response()
    }
}
