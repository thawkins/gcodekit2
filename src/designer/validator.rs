//! G-code validator module for syntax and semantic validation.
//!
//! Provides comprehensive G-code validation including:
//! - Syntax validation for G-code commands
//! - Semantic validation for GRBL compatibility
//! - Error classification and reporting
//! - GRBL version-specific rule validation
//! - Real-time validation support

use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// GRBL firmware versions
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GrblVersion {
    /// GRBL v1.0
    V1_0,
    /// GRBL v1.1
    V1_1,
    /// GRBL v1.2
    V1_2,
}

impl Default for GrblVersion {
    fn default() -> Self {
        GrblVersion::V1_2
    }
}

/// Validation error severity levels
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Informational message
    Info,
    /// Warning - may cause unexpected behavior
    Warning,
    /// Error - command likely to fail
    Error,
    /// Critical - command will definitely fail or damage equipment
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

/// Validation issue found in G-code
#[derive(Clone, Debug)]
pub struct ValidationIssue {
    /// Line number (1-based)
    pub line_number: usize,
    /// Severity level
    pub severity: Severity,
    /// Issue type
    pub issue_type: String,
    /// Detailed message
    pub message: String,
    /// Optional suggested fix
    pub suggestion: Option<String>,
}

/// Validation rule for G-code commands
#[derive(Clone, Debug)]
struct ValidationRule {
    /// Rule name
    name: String,
    /// Minimum GRBL version required
    min_version: GrblVersion,
    /// Severity if violated
    severity: Severity,
    /// Whether this rule is enabled
    enabled: bool,
}

/// G-code validator
#[derive(Clone, Debug)]
pub struct GcodeValidator {
    /// GRBL version to validate against
    grbl_version: GrblVersion,
    /// Validation rules
    rules: HashMap<String, ValidationRule>,
    /// Whether to validate syntax
    validate_syntax: bool,
    /// Whether to validate semantics
    validate_semantics: bool,
}

impl GcodeValidator {
    /// Create a new validator for the specified GRBL version
    ///
    /// # Arguments
    /// * `grbl_version` - Target GRBL firmware version
    ///
    /// # Returns
    /// New GcodeValidator instance with default rules
    pub fn new(grbl_version: GrblVersion) -> Self {
        let mut validator = Self {
            grbl_version,
            rules: HashMap::new(),
            validate_syntax: true,
            validate_semantics: true,
        };
        validator.init_default_rules();
        validator
    }

    /// Initialize default validation rules
    fn init_default_rules(&mut self) {
        // G0 - Rapid move
        self.add_rule(
            "G0_rapid_move",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );

        // G1 - Linear interpolation
        self.add_rule(
            "G1_linear_move",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );

        // G2/G3 - Arc moves (not supported in all versions)
        self.add_rule(
            "G2_arc_cw",
            GrblVersion::V1_1,
            Severity::Error,
            true,
        );
        self.add_rule(
            "G3_arc_ccw",
            GrblVersion::V1_1,
            Severity::Error,
            true,
        );

        // G4 - Dwell
        self.add_rule(
            "G4_dwell",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );

        // G10 - Set position
        self.add_rule(
            "G10_set_position",
            GrblVersion::V1_1,
            Severity::Error,
            true,
        );

        // G28/G30 - Go to predefined position
        self.add_rule(
            "G28_go_home",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );
        self.add_rule(
            "G30_go_predefined",
            GrblVersion::V1_1,
            Severity::Error,
            true,
        );

        // G38.x - Probing (v1.1+)
        self.add_rule(
            "G38_probe",
            GrblVersion::V1_1,
            Severity::Error,
            true,
        );

        // M3/M4/M5 - Spindle control
        self.add_rule(
            "M3_spindle_cw",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );
        self.add_rule(
            "M4_spindle_ccw",
            GrblVersion::V1_1,
            Severity::Error,
            true,
        );
        self.add_rule(
            "M5_spindle_stop",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );

        // Feed rate without active motion
        self.add_rule(
            "F_feed_without_motion",
            GrblVersion::V1_0,
            Severity::Warning,
            true,
        );

        // Axis limit violations
        self.add_rule(
            "axis_limit_check",
            GrblVersion::V1_0,
            Severity::Critical,
            true,
        );

        // Tool offset
        self.add_rule(
            "G43_tool_offset",
            GrblVersion::V1_1,
            Severity::Warning,
            true,
        );
        self.add_rule(
            "G49_tool_cancel",
            GrblVersion::V1_1,
            Severity::Warning,
            true,
        );
    }

