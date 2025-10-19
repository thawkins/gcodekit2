//! WebSocket connection management for real-time pendant communication
//!
//! Handles bidirectional WebSocket connections for streaming real-time status updates,
//! sending commands, and managing multiple concurrent connections.
//!
//! # Message Protocol
//! - Status updates: `{"type":"status","data":{...}}`
//! - Command responses: `{"type":"command","data":{...}}`
//! - Error messages: `{"type":"error","message":"..."}`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

/// WebSocket message types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    /// Status update message
    #[serde(rename = "status")]
    Status,
    /// Command response message
    #[serde(rename = "command")]
    Command,
    /// Error message
    #[serde(rename = "error")]
    Error,
    /// Connection acknowledgment
    #[serde(rename = "connected")]
    Connected,
}

/// WebSocket message envelope
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsMessage {
    /// Message type
    pub msg_type: String,
    /// Message timestamp (Unix milliseconds)
    pub timestamp: u64,
    /// Message payload
    pub data: serde_json::Value,
}

/// Connection metadata
#[derive(Clone, Debug)]
pub struct ConnectionMetadata {
    /// Connection ID
    pub id: u64,
    /// Client IP address
    pub client_ip: String,
    /// Connection start time (Unix seconds)
    pub connected_at: u64,
    /// Messages sent
    pub messages_sent: Arc<AtomicU64>,
    /// Messages received
    pub messages_received: Arc<AtomicU64>,
    /// Last activity timestamp
    pub last_activity: Arc<RwLock<u64>>,
}

impl ConnectionMetadata {
    /// Create new connection metadata
    pub fn new(id: u64, client_ip: String, connected_at: u64) -> Self {
        Self {
            id,
            client_ip,
            connected_at,
            messages_sent: Arc::new(AtomicU64::new(0)),
            messages_received: Arc::new(AtomicU64::new(0)),
            last_activity: Arc::new(RwLock::new(connected_at)),
        }
    }

    /// Increment messages sent counter
    pub fn record_sent(&self) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment messages received counter
    pub fn record_received(&self) {
        self.messages_received.fetch_add(1, Ordering::Relaxed);
    }

    /// Update last activity timestamp
    pub async fn update_activity(&self, timestamp: u64) {
        let mut last_activity = self.last_activity.write().await;
        *last_activity = timestamp;
    }
}

/// WebSocket connection manager
pub struct WsConnectionManager {
    /// Active connections indexed by ID
    connections: Arc<RwLock<HashMap<u64, ConnectionMetadata>>>,
    /// Next connection ID
    next_id: Arc<AtomicU64>,
}

impl WsConnectionManager {
    /// Create new connection manager
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Register new connection
    pub async fn register(&self, client_ip: String, connected_at: u64) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let metadata = ConnectionMetadata::new(id, client_ip, connected_at);
        let mut conns = self.connections.write().await;
        conns.insert(id, metadata);
        id
    }

    /// Unregister connection
    pub async fn unregister(&self, id: u64) {
        let mut conns = self.connections.write().await;
        conns.remove(&id);
    }

    /// Get active connection count
    pub async fn active_count(&self) -> usize {
        let conns = self.connections.read().await;
        conns.len()
    }

    /// Get connection metadata
    pub async fn get_connection(&self, id: u64) -> Option<ConnectionMetadata> {
        let conns = self.connections.read().await;
        conns.get(&id).cloned()
    }

    /// Get all active connections
    pub async fn list_connections(&self) -> Vec<ConnectionMetadata> {
        let conns = self.connections.read().await;
        conns.values().cloned().collect()
    }
}

impl Default for WsConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_message_creation() {
        let msg = WsMessage {
            msg_type: "status".to_string(),
            timestamp: 1634000000000,
            data: serde_json::json!({"connected": true}),
        };
        assert_eq!(msg.msg_type, "status");
    }

    #[test]
    fn test_connection_metadata_creation() {
        let conn = ConnectionMetadata::new(1, "127.0.0.1".to_string(), 1634000000);
        assert_eq!(conn.id, 1);
        assert_eq!(conn.client_ip, "127.0.0.1");
    }

    #[test]
    fn test_connection_metadata_counters() {
        let conn = ConnectionMetadata::new(1, "127.0.0.1".to_string(), 1634000000);
        conn.record_sent();
        conn.record_received();
        assert_eq!(conn.messages_sent.load(Ordering::Relaxed), 1);
        assert_eq!(conn.messages_received.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_ws_connection_manager() {
        let manager = WsConnectionManager::new();
        let id = manager
            .register("192.168.1.100".to_string(), 1634000000)
            .await;
        assert!(id > 0);
        assert_eq!(manager.active_count().await, 1);

        let conn = manager.get_connection(id).await;
        assert!(conn.is_some());
        let conn = conn.unwrap();
        assert_eq!(conn.client_ip, "192.168.1.100");

        manager.unregister(id).await;
        assert_eq!(manager.active_count().await, 0);
    }

    #[tokio::test]
    async fn test_ws_connection_list() {
        let manager = WsConnectionManager::new();
        manager.register("192.168.1.1".to_string(), 1000).await;
        manager.register("192.168.1.2".to_string(), 1000).await;
        manager.register("192.168.1.3".to_string(), 1000).await;

        let connections = manager.list_connections().await;
        assert_eq!(connections.len(), 3);
    }
}
