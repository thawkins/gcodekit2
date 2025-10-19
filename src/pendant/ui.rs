//! Mobile-responsive HTML5 UI for web pendant
//!
//! Provides responsive web interface with support for mobile devices,
//! cross-browser compatibility, and real-time status updates.

use std::borrow::Cow;

/// Pendant UI template - responsive HTML5 interface
pub const PENDANT_UI_HTML: &str = include_str!("./ui_template.html");

/// Pendant UI CSS - responsive styles
pub const PENDANT_UI_CSS: &str = include_str!("./ui_styles.css");

/// Pendant UI JavaScript - real-time client logic
pub const PENDANT_UI_JS: &str = include_str!("./ui_client.js");

/// UI context data for template rendering
#[derive(Clone, Debug)]
pub struct UiContext {
    /// Application title
    pub title: String,
    /// Application version
    pub version: String,
    /// API base URL
    pub api_url: String,
    /// WebSocket URL
    pub ws_url: String,
    /// Enable dark mode
    pub dark_mode: bool,
}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            title: "GCodeKit2 Web Pendant".to_string(),
            version: "0.2.0-alpha".to_string(),
            api_url: "/api".to_string(),
            ws_url: "ws://localhost:8080/ws".to_string(),
            dark_mode: false,
        }
    }
}

impl UiContext {
    /// Create UI context with custom parameters
    pub fn new(title: String, api_url: String, ws_url: String, dark_mode: bool) -> Self {
        Self {
            title,
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_url,
            ws_url,
            dark_mode,
        }
    }

    /// Render HTML with context
    pub fn render_html(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>{}</style>
</head>
<body class="{}">
    <div id="app" class="app-container">
        <header class="header">
            <h1>{}</h1>
            <span class="version">v{}</span>
        </header>
        <div class="content">
            <section class="status-panel">
                <h2>Machine Status</h2>
                <div class="status-grid">
                    <div class="status-item">
                        <label>State</label>
                        <span id="state" class="state idle">Disconnected</span>
                    </div>
                    <div class="status-item">
                        <label>Position X</label>
                        <span id="posX">0.00</span> mm
                    </div>
                    <div class="status-item">
                        <label>Position Y</label>
                        <span id="posY">0.00</span> mm
                    </div>
                    <div class="status-item">
                        <label>Position Z</label>
                        <span id="posZ">0.00</span> mm
                    </div>
                </div>
            </section>
            <section class="control-panel">
                <h2>Controls</h2>
                <div class="jog-panel">
                    <h3>Jog</h3>
                    <div class="axis-controls">
                        <button class="jog-btn" data-axis="X" data-distance="-1">X-</button>
                        <button class="jog-btn" data-axis="X" data-distance="1">X+</button>
                        <button class="jog-btn" data-axis="Y" data-distance="-1">Y-</button>
                        <button class="jog-btn" data-axis="Y" data-distance="1">Y+</button>
                        <button class="jog-btn" data-axis="Z" data-distance="-1">Z-</button>
                        <button class="jog-btn" data-axis="Z" data-distance="1">Z+</button>
                    </div>
                </div>
                <div class="emergency-panel">
                    <button id="emergencyStop" class="emergency-btn">Emergency Stop</button>
                </div>
            </section>
        </div>
        <footer class="footer">
            <span id="connection-status" class="connection disconnected">Disconnected</span>
            <span id="message-count" class="message-count">0 messages</span>
        </footer>
    </div>
    <script>
    const config = {{
        apiUrl: "{}",
        wsUrl: "{}",
        darkMode: {}
    }};
    {}
    </script>
</body>
</html>"#,
            self.title,
            PENDANT_UI_CSS,
            if self.dark_mode { "dark-mode" } else { "" },
            self.title,
            self.version,
            self.api_url,
            self.ws_url,
            self.dark_mode,
            PENDANT_UI_JS
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_context_default() {
        let ctx = UiContext::default();
        assert_eq!(ctx.title, "GCodeKit2 Web Pendant");
        assert!(!ctx.dark_mode);
    }

    #[test]
    fn test_ui_context_custom() {
        let ctx = UiContext::new(
            "Custom Pendant".to_string(),
            "/api/v1".to_string(),
            "ws://example.com/ws".to_string(),
            true,
        );
        assert_eq!(ctx.title, "Custom Pendant");
        assert_eq!(ctx.api_url, "/api/v1");
        assert!(ctx.dark_mode);
    }

    #[test]
    fn test_ui_context_render() {
        let ctx = UiContext::default();
        let html = ctx.render_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("GCodeKit2 Web Pendant"));
        assert!(html.contains("Machine Status"));
    }

    #[test]
    fn test_ui_context_render_dark_mode() {
        let ctx = UiContext {
            dark_mode: true,
            ..Default::default()
        };
        let html = ctx.render_html();
        assert!(html.contains("dark-mode"));
    }
}
