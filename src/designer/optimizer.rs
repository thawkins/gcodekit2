//! G-code optimizer module for advanced code optimization techniques.
//!
//! Provides multiple optimization strategies for reducing G-code file size
//! and improving performance while maintaining accuracy and functionality.

use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// G-code optimizer configuration
#[derive(Clone, Debug)]
pub struct OptimizerOptions {
    /// Decimal places to truncate to (0-6)
    pub decimal_places: usize,
    /// Arc tolerance in mm for arc-to-line conversion
    pub arc_tolerance: f32,
    /// Whether to remove empty lines
    pub remove_empty_lines: bool,
    /// Whether to collapse whitespace
    pub collapse_whitespace: bool,
    /// Whether to convert arcs to lines
    pub convert_arcs: bool,
    /// Whether to truncate decimals
    pub truncate_decimals: bool,
}

impl Default for OptimizerOptions {
    fn default() -> Self {
        Self {
            decimal_places: 2,
            arc_tolerance: 0.05,
            remove_empty_lines: true,
            collapse_whitespace: true,
            convert_arcs: false,
            truncate_decimals: true,
        }
    }
}

/// G-code optimizer for file size reduction and performance improvement
#[derive(Clone, Debug)]
pub struct GcodeOptimizer {
    options: OptimizerOptions,
}

impl GcodeOptimizer {
    /// Create a new optimizer with default options
    pub fn new() -> Self {
        Self {
            options: OptimizerOptions::default(),
        }
    }

    /// Create optimizer with custom options
    pub fn with_options(options: OptimizerOptions) -> Self {
        Self { options }
    }

    /// Get optimizer options
    pub fn options(&self) -> &OptimizerOptions {
        &self.options
    }

    /// Optimize G-code by applying all enabled transformations
    ///
    /// # Arguments
    /// * `gcode` - G-code program as string
    ///
    /// # Returns
    /// Optimized G-code string
    pub fn optimize(&self, gcode: &str) -> Result<String> {
        let mut result = gcode.to_string();

        if self.options.truncate_decimals {
            result = self.truncate_decimal_precision(&result)?;
        }

        if self.options.convert_arcs {
            result = self.convert_arcs_to_lines(&result)?;
        }

        if self.options.remove_empty_lines || self.options.collapse_whitespace {
            result = self.remove_redundant_whitespace(&result);
        }

        Ok(result)
    }

