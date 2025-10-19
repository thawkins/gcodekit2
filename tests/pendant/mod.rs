//! Web Pendant Integration Tests
//!
//! Comprehensive tests for pendant server, API, WebSocket, and UI components

use gcodekit2::pendant::{PendantConfig, PendantServer};

#[test]
fn test_pendant_server_default_config() {
    let server = PendantServer::new();
    let config = server.config();
    assert_eq!(config.listen_port, 8080);
    assert_eq!(config.listen_addr, "0.0.0.0");
    assert!(config.enable_cors);
}

#[test]
fn test_pendant_server_custom_config() {
    let config = PendantConfig {
        listen_addr: "127.0.0.1".to_string(),
        listen_port: 9090,
        enable_cors: false,
        ws_timeout: 60,
        max_ws_connections: 20,
    };
    let server = PendantServer::with_config(config);
    assert_eq!(server.config().listen_port, 9090);
    assert_eq!(server.config().listen_addr, "127.0.0.1");
}

#[test]
fn test_pendant_config_default_values() {
    let config = PendantConfig::default();
    assert_eq!(config.listen_addr, "0.0.0.0");
    assert_eq!(config.listen_port, 8080);
    assert!(config.enable_cors);
    assert_eq!(config.ws_timeout, 30);
    assert_eq!(config.max_ws_connections, 10);
}

#[test]
fn test_pendant_api_status_response() {
    use gcodekit2::pendant::api::StatusResponse;
    let status = StatusResponse::default();
    assert!(!status.connected);
    assert_eq!(status.state, "disconnected");
    assert_eq!(status.pos_x, 0.0);
    assert_eq!(status.pos_y, 0.0);
    assert_eq!(status.pos_z, 0.0);
}

#[test]
fn test_pendant_api_jog_request() {
    use gcodekit2::pendant::api::JogRequest;
    let req = JogRequest {
        axis: "X".to_string(),
        distance: 10.0,
        feed_rate: Some(100.0),
    };
    assert_eq!(req.axis, "X");
    assert_eq!(req.distance, 10.0);
}

#[test]
fn test_pendant_api_override_request() {
    use gcodekit2::pendant::api::OverrideRequest;
    let req = OverrideRequest {
        override_type: "feed_rate".to_string(),
        value: 110,
    };
    assert_eq!(req.override_type, "feed_rate");
    assert_eq!(req.value, 110);
}

#[test]
fn test_pendant_api_error() {
    use gcodekit2::pendant::api::ApiError;
    let error = ApiError {
        code: 404,
        message: "Not found".to_string(),
    };
    assert_eq!(error.code, 404);
}

#[test]
fn test_pendant_http_server_default() {
    use gcodekit2::pendant::server::HttpServer;
    let server = HttpServer::new();
    assert_eq!(server.address(), "0.0.0.0:8080");
    assert_eq!(server.scheme(), "http");
    assert_eq!(server.url(), "http://0.0.0.0:8080");
}

#[test]
fn test_pendant_http_server_https() {
    use gcodekit2::pendant::server::{HttpServer, HttpServerConfig};
    let config = HttpServerConfig {
        use_tls: true,
        listen_port: 8443,
        ..Default::default()
    };
    let server = HttpServer::with_config(config);
    assert_eq!(server.scheme(), "https");
    assert_eq!(server.url(), "https://0.0.0.0:8443");
}

#[test]
fn test_pendant_http_server_socket_addr() {
    use gcodekit2::pendant::server::HttpServer;
    let server = HttpServer::new();
    let addr = server.socket_addr();
    assert!(addr.is_ok());
}

