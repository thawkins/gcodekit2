//! Console Logging - Captures tracing output for display in Device Console
//!
//! Redirects structured logging from tracing to the device console instead of stdout/stderr

use std::sync::{Arc, Mutex};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// Maximum number of lines to keep in console buffer
const MAX_CONSOLE_LINES: usize = 1000;

/// Shared console output buffer for storing log messages
pub type ConsoleBuffer = Arc<Mutex<Vec<String>>>;

/// Initialize console logging layer
pub fn init_console_logging() -> ConsoleBuffer {
    let console_buffer = Arc::new(Mutex::new(Vec::new()));
    
    // Create a custom layer that writes to our buffer
    let console_buffer_clone = console_buffer.clone();
    
    let custom_layer = tracing_subscriber::fmt::layer()
        .with_writer(move || {
            let buffer = console_buffer_clone.clone();
            ConsoleWriter { buffer }
        })
        .with_target(true)
        .with_level(true)
        .with_ansi(false)
        .compact();
    
    let env_filter = EnvFilter::new("gcodekit2=trace");
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(custom_layer)
        .init();
    
    console_buffer
}

/// Custom writer that redirects output to our console buffer
struct ConsoleWriter {
    buffer: ConsoleBuffer,
}

impl std::io::Write for ConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(text) = std::str::from_utf8(buf) {
            if let Ok(mut buffer) = self.buffer.lock() {
                // Split into lines and add each one
                for line in text.lines() {
                    if !line.is_empty() {
                        buffer.push(line.to_string());
                        
                        // Enforce max lines limit
                        if buffer.len() > MAX_CONSOLE_LINES {
                            buffer.remove(0);
                        }
                    }
                }
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Get current console logs
pub fn get_console_logs(buffer: &ConsoleBuffer) -> Vec<String> {
    buffer
        .lock()
        .ok()
        .map(|b| b.clone())
        .unwrap_or_default()
}

/// Clear console logs
pub fn clear_console_logs(buffer: &ConsoleBuffer) {
    if let Ok(mut b) = buffer.lock() {
        b.clear();
    }
}

/// Add a message to console
pub fn add_console_message(buffer: &ConsoleBuffer, message: String) {
    if let Ok(mut b) = buffer.lock() {
        b.push(message);
        
        // Enforce max lines limit
        if b.len() > MAX_CONSOLE_LINES {
            b.remove(0);
        }
    }
}

/// Filter console logs by level and content
pub fn filter_console_logs(
    buffer: &ConsoleBuffer,
    show_info: bool,
    show_debug: bool,
    show_warn: bool,
    show_error: bool,
    show_trace: bool,
    show_other: bool,
) -> Vec<String> {
    get_console_logs(buffer)
        .into_iter()
        .map(|line| strip_ansi_codes(&line))
        .filter(|line| {
            if line.contains(" INFO ") || line.contains("[INFO]") {
                show_info
            } else if line.contains(" DEBUG ") || line.contains("[DEBUG]") {
                show_debug
            } else if line.contains(" WARN ") || line.contains("[WARN]") {
                show_warn
            } else if line.contains(" ERROR ") || line.contains("[ERROR]") {
                show_error
            } else if line.contains(" TRACE ") || line.contains("[TRACE]") {
                show_trace
            } else {
                // Show lines that don't match any tracing level
                show_other
            }
        })
        .collect()
}

/// Strip ANSI escape codes from a string
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// Get console logs as a single string for export
pub fn get_console_as_string(buffer: &ConsoleBuffer) -> String {
    get_console_logs(buffer)
        .into_iter()
        .map(|line| strip_ansi_codes(&line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Extract tracing level from log line
pub fn extract_level(line: &str) -> Option<&str> {
    if line.contains(" INFO ") || line.contains("[INFO]") {
        Some("INFO")
    } else if line.contains(" DEBUG ") || line.contains("[DEBUG]") {
        Some("DEBUG")
    } else if line.contains(" WARN ") || line.contains("[WARN]") {
        Some("WARN")
    } else if line.contains(" ERROR ") || line.contains("[ERROR]") {
        Some("ERROR")
    } else if line.contains(" TRACE ") || line.contains("[TRACE]") {
        Some("TRACE")
    } else {
        None
    }
}

/// Format log line with tracing level at the start
pub fn format_log_line(line: &str) -> String {
    if let Some(level) = extract_level(line) {
        // Extract the level indicator and message
        let level_format = if line.contains(" INFO ") {
            " INFO "
        } else if line.contains(" DEBUG ") {
            " DEBUG "
        } else if line.contains(" WARN ") {
            " WARN "
        } else if line.contains(" ERROR ") {
            " ERROR "
        } else if line.contains(" TRACE ") {
            " TRACE "
        } else if line.contains("[INFO]") {
            "[INFO]"
        } else if line.contains("[DEBUG]") {
            "[DEBUG]"
        } else if line.contains("[WARN]") {
            "[WARN]"
        } else if line.contains("[ERROR]") {
            "[ERROR]"
        } else {
            "[TRACE]"
        };
        
        // Find the message after the level indicator
        if let Some(pos) = line.find(level_format) {
            let msg_start = pos + level_format.len();
            let message = &line[msg_start..].trim_start();
            format!("{}: {}", level, message)
        } else {
            line.to_string()
        }
    } else {
        line.to_string()
    }
}