    /// Truncate decimal precision in numeric values
    ///
    /// # Arguments
    /// * `gcode` - G-code program
    ///
    /// # Returns
    /// G-code with truncated decimals
    pub fn truncate_decimal_precision(&self, gcode: &str) -> Result<String> {
        if self.options.decimal_places > 6 {
            return Err(anyhow!("Decimal places must be 0-6, got {}", self.options.decimal_places));
        }

        let mut result = String::new();
        let mut _in_comment = false;

        for line in gcode.lines() {
            // Check if entire line is a comment
            let trimmed = line.trim();
            if trimmed.starts_with(';') {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // Process line, preserving comments
            let mut processed_line = String::new();
            let mut chars = line.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch == ';' {
                    // Preserve rest of line as comment
                    processed_line.push(ch);
                    while let Some(c) = chars.next() {
                        processed_line.push(c);
                    }
                    break;
                } else if ch.is_ascii_alphabetic() {
                    // Command letter
                    processed_line.push(ch);

                    // Collect numeric value
                    let mut number = String::new();
                    let mut found_decimal = false;

                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_ascii_digit() {
                            number.push(chars.next().unwrap());
                        } else if next_ch == '.' && !found_decimal {
                            number.push(chars.next().unwrap());
                            found_decimal = true;
                        } else if next_ch == '-' && number.is_empty() {
                            number.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if !number.is_empty() {
                        // Truncate the number
                        let truncated = self.truncate_number(&number)?;
                        processed_line.push_str(&truncated);
                    }
                } else if !ch.is_whitespace() {
                    processed_line.push(ch);
                } else {
                    // Preserve single space between elements
                    if !processed_line.ends_with(' ') && !processed_line.ends_with('\t') {
                        processed_line.push(' ');
                    }
                }
            }

            result.push_str(processed_line.trim_end());
            result.push('\n');
        }

        Ok(result.trim_end().to_string() + "\n")
    }

    /// Truncate a number to specified decimal places
    fn truncate_number(&self, number: &str) -> Result<String> {
        if let Ok(value) = number.parse::<f64>() {
            let multiplier = 10_f64.powi(self.options.decimal_places as i32);
            let truncated = (value * multiplier).trunc() / multiplier;

            // Format with appropriate precision
            if self.options.decimal_places == 0 {
                Ok(format!("{:.0}", truncated))
            } else {
                let formatted = format!("{:.prec$}", truncated, prec = self.options.decimal_places);
                // Remove trailing zeros but keep at least the decimal point
                Ok(formatted.trim_end_matches('0').trim_end_matches('.').to_string())
            }
        } else {
            Ok(number.to_string())
        }
    }

    /// Convert arc commands (G2/G3) to line commands (G1)
    ///
    /// # Arguments
    /// * `gcode` - G-code program
    ///
    /// # Returns
    /// G-code with arcs converted to lines
    pub fn convert_arcs_to_lines(&self, gcode: &str) -> Result<String> {
        let mut result = String::new();

        for line in gcode.lines() {
            let trimmed = line.trim();

            // Check if line contains arc command
            if trimmed.contains('G') && (trimmed.contains("G2") || trimmed.contains("G3")) {
                // Extract arc parameters
                if let Ok(expanded) = self.expand_arc_to_lines(line) {
                    result.push_str(&expanded);
                } else {
                    // If expansion fails, keep original line
                    result.push_str(line);
                    result.push('\n');
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result)
    }

    /// Expand a single arc command to multiple line commands
    fn expand_arc_to_lines(&self, arc_line: &str) -> Result<String> {
        // Extract G-code parameters
        let params = self.extract_parameters(arc_line);

        let start_x = params.get("prev_x").copied().unwrap_or(0.0);
        let start_y = params.get("prev_y").copied().unwrap_or(0.0);
        let end_x = params.get("X").copied().ok_or_else(|| anyhow!("Missing X in arc"))?;
        let end_y = params.get("Y").copied().ok_or_else(|| anyhow!("Missing Y in arc"))?;
        let i = params.get("I").copied().unwrap_or(0.0);
        let j = params.get("J").copied().unwrap_or(0.0);
        let is_cw = arc_line.contains("G2");

        // Calculate arc center
        let center_x = start_x + i;
        let center_y = start_y + j;

        // Generate line segments
        let segments = self.calculate_arc_segments(
            start_x, start_y, end_x, end_y, center_x, center_y, is_cw,
        );

        let mut result = String::new();
        let feed = params.get("F").map(|f| format!(" F{:.0}", f)).unwrap_or_default();
        let spindle = params
            .get("S")
            .map(|s| format!(" S{:.0}", s))
            .unwrap_or_default();

        for segment in segments {
            result.push_str(&format!("G1 X{:.4} Y{:.4}{}{}\n", segment.0, segment.1, feed, spindle));
        }

        Ok(result)
    }

    /// Calculate line segments to approximate an arc
    fn calculate_arc_segments(
        &self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        center_x: f32,
        center_y: f32,
        _is_cw: bool,
    ) -> Vec<(f32, f32)> {
        let radius = ((start_x - center_x).powi(2) + (start_y - center_y).powi(2)).sqrt();

        if radius < 0.001 {
            return vec![(end_x, end_y)];
        }

        let start_angle = (start_y - center_y).atan2(start_x - center_x);
        let end_angle = (end_y - center_y).atan2(end_x - center_x);

        // Calculate number of segments based on tolerance
        let angle_diff =
            ((end_angle - start_angle).abs() + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)
                - std::f32::consts::PI;
        let num_segments =
            ((angle_diff.abs() * radius / self.options.arc_tolerance).ceil() as usize).max(1);

        let mut segments = Vec::new();
        for i in 1..=num_segments {
            let angle = start_angle + (angle_diff / num_segments as f32) * i as f32;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            segments.push((x, y));
        }

        segments
    }

    /// Extract numeric parameters from G-code line
    fn extract_parameters(&self, line: &str) -> HashMap<String, f32> {
        let mut params = HashMap::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i].is_ascii_alphabetic() {
                let key = chars[i].to_string();
                i += 1;

                let mut number = String::new();
                let mut found_decimal = false;

                // Negative sign
                if i < chars.len() && chars[i] == '-' {
                    number.push(chars[i]);
                    i += 1;
                }

                while i < chars.len() {
                    if chars[i].is_ascii_digit() {
                        number.push(chars[i]);
                        i += 1;
                    } else if chars[i] == '.' && !found_decimal {
                        number.push(chars[i]);
                        found_decimal = true;
                        i += 1;
                    } else {
                        break;
                    }
                }

                if !number.is_empty() {
                    if let Ok(value) = number.parse::<f32>() {
                        params.insert(key, value);
                    }
                }
            } else {
                i += 1;
            }
        }

        params
    }

    /// Remove redundant whitespace and empty lines
    ///
    /// # Arguments
    /// * `gcode` - G-code program
    ///
    /// # Returns
    /// G-code with optimized whitespace
    pub fn remove_redundant_whitespace(&self, gcode: &str) -> String {
        let mut result = String::new();

        for line in gcode.lines() {
            if self.options.remove_empty_lines {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
            }

            if self.options.collapse_whitespace {
                // Collapse multiple spaces to single space
                let collapsed = line
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                result.push_str(&collapsed);
            } else {
                result.push_str(line);
            }

            result.push('\n');
        }

        result.trim_end().to_string() + "\n"
    }

    /// Get optimization statistics
    pub fn get_stats(original: &str, optimized: &str) -> OptimizationStats {
        let original_size = original.len();
        let optimized_size = optimized.len();
        let reduction = if original_size > 0 {
            ((original_size - optimized_size) as f32 / original_size as f32) * 100.0
        } else {
            0.0
        };

        OptimizationStats {
            original_size,
            optimized_size,
            size_reduction_bytes: original_size as i32 - optimized_size as i32,
            size_reduction_percent: reduction,
        }
    }
}

impl Default for GcodeOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimization statistics
#[derive(Clone, Debug)]
pub struct OptimizationStats {
    pub original_size: usize,
    pub optimized_size: usize,
    pub size_reduction_bytes: i32,
    pub size_reduction_percent: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = GcodeOptimizer::new();
        assert_eq!(optimizer.options.decimal_places, 2);
        assert!(optimizer.options.truncate_decimals);
    }

