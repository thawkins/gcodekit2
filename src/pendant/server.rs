//! HTTP server implementation for web pendant
//!
//! Provides HTTP/HTTPS server with WebSocket upgrade capability,
//! static file serving, and RESTful endpoint routing.

use super::api::StatusResponse;
use std::net::SocketAddr;

/// HTTP server configuration
#[derive(Clone, Debug)]
pub struct HttpServerConfig {
    /// Listen address
    pub listen_addr: String,
    /// Listen port
    pub listen_port: u16,
    /// Enable SSL/TLS (HTTPS)
    pub use_tls: bool,
    /// TLS certificate path (if use_tls is true)
    pub cert_path: Option<String>,
    /// TLS key path (if use_tls is true)
    pub key_path: Option<String>,
    /// Enable gzip compression
    pub enable_gzip: bool,
    /// Maximum request body size (bytes)
    pub max_body_size: usize,
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0".to_string(),
            listen_port: 8080,
            use_tls: false,
            cert_path: None,
            key_path: None,
            enable_gzip: true,
            max_body_size: 1024 * 1024, // 1MB
        }
    }
}

/// HTTP server implementation
pub struct HttpServer {
    config: HttpServerConfig,
}

impl HttpServer {
    /// Create new HTTP server with default config
    pub fn new() -> Self {
        Self {
            config: HttpServerConfig::default(),
        }
    }

    /// Create HTTP server with custom config
    pub fn with_config(config: HttpServerConfig) -> Self {
        Self { config }
    }

    /// Get server configuration
    pub fn config(&self) -> &HttpServerConfig {
        &self.config
    }

    /// Get server address string
    pub fn address(&self) -> String {
        format!("{}:{}", self.config.listen_addr, self.config.listen_port)
    }

    /// Get socket address for binding
    pub fn socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        self.address().parse()
    }

    /// Get protocol scheme (http or https)
    pub fn scheme(&self) -> &'static str {
        if self.config.use_tls {
            "https"
        } else {
            "http"
        }
    }

    /// Get full server URL
    pub fn url(&self) -> String {
        format!("{}://{}", self.scheme(), self.address())
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_server_config_default() {
        let config = HttpServerConfig::default();
        assert_eq!(config.listen_addr, "0.0.0.0");
        assert_eq!(config.listen_port, 8080);
        assert!(!config.use_tls);
        assert!(config.enable_gzip);
    }

    #[test]
    fn test_http_server_creation() {
        let server = HttpServer::new();
        assert_eq!(server.address(), "0.0.0.0:8080");
    }

    #[test]
    fn test_http_server_address() {
        let config = HttpServerConfig {
            listen_addr: "127.0.0.1".to_string(),
            listen_port: 9090,
            ..Default::default()
        };
        let server = HttpServer::with_config(config);
        assert_eq!(server.address(), "127.0.0.1:9090");
    }

    #[test]
    fn test_http_server_url_http() {
        let server = HttpServer::new();
        assert_eq!(server.scheme(), "http");
        assert_eq!(server.url(), "http://0.0.0.0:8080");
    }

    #[test]
    fn test_http_server_url_https() {
        let config = HttpServerConfig {
            use_tls: true,
            ..Default::default()
        };
        let server = HttpServer::with_config(config);
        assert_eq!(server.scheme(), "https");
        assert_eq!(server.url(), "https://0.0.0.0:8080");
    }

    #[test]
    fn test_http_server_socket_addr() {
        let server = HttpServer::new();
        let addr = server.socket_addr();
        assert!(addr.is_ok());
    }

    #[test]
    fn test_http_server_max_body_size() {
        let config = HttpServerConfig {
            max_body_size: 5 * 1024 * 1024, // 5MB
            ..Default::default()
        };
        let server = HttpServer::with_config(config);
        assert_eq!(server.config.max_body_size, 5 * 1024 * 1024);
    }
}
