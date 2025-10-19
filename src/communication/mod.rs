//! GRBL Protocol Communication Module
//!
//! Handles GRBL firmware communication including serial port management,
//! command sending, response parsing, version detection, and real-time status monitoring.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{error, info, warn};

mod grbl;
mod serial;
pub use grbl::*;
pub use serial::{SerialConfig, SerialConnection};

/// GRBL machine state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub enum MachineState {
    #[default]
    Unknown,
    Idle,
    Run,
    Hold,
    Jog,
    Alarm,
    Door,
    Check,
    Home,
    Sleep,
}

impl MachineState {
    /// Parse state from GRBL status response
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "IDLE" => MachineState::Idle,
            "RUN" => MachineState::Run,
            "HOLD" => MachineState::Hold,
            "JOG" => MachineState::Jog,
            "ALARM" => MachineState::Alarm,
            "DOOR" => MachineState::Door,
            "CHECK" => MachineState::Check,
            "HOME" => MachineState::Home,
            "SLEEP" => MachineState::Sleep,
            _ => MachineState::Unknown,
        }
    }

    /// Get color code for this state (for UI display)
    pub fn color(&self) -> &'static str {
        match self {
            MachineState::Idle => "#00AA00", // Green
            MachineState::Run => "#0000FF",  // Blue
            MachineState::Jog => "#0000FF",  // Blue
            MachineState::Hold => "#FFFF00", // Yellow
            MachineState::Door => "#FFFF00", // Yellow
            MachineState::Alarm => "#FF0000", // Red
            MachineState::Check => "#FF0000", // Red
            _ => "#CCCCCC", // Gray
        }
    }
}

/// GRBL device position (X, Y, Z coordinates)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// GRBL status response
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrblStatus {
    pub state: MachineState,
    pub mpos: Position,
    pub wpos: Position,
    pub feed_rate: u32,
    pub spindle_speed: u32,
    pub version: String,
    pub connected: bool,
}

/// Response type from GRBL device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrblResponse {
    Ok,
    Error(String),
    Status(GrblStatus),
    Version(String),
    Settings(String),
}

/// Error recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryConfig {
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub auto_reconnect: bool,
    pub reconnect_delay_ms: u64,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        RecoveryConfig {
            max_retries: 3,
            retry_delay_ms: 500,
            auto_reconnect: true,
            reconnect_delay_ms: 2000,
        }
    }
}

/// GRBL Controller for managing device communication
pub struct GrblController {
    serial: Arc<SerialConnection>,
    port: Arc<Mutex<Option<String>>>,
    version: Arc<Mutex<String>>,
    status: Arc<Mutex<GrblStatus>>,
    recovery_config: Arc<Mutex<RecoveryConfig>>,
    pub command_queue: Arc<Mutex<VecDeque<String>>>,
    response_log: Arc<Mutex<VecDeque<String>>>,
}