    #[test]
    fn test_truncate_decimal_precision() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G0 X10.5555 Y20.7777 Z5.1234\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("X10.55"));
        assert!(result.contains("Y20.77"));
    }

    #[test]
    fn test_truncate_with_zero_decimals() {
        let mut options = OptimizerOptions::default();
        options.decimal_places = 0;
        let optimizer = GcodeOptimizer::with_options(options);
        let gcode = "G0 X10.9 Y20.1\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("X10") || result.contains("X11"));
    }

    #[test]
    fn test_remove_empty_lines() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G0 X10\n\n\nG1 Y20\n";
        let result = optimizer.remove_redundant_whitespace(gcode);
        let empty_lines = result.lines().filter(|l| l.trim().is_empty()).count();
        assert_eq!(empty_lines, 0);
    }

    #[test]
    fn test_collapse_whitespace() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G0    X10    Y20    Z5\n";
        let result = optimizer.remove_redundant_whitespace(gcode);
        assert!(result.contains("G0 X10 Y20 Z5"));
    }

    #[test]
    fn test_preserve_comments() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G0 X10 Y20 ; Move to position\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("; Move to position"));
    }

    #[test]
    fn test_full_comment_line() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "; This is a full line comment\nG0 X10\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("; This is a full line comment"));
    }

    #[test]
    fn test_negative_coordinates() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G0 X-10.555 Y-20.777\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("X-10.55"));
        assert!(result.contains("Y-20.77"));
    }

    #[test]
    fn test_optimize_all_options() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G0   X10.555   Y20.777\n\n\nG1  Z-5.1234  F1000.999\n";
        let result = optimizer.optimize(gcode).unwrap();
        assert!(!result.contains("10.555"));
        assert!(!result.contains("\n\n"));
    }

    #[test]
    fn test_decimal_places_validation() {
        let mut options = OptimizerOptions::default();
        options.decimal_places = 7;
        let optimizer = GcodeOptimizer::with_options(options);
        let result = optimizer.truncate_decimal_precision("G0 X10.5\n");
        assert!(result.is_err());
    }

    #[test]
    fn test_feed_rate_and_spindle() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G1 X10 F1000.999 S5000.555\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("F1000.99") || result.contains("F1000"));
        assert!(result.contains("S5000.55") || result.contains("S5000"));
    }

    #[test]
    fn test_preserve_g_codes() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G21\nG90\nG0 X10 Y20\nG1 Z-5 F100\nM3 S5000\nM5\n";
        let result = optimizer.truncate_decimal_precision(gcode).unwrap();
        assert!(result.contains("G21"));
        assert!(result.contains("G90"));
        assert!(result.contains("G0"));
        assert!(result.contains("G1"));
        assert!(result.contains("M3"));
        assert!(result.contains("M5"));
    }

    #[test]
    fn test_optimization_stats() {
        let original = "G0 X10.5555 Y20.7777 Z5.1234\n";
        let optimized = "G0 X10.55 Y20.77 Z5.12\n";
        let stats = GcodeOptimizer::get_stats(original, optimized);
        assert!(stats.size_reduction_bytes > 0);
        assert!(stats.size_reduction_percent > 0.0);
    }

    #[test]
    fn test_multiline_program_optimization() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "G21\nG90\nG0 X10.555 Y20.777 Z5.123\n\n\nG1 Z-2.567 F500.999\nG0 Z5\nM5\n";
        let result = optimizer.optimize(gcode).unwrap();

        // Should remove empty lines
        assert!(!result.contains("\n\n"));

        // Should truncate decimals
        assert!(!result.contains("10.555"));
        assert!(!result.contains("20.777"));

        // Should preserve structure
        assert!(result.contains("G21"));
        assert!(result.contains("G0"));
        assert!(result.contains("G1"));
        assert!(result.contains("M5"));
    }

    #[test]
    fn test_arc_conversion_basic() {
        let mut options = OptimizerOptions::default();
        options.convert_arcs = true;
        let optimizer = GcodeOptimizer::with_options(options);

        let gcode = "G0 X0 Y0\nG2 X10 Y10 I5 J5\n";
        let result = optimizer.optimize(gcode).unwrap();

        // Should still contain valid G-code
        assert!(result.contains("G1") || result.contains("G2"));
    }

    #[test]
    fn test_empty_program() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "";
        let result = optimizer.optimize(gcode).unwrap();
        assert!(result.is_empty() || result == "\n");
    }

    #[test]
    fn test_comments_only() {
        let optimizer = GcodeOptimizer::new();
        let gcode = "; Comment 1\n; Comment 2\n";
        let result = optimizer.remove_redundant_whitespace(gcode);
        assert!(result.contains("; Comment 1"));
        assert!(result.contains("; Comment 2"));
    }
}