#[tokio::test]
async fn test_pendant_ws_connection_manager() {
    use gcodekit2::pendant::ws::WsConnectionManager;
    let manager = WsConnectionManager::new();

    let id1 = manager.register("192.168.1.1".to_string(), 1000).await;
    let id2 = manager.register("192.168.1.2".to_string(), 1000).await;

    assert_eq!(manager.active_count().await, 2);

    let conn = manager.get_connection(id1).await;
    assert!(conn.is_some());

    manager.unregister(id1).await;
    assert_eq!(manager.active_count().await, 1);
}

#[tokio::test]
async fn test_pendant_ws_connection_list() {
    use gcodekit2::pendant::ws::WsConnectionManager;
    let manager = WsConnectionManager::new();

    manager.register("192.168.1.1".to_string(), 1000).await;
    manager.register("192.168.1.2".to_string(), 1000).await;
    manager.register("192.168.1.3".to_string(), 1000).await;

    let conns = manager.list_connections().await;
    assert_eq!(conns.len(), 3);
}

#[test]
fn test_pendant_ws_message_creation() {
    use gcodekit2::pendant::ws::WsMessage;
    let msg = WsMessage {
        msg_type: "status".to_string(),
        timestamp: 1634000000000,
        data: serde_json::json!({"connected": true}),
    };
    assert_eq!(msg.msg_type, "status");
    assert_eq!(msg.timestamp, 1634000000000);
}

#[tokio::test]
async fn test_pendant_ws_connection_metadata() {
    use gcodekit2::pendant::ws::ConnectionMetadata;
    let conn = ConnectionMetadata::new(1, "127.0.0.1".to_string(), 1634000000);

    conn.record_sent();
    conn.record_sent();
    conn.record_received();

    assert_eq!(conn.messages_sent.load(std::sync::atomic::Ordering::Relaxed), 2);
    assert_eq!(conn.messages_received.load(std::sync::atomic::Ordering::Relaxed), 1);

    conn.update_activity(1634000005).await;
    let last_activity = conn.last_activity.read().await;
    assert_eq!(*last_activity, 1634000005);
}

#[test]
fn test_pendant_ui_context_default() {
    use gcodekit2::pendant::ui::UiContext;
    let ctx = UiContext::default();
    assert_eq!(ctx.title, "GCodeKit2 Web Pendant");
    assert!(!ctx.dark_mode);
}

#[test]
fn test_pendant_ui_context_custom() {
    use gcodekit2::pendant::ui::UiContext;
    let ctx = UiContext::new(
        "Custom Pendant".to_string(),
        "/api/v1".to_string(),
        "ws://example.com/ws".to_string(),
        true,
    );
    assert_eq!(ctx.title, "Custom Pendant");
    assert_eq!(ctx.api_url, "/api/v1");
    assert_eq!(ctx.ws_url, "ws://example.com/ws");
    assert!(ctx.dark_mode);
}

#[test]
fn test_pendant_ui_context_render() {
    use gcodekit2::pendant::ui::UiContext;
    let ctx = UiContext::default();
    let html = ctx.render_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("GCodeKit2 Web Pendant"));
    assert!(html.contains("Machine Status"));
    assert!(html.contains("Controls"));
}

#[test]
fn test_pendant_ui_context_render_dark_mode() {
    use gcodekit2::pendant::ui::UiContext;
    let ctx = UiContext {
        dark_mode: true,
        ..Default::default()
    };
    let html = ctx.render_html();
    assert!(html.contains("dark-mode"));
}

#[test]
fn test_pendant_ui_context_render_endpoints() {
    use gcodekit2::pendant::ui::UiContext;
    let ctx = UiContext::new(
        "Test".to_string(),
        "/api/v2".to_string(),
        "ws://server:8081/ws".to_string(),
        false,
    );
    let html = ctx.render_html();
    assert!(html.contains("/api/v2"));
    assert!(html.contains("ws://server:8081/ws"));
}

#[test]
fn test_pendant_server_creation() {
    let server = PendantServer::new();
    let config = server.config();
    assert_eq!(config.listen_port, 8080);
    assert_eq!(config.max_ws_connections, 10);
}