impl GrblController {
    /// Create a new GRBL controller
    pub fn new() -> Self {
        GrblController {
            serial: Arc::new(SerialConnection::default_config()),
            port: Arc::new(Mutex::new(None)),
            version: Arc::new(Mutex::new(String::new())),
            status: Arc::new(Mutex::new(GrblStatus {
                connected: false,
                ..Default::default()
            })),
            recovery_config: Arc::new(Mutex::new(RecoveryConfig::default())),
            command_queue: Arc::new(Mutex::new(VecDeque::new())),
            response_log: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Create with custom serial configuration
    pub fn with_config(config: SerialConfig) -> Self {
        GrblController {
            serial: Arc::new(SerialConnection::new(config)),
            port: Arc::new(Mutex::new(None)),
            version: Arc::new(Mutex::new(String::new())),
            status: Arc::new(Mutex::new(GrblStatus {
                connected: false,
                ..Default::default()
            })),
            recovery_config: Arc::new(Mutex::new(RecoveryConfig::default())),
            command_queue: Arc::new(Mutex::new(VecDeque::new())),
            response_log: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Connect to a GRBL device on the specified port
    pub async fn connect(&self, port_name: &str) -> Result<()> {
        info!("Connecting to GRBL device on port: {}", port_name);

        // Attempt to connect with retries
        let config = self.recovery_config.lock().await;
        let mut attempts = 0;
        let max_attempts = config.max_retries as usize;

        loop {
            match self.serial.connect(port_name).await {
                Ok(_) => {
                    let mut port = self.port.lock().await;
                    *port = Some(port_name.to_string());

                    let mut status = self.status.lock().await;
                    status.connected = true;
                    status.state = MachineState::Idle;

                    info!("Connected to GRBL device on {}", port_name);
                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        error!("Failed to connect after {} attempts: {}", attempts, e);
                        return Err(e);
                    }
                    warn!(
                        "Connection attempt {} failed: {}. Retrying...",
                        attempts, e
                    );
                    sleep(Duration::from_millis(config.retry_delay_ms)).await;
                }
            }
        }
    }

    /// Disconnect from the device
    pub async fn disconnect(&self) -> Result<()> {
        info!("Disconnecting from GRBL device");
        self.serial.disconnect().await?;

        let mut port = self.port.lock().await;
        *port = None;

        let mut status = self.status.lock().await;
        status.connected = false;

        Ok(())
    }

    /// Detect GRBL firmware version
    pub async fn detect_version(&self) -> Result<String> {
        info!("Detecting GRBL version");

        // Send version request
        self.serial.send_command("$I").await?;

        // Read response
        match self.serial.read_response_timeout(256, Duration::from_secs(2)).await {
            Ok(response) => {
                let version = response.trim().to_string();
                let mut ver = self.version.lock().await;
                *ver = version.clone();

                let mut status = self.status.lock().await;
                status.version = version.clone();

                info!("Detected version: {}", version);
                Ok(version)
            }
            Err(e) => {
                warn!("Failed to detect version: {}", e);
                Err(e)
            }
        }
    }

    /// Send a command to GRBL
    pub async fn send_command(&self, command: &str) -> Result<()> {
        // Queue the command
        let mut queue = self.command_queue.lock().await;
        queue.push_back(command.to_string());

        // Send to device
        self.serial.send_command(command).await?;

        // Try to read response
        match self.serial.read_response_timeout(256, Duration::from_secs(1)).await {
            Ok(response) => {
                self.log_response(response).await;
                info!("Command {} sent successfully", command);
                Ok(())
            }
            Err(e) => {
                warn!("Command {} sent but no immediate response: {}", command, e);
                // Command was sent, but we might not get immediate response
                // This is not necessarily an error
                Ok(())
            }
        }
    }

    /// Get the next queued command
    pub async fn get_next_command(&self) -> Option<String> {
        let mut queue = self.command_queue.lock().await;
        queue.pop_front()
    }

    /// Get current status from GRBL
    pub async fn get_status(&self) -> Result<GrblStatus> {
        let status = self.status.lock().await;
        Ok(status.clone())
    }

    /// Update machine status
    pub async fn update_status(
        &self,
        state: MachineState,
        mpos: Position,
        wpos: Position,
        feed_rate: u32,
        spindle_speed: u32,
    ) {
        let mut status = self.status.lock().await;
        status.state = state;
        status.mpos = mpos;
        status.wpos = wpos;
        status.feed_rate = feed_rate;
        status.spindle_speed = spindle_speed;
        info!(
            "Status: {:?} MPos({:.2}, {:.2}, {:.2})",
            state, mpos.x, mpos.y, mpos.z
        );
    }

    /// Add response to log
    pub async fn log_response(&self, response: String) {
        let mut log = self.response_log.lock().await;
        log.push_back(response);
        if log.len() > 1000 {
            log.pop_front();
        }
    }

    /// Get response log
    pub async fn get_response_log(&self) -> Vec<String> {
        let log = self.response_log.lock().await;
        log.iter().cloned().collect()
    }

    /// Clear response log
    pub async fn clear_response_log(&self) {
        let mut log = self.response_log.lock().await;
        log.clear();
    }

    /// Get is connected status
    pub async fn is_connected(&self) -> bool {
        let status = self.status.lock().await;
        status.connected
    }

    /// Set recovery configuration
    pub async fn set_recovery_config(&self, config: RecoveryConfig) {
        let mut cfg = self.recovery_config.lock().await;
        *cfg = config;
    }

    /// Get recovery configuration
    pub async fn get_recovery_config(&self) -> RecoveryConfig {
        let cfg = self.recovery_config.lock().await;
        cfg.clone()
    }

    /// Emergency stop
    pub async fn emergency_stop(&self) -> Result<()> {
        info!("Emergency stop triggered");
        let mut status = self.status.lock().await;
        status.state = MachineState::Alarm;
        Ok(())
    }

    /// Reset machine alarm
    pub async fn reset_alarm(&self) -> Result<()> {
        info!("Resetting machine alarm");
        self.send_command("$X").await?;
        Ok(())
    }

    /// Unlock machine
    pub async fn unlock(&self) -> Result<()> {
        info!("Unlocking machine");
        self.send_command("$X").await?;
        Ok(())
    }
}

impl Default for GrblController {
    fn default() -> Self {
        Self::new()
    }
}
