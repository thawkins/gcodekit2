//! GRBL protocol implementation

use serde::{Deserialize, Serialize};

/// GRBL version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub major: u32,
    pub minor: u32,
    pub patch: char,
}

impl VersionInfo {
    /// Parse GRBL version from response string
    pub fn parse(response: &str) -> Option<Self> {
        // Expected format: "GRBL v1.1h"
        let parts: Vec<&str> = response.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "GRBL" {
            let version_str = parts[1].trim_start_matches('v');
            let mut version_parts = version_str.split('.');
            
            if let (Some(major_str), Some(minor_str)) = (version_parts.next(), version_parts.next()) {
                if let (Ok(major), Some(minor_char)) = (major_str.parse::<u32>(), minor_str.chars().next()) {
                    let (minor, patch) = if minor_char.is_numeric() {
                        let rest: String = minor_str.chars().skip(1).collect();
                        if let Ok(m) = minor_char.to_digit(10).ok_or(()).and_then(|d| Ok(d as u32)) {
                            (m, rest.chars().next().unwrap_or('0'))
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    };
                    
                    return Some(VersionInfo { major, minor, patch });
                }
            }
        }
        None
    }
}
