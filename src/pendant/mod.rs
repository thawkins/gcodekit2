//! Web Pendant Interface Module
//!
//! Provides remote control capabilities via web-based pendant interface with
//! real-time streaming, mobile responsiveness, and cross-browser compatibility.
//!
//! # Features
//! - RESTful API for machine control and status queries
//! - WebSocket support for real-time bidirectional communication
//! - Mobile-responsive HTML5 interface
//! - Cross-browser compatibility (Chrome, Firefox, Safari, Edge)
//! - Real-time position, feed rate, spindle speed updates
//! - Jog, override, and emergency stop commands
//!
//! # Architecture
//! - `server`: HTTP/WebSocket server implementation
//! - `api`: RESTful endpoint handlers
//! - `ws`: WebSocket connection management
//! - `ui`: Mobile-responsive HTML/CSS/JS interface

pub mod api;
pub mod server;
pub mod ui;
pub mod ws;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Pendant server configuration
#[derive(Clone, Debug)]
pub struct PendantConfig {
    /// Server listen address (default: 0.0.0.0)
    pub listen_addr: String,
    /// Server listen port (default: 8080)
    pub listen_port: u16,
    /// Enable CORS for cross-origin requests
    pub enable_cors: bool,
    /// WebSocket connection timeout (seconds)
    pub ws_timeout: u64,
    /// Maximum concurrent WebSocket connections
    pub max_ws_connections: usize,
}

impl Default for PendantConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0".to_string(),
            listen_port: 8080,
            enable_cors: true,
            ws_timeout: 30,
            max_ws_connections: 10,
        }
    }
}

/// Web pendant server state
pub struct PendantServer {
    config: PendantConfig,
    // State would be shared with main app
}

impl PendantServer {
    /// Create a new pendant server with default config
    pub fn new() -> Self {
        Self {
            config: PendantConfig::default(),
        }
    }

    /// Create pendant server with custom configuration
    pub fn with_config(config: PendantConfig) -> Self {
        Self { config }
    }

    /// Get server configuration
    pub fn config(&self) -> &PendantConfig {
        &self.config
    }
}

impl Default for PendantServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pendant_config_default() {
        let config = PendantConfig::default();
        assert_eq!(config.listen_addr, "0.0.0.0");
        assert_eq!(config.listen_port, 8080);
        assert!(config.enable_cors);
        assert_eq!(config.ws_timeout, 30);
        assert_eq!(config.max_ws_connections, 10);
    }

    #[test]
    fn test_pendant_server_creation() {
        let server = PendantServer::new();
        assert_eq!(server.config.listen_port, 8080);
    }

    #[test]
    fn test_pendant_server_custom_config() {
        let config = PendantConfig {
            listen_addr: "127.0.0.1".to_string(),
            listen_port: 9090,
            enable_cors: false,
            ws_timeout: 60,
            max_ws_connections: 5,
        };
        let server = PendantServer::with_config(config);
        assert_eq!(server.config.listen_port, 9090);
        assert_eq!(server.config.listen_addr, "127.0.0.1");
        assert!(!server.config.enable_cors);
    }
}
