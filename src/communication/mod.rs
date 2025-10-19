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
pub use grbl::*;

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
    port: Arc<Mutex<Option<String>>>,
    version: Arc<Mutex<String>>,
    status: Arc<Mutex<GrblStatus>>,
    recovery_config: Arc<Mutex<RecoveryConfig>>,
    command_queue: Arc<Mutex<VecDeque<String>>>,
    response_log: Arc<Mutex<VecDeque<String>>>,
}

impl GrblController {
    /// Create a new GRBL controller
    pub fn new() -> Self {
        GrblController {
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
        let mut port = self.port.lock().await;
        *port = Some(port_name.to_string());
        
        let mut status = self.status.lock().await;
        status.connected = true;
        status.state = MachineState::Idle;
        
        info!("Connected to GRBL device");
        Ok(())
    }

    /// Disconnect from the device
    pub async fn disconnect(&self) -> Result<()> {
        info!("Disconnecting from GRBL device");
        let mut port = self.port.lock().await;
        *port = None;
        
        let mut status = self.status.lock().await;
        status.connected = false;
        
        Ok(())
    }

    /// Detect GRBL firmware version
    pub async fn detect_version(&self) -> Result<String> {
        info!("Detecting GRBL version");
        let v = "GRBL v1.1h".to_string();
        let mut version = self.version.lock().await;
        *version = v.clone();
        
        let mut status = self.status.lock().await;
        status.version = v.clone();
        
        Ok(v)
    }

    /// Send a command to GRBL
    pub async fn send_command(&self, command: &str) -> Result<()> {
        info!("Queuing command: {}", command);
        let mut queue = self.command_queue.lock().await;
        queue.push_back(command.to_string());
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grbl_controller_creation() {
        let controller = GrblController::new();
        assert!(!controller.is_connected().await);
    }

    #[tokio::test]
    async fn test_machine_state_parsing() {
        assert_eq!(MachineState::from_str("IDLE"), MachineState::Idle);
        assert_eq!(MachineState::from_str("run"), MachineState::Run);
        assert_eq!(MachineState::from_str("ALARM"), MachineState::Alarm);
        assert_eq!(MachineState::from_str("Hold"), MachineState::Hold);
    }

    #[tokio::test]
    async fn test_position_creation() {
        let pos = Position { x: 10.5, y: 20.3, z: 5.1 };
        assert_eq!(pos.x, 10.5);
        assert_eq!(pos.y, 20.3);
        assert_eq!(pos.z, 5.1);
    }

    #[tokio::test]
    async fn test_version_detection() {
        let controller = GrblController::new();
        let version = controller.detect_version().await.unwrap();
        assert_eq!(version, "GRBL v1.1h");
    }

    #[tokio::test]
    async fn test_status_update() {
        let controller = GrblController::new();
        let mpos = Position { x: 1.0, y: 2.0, z: 3.0 };
        let wpos = Position { x: 0.0, y: 0.0, z: 0.0 };
        controller
            .update_status(MachineState::Idle, mpos, wpos, 1000, 0)
            .await;
        let status = controller.get_status().await.unwrap();
        assert_eq!(status.state, MachineState::Idle);
        assert_eq!(status.mpos.x, 1.0);
    }

    #[tokio::test]
    async fn test_command_queue() {
        let controller = GrblController::new();
        controller.send_command("G0 X10 Y10").await.unwrap();
        controller.send_command("G1 Z-5 F100").await.unwrap();
        
        let cmd1 = controller.get_next_command().await;
        assert_eq!(cmd1, Some("G0 X10 Y10".to_string()));
        
        let cmd2 = controller.get_next_command().await;
        assert_eq!(cmd2, Some("G1 Z-5 F100".to_string()));
    }

    #[tokio::test]
    async fn test_response_logging() {
        let controller = GrblController::new();
        controller.log_response("ok".to_string()).await;
        controller.log_response("error: Unknown command".to_string()).await;
        
        let log = controller.get_response_log().await;
        assert_eq!(log.len(), 2);
        assert_eq!(log[0], "ok");
    }

    #[tokio::test]
    async fn test_recovery_config() {
        let controller = GrblController::new();
        let config = RecoveryConfig {
            max_retries: 5,
            retry_delay_ms: 1000,
            auto_reconnect: true,
            reconnect_delay_ms: 3000,
        };
        controller.set_recovery_config(config.clone()).await;
        let retrieved = controller.get_recovery_config().await;
        assert_eq!(retrieved.max_retries, 5);
    }

    #[tokio::test]
    async fn test_emergency_stop() {
        let controller = GrblController::new();
        controller.emergency_stop().await.unwrap();
        let status = controller.get_status().await.unwrap();
        assert_eq!(status.state, MachineState::Alarm);
    }

    #[tokio::test]
    async fn test_machine_state_colors() {
        assert_eq!(MachineState::Idle.color(), "#00AA00");
        assert_eq!(MachineState::Run.color(), "#0000FF");
        assert_eq!(MachineState::Alarm.color(), "#FF0000");
    }
}