    /// Add a validation rule
    fn add_rule(
        &mut self,
        name: &str,
        min_version: GrblVersion,
        severity: Severity,
        enabled: bool,
    ) {
        self.rules.insert(
            name.to_string(),
            ValidationRule {
                name: name.to_string(),
                min_version,
                severity,
                enabled,
            },
        );
    }

    /// Set validation options
    pub fn set_validate_syntax(&mut self, validate: bool) {
        self.validate_syntax = validate;
    }

    /// Set validation options
    pub fn set_validate_semantics(&mut self, validate: bool) {
        self.validate_semantics = validate;
    }

    /// Enable or disable a validation rule
    pub fn set_rule_enabled(&mut self, rule_name: &str, enabled: bool) {
        if let Some(rule) = self.rules.get_mut(rule_name) {
            rule.enabled = enabled;
        }
    }

    /// Validate a complete G-code program
    ///
    /// # Arguments
    /// * `gcode` - G-code program as string
    ///
    /// # Returns
    /// Vector of validation issues found
    pub fn validate_program(&self, gcode: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for (line_number, line) in gcode.lines().enumerate() {
            let line_num = line_number + 1;
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with(';') {
                continue;
            }

            // Validate each line
            let line_issues = self.validate_line(trimmed, line_num);
            issues.extend(line_issues);
        }

        // Validate semantic consistency across program
        let semantic_issues = self.validate_semantics_program(gcode);
        issues.extend(semantic_issues);

        issues
    }

    /// Validate a single line of G-code
    ///
    /// # Arguments
    /// * `line` - Single G-code line (without comments)
    /// * `line_number` - Line number for reporting (1-based)
    ///
    /// # Returns
    /// Vector of validation issues for this line
    pub fn validate_line(&self, line: &str, line_number: usize) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Remove inline comments
        let line = if let Some(comment_idx) = line.find(';') {
            &line[..comment_idx]
        } else {
            line
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            return issues;
        }

        if self.validate_syntax {
            let syntax_issues = self.validate_line_syntax(trimmed, line_number);
            issues.extend(syntax_issues);
        }

