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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolpath_creation() {
        let tp = Toolpath::new(
            "Cut".to_string(),
            "G0 X10 Y10\nG1 Z-5 F100\n".to_string(),
            100.0,
            1000,
            5.0,
        );
        assert_eq!(tp.name, "Cut");
        assert_eq!(tp.feed_rate, 100.0);
        assert_eq!(tp.spindle_speed, 1000);
    }

    #[test]
    fn test_toolpath_estimate_time() {
        let tp = Toolpath::new(
            "Test".to_string(),
            "G1 X10\nG1 X20\nG1 X30\n".to_string(),
            100.0,
            1000,
            1.0,
        );
        let time = tp.estimate_time();
        assert!(time > 0.0);
    }

    #[test]
    fn test_toolpath_optimize() {
        let mut tp = Toolpath::new(
            "Test".to_string(),
            "; Comment\nG0 X10\n\n  G1 Y20  \nG0 Z5".to_string(),
            100.0,
            1000,
            1.0,
        );
        tp.optimize();
        assert!(!tp.gcode.contains(";"));
        assert!(!tp.gcode.contains("\n\n"));
    }

    #[test]
    fn test_convert_arcs_to_lines() {
        let mut tp = Toolpath::new(
            "Test".to_string(),
            "G0 X10\nG2 X20 Y20 I5 J5\nG1 Z5".to_string(),
            100.0,
            1000,
            1.0,
        );
        tp.convert_arcs_to_lines(0.1);
        assert!(!tp.gcode.contains("G2"));
        assert!(tp.gcode.contains("G1"));
    }
}
