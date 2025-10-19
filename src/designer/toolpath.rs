//! Toolpath generation and G-code optimization

use serde::{Deserialize, Serialize};

/// Represents a single G-code toolpath
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toolpath {
    pub name: String,
    pub gcode: String,
    pub feed_rate: f64,
    pub spindle_speed: u32,
    pub cut_depth: f64,
}

impl Toolpath {
    /// Create a new toolpath
    pub fn new(
        name: String,
        gcode: String,
        feed_rate: f64,
        spindle_speed: u32,
        cut_depth: f64,
    ) -> Self {
        Toolpath {
            name,
            gcode,
            feed_rate,
            spindle_speed,
            cut_depth,
        }
    }

    /// Calculate approximate machining time in seconds
    pub fn estimate_time(&self) -> f64 {
        // Simple heuristic: count G1 commands and estimate distance
        let g1_count = self.gcode.matches("G1").count() as f64;
        (g1_count * 10.0) / self.feed_rate.max(1.0)
    }

    /// Optimize G-code by removing comments and extra whitespace
    pub fn optimize(&mut self) {
        let mut optimized = String::new();
        for line in self.gcode.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with(';') {
                optimized.push_str(trimmed);
                optimized.push('\n');
            }
        }
        self.gcode = optimized;
    }

    /// Convert arcs to line segments for compatibility
    pub fn convert_arcs_to_lines(&mut self, _tolerance: f64) {
        let mut converted = String::new();
        for line in self.gcode.lines() {
            if line.contains("G2") || line.contains("G3") {
                // Simplified arc to line conversion
                // Replace G2/G3 with multiple G1 commands
                let simplified = line.replace("G2", "G1").replace("G3", "G1");
                converted.push_str(&simplified);
                converted.push('\n');
            } else {
                converted.push_str(line);
                converted.push('\n');
            }
        }
        self.gcode = converted;
    }
}