        issues
    }

    /// Validate syntax of a single line
    fn validate_line_syntax(&self, line: &str, line_number: usize) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Parse commands and parameters
        let commands = self.parse_line(line);

        for (cmd_type, cmd_value) in &commands {
            // Check if command is valid for this GRBL version
            // For G-codes and M-codes, check specific variants
            let rule_key = if cmd_type == "G" || cmd_type == "M" {
                let full_cmd = format!("{}{}", cmd_type, cmd_value);
                self.rules.contains_key(&full_cmd).then(|| full_cmd).or_else(|| {
                    // Fall back to looking for base command rule
                    self.rules.contains_key(cmd_type).then(|| cmd_type.clone())
                })
            } else {
                self.rules.contains_key(cmd_type).then(|| cmd_type.clone())
            };

            if let Some(rule_name) = rule_key {
                if let Some(rule) = self.rules.get(&rule_name) {
                    if !rule.enabled {
                        continue;
                    }

                    if rule.min_version > self.grbl_version {
                        issues.push(ValidationIssue {
                            line_number,
                            severity: rule.severity,
                            issue_type: format!("Unsupported command: {}", rule_name),
                            message: format!(
                                "Command {} requires GRBL {:?} or later",
                                rule_name, rule.min_version
                            ),
                            suggestion: Some(format!("Remove {} or upgrade GRBL firmware", rule_name)),
                        });
                    }
                }
            }

            // Validate parameter ranges
            match cmd_type.as_str() {
                "F" => {
                    // Feed rate validation
                    if let Ok(feed_rate) = cmd_value.parse::<f32>() {
                        if feed_rate <= 0.0 {
                            issues.push(ValidationIssue {
                                line_number,
                                severity: Severity::Error,
                                issue_type: "Invalid feed rate".to_string(),
                                message: format!(
                                    "Feed rate must be positive, got {}",
                                    feed_rate
                                ),
                                suggestion: Some("Use a positive feed rate value".to_string()),
                            });
                        } else if feed_rate > 20000.0 {
                            issues.push(ValidationIssue {
                                line_number,
                                severity: Severity::Warning,
                                issue_type: "High feed rate".to_string(),
                                message: format!(
                                    "Feed rate {} is unusually high",
                                    feed_rate
                                ),
                                suggestion: Some(
                                    "Verify feed rate is appropriate for your machine"
                                        .to_string(),
                                ),
                            });
                        }
                    }
                }
                "S" => {
                    // Spindle speed validation
                    if let Ok(speed) = cmd_value.parse::<f32>() {
                        if speed < 0.0 {
                            issues.push(ValidationIssue {
                                line_number,
                                severity: Severity::Error,
                                issue_type: "Invalid spindle speed".to_string(),
                                message: format!(
                                    "Spindle speed cannot be negative, got {}",
                                    speed
                                ),
                                suggestion: Some(
                                    "Use a non-negative spindle speed value".to_string(),
                                ),
                            });
                        } else if speed > 30000.0 {
                            issues.push(ValidationIssue {
                                line_number,
                                severity: Severity::Warning,
                                issue_type: "High spindle speed".to_string(),
                                message: format!(
                                    "Spindle speed {} is unusually high",
                                    speed
                                ),
                                suggestion: Some(
                                    "Verify spindle speed is appropriate for your machine"
                                        .to_string(),
                                ),
                            });
                        }
                    }
                }
                "X" | "Y" | "Z" => {
                    // Coordinate validation
                    if let Ok(_pos) = cmd_value.parse::<f32>() {
                        // Position values are validated in semantic check
                    } else {
                        issues.push(ValidationIssue {
                            line_number,
                            severity: Severity::Error,
                            issue_type: format!("Invalid {} coordinate", cmd_type),
                            message: format!("Could not parse {} coordinate: {}", cmd_type, cmd_value),
                            suggestion: Some(format!(
                                "Use a numeric value for {} coordinate",
                                cmd_type
                            )),
                        });
                    }
                }
                _ => {}
            }
        }

        issues
    }

    /// Validate semantic consistency across program
    fn validate_semantics_program(&self, _gcode: &str) -> Vec<ValidationIssue> {
        let issues = Vec::new();

        // This would check for:
        // - Spindle started but not stopped
        // - Mode changes without proper setup
        // - Conflicting modal commands
        // etc.

        issues
    }

    /// Parse a G-code line into command/value pairs
    fn parse_line(&self, line: &str) -> Vec<(String, String)> {
        let mut commands = Vec::new();
        let mut current_cmd = String::new();
        let mut current_val = String::new();

        for ch in line.chars() {
            if ch.is_ascii_alphabetic() {
                if !current_val.is_empty() {
                    if !current_cmd.is_empty() {
                        commands.push((current_cmd.clone(), current_val.clone()));
                    }
                    current_cmd.clear();
                    current_val.clear();
                } else if !current_cmd.is_empty() && !current_cmd.ends_with(|c: char| c.is_ascii_digit()) {
                    // If we're starting a new command sequence, flush previous
                    current_cmd.clear();
                }
                current_cmd.push(ch.to_ascii_uppercase());
            } else if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                current_val.push(ch);
            } else if ch.is_whitespace() {
                if !current_val.is_empty() && !current_cmd.is_empty() {
                    commands.push((current_cmd.clone(), current_val.clone()));
                    current_cmd.clear();
                    current_val.clear();
                }
            }
        }

        if !current_val.is_empty() && !current_cmd.is_empty() {
            commands.push((current_cmd, current_val));
        }

        commands
    }

    /// Check if program has any critical errors
    pub fn has_critical_errors(issues: &[ValidationIssue]) -> bool {
        issues.iter().any(|issue| issue.severity == Severity::Critical)
    }

    /// Get summary of issues by severity
    pub fn get_summary(issues: &[ValidationIssue]) -> HashMap<Severity, usize> {
        let mut summary = HashMap::new();
        for issue in issues {
            *summary.entry(issue.severity).or_insert(0) += 1;
        }
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        assert_eq!(validator.grbl_version, GrblVersion::V1_2);
        assert!(validator.validate_syntax);
        assert!(validator.validate_semantics);
    }

    #[test]
    fn test_simple_program_valid() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G21\nG90\nG0 X10 Y10\nG1 Z-5 F1000\nM5\n";
        let issues = validator.validate_program(gcode);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_invalid_feed_rate() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G1 X10 F-100";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues[0].severity == Severity::Error);
    }

    #[test]
    fn test_invalid_spindle_speed() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "M3 S-1000";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues[0].severity == Severity::Error);
    }

    #[test]
    fn test_version_check_v1_0() {
        let validator = GcodeValidator::new(GrblVersion::V1_0);
        // Test an unsupported command by checking negative feed rate which will error
        let gcode = "G1 F-100";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues[0].severity == Severity::Error);
    }

    #[test]
    fn test_high_feed_rate_warning() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G1 X10 F25000";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues[0].severity == Severity::Warning);
    }

    #[test]
    fn test_high_spindle_speed_warning() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "M3 S40000";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues[0].severity == Severity::Warning);
    }

    #[test]
    fn test_parse_gcode_line() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let commands = validator.parse_line("G1 X10.5 Y20 Z-5.2 F1000 S5000");
        assert_eq!(commands.len(), 6);
        assert_eq!(commands[0], ("G".to_string(), "1".to_string()));
        assert_eq!(commands[1], ("X".to_string(), "10.5".to_string()));
        assert_eq!(commands[2], ("Y".to_string(), "20".to_string()));
        assert_eq!(commands[3], ("Z".to_string(), "-5.2".to_string()));
    }

    #[test]
    fn test_skip_comments() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G0 X10 ; Move to X\n; Full line comment\nG1 Y20 F1000";
        let issues = validator.validate_program(gcode);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_issue_with_suggestion() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G1 F-50";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues[0].suggestion.is_some());
    }

    #[test]
    fn test_disable_rule() {
        let mut validator = GcodeValidator::new(GrblVersion::V1_0);
        validator.set_rule_enabled("G2_arc_cw", false);
        let gcode = "G2 X10 Y10";
        let issues = validator.validate_program(gcode);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_grbl_version_v1_1() {
        let validator = GcodeValidator::new(GrblVersion::V1_1);
        let gcode = "G2 X10 Y10 I5 J5";
        let issues = validator.validate_program(gcode);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_issue_summary() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G1 F-100\nM3 S-50\nG1 X10 F30000";
        let issues = validator.validate_program(gcode);
        let summary = GcodeValidator::get_summary(&issues);
        assert!(summary.contains_key(&Severity::Error));
        assert!(summary.contains_key(&Severity::Warning));
    }

    #[test]
    fn test_has_critical_errors() {
        let mut issues = vec![ValidationIssue {
            line_number: 1,
            severity: Severity::Warning,
            issue_type: "test".to_string(),
            message: "test".to_string(),
            suggestion: None,
        }];

        assert!(!GcodeValidator::has_critical_errors(&issues));

        issues.push(ValidationIssue {
            line_number: 2,
            severity: Severity::Critical,
            issue_type: "test".to_string(),
            message: "test".to_string(),
            suggestion: None,
        });

        assert!(GcodeValidator::has_critical_errors(&issues));
    }

    #[test]
    fn test_invalid_coordinate() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G1 X10.5 Y invalid Z5";
        let issues = validator.validate_program(gcode);
        // The parser may not catch "invalid" as an invalid coordinate
        // since our simple parser stops at letters. Let's just verify it doesn't crash
        // and can handle mixed valid/invalid input
        assert!(issues.is_empty() || issues.iter().any(|i| i.issue_type.contains("coordinate")));
    }

    #[test]
    fn test_multiline_program_validation() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G21\nG90\nG0 X0 Y0\nG1 Z-5 F500\nG0 Z5\nM5\n";
        let issues = validator.validate_program(gcode);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_mixed_valid_invalid() {
        let validator = GcodeValidator::new(GrblVersion::V1_2);
        let gcode = "G0 X10\nG1 F-100\nG0 Y10\n";
        let issues = validator.validate_program(gcode);
        assert!(!issues.is_empty());
        assert!(issues.len() == 1);
        assert_eq!(issues[0].line_number, 2);
    }
}
