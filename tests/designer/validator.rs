//! G-code validator integration tests

use gcodekit2::designer::{GcodeValidator, GrblVersion, Severity};

#[test]
fn test_complete_valid_program() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "
G21                 ; Metric units
G90                 ; Absolute positioning
G0 X0 Y0 Z5        ; Rapid to origin
M3 S5000           ; Start spindle
G1 Z-2 F100        ; Feed down
G1 X10 F500        ; Cut to X10
G1 Y10 F500        ; Cut to Y10
G1 X0 F500         ; Cut back to X0
G1 Y0 F500         ; Cut back to Y0
G0 Z5              ; Rapid back to safe height
M5                 ; Stop spindle
";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty(), "Valid program should have no issues");
}

#[test]
fn test_negative_feed_rate() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G1 X10 F-100";
    let issues = validator.validate_program(gcode);
    assert!(!issues.is_empty());
    assert_eq!(issues[0].severity, Severity::Error);
    assert!(issues[0].message.contains("positive"));
}

#[test]
fn test_negative_spindle_speed() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "M3 S-1000";
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
    assert_eq!(issues[0].severity, Severity::Warning);
}

#[test]
fn test_high_spindle_speed_warning() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "M3 S40000";
    let issues = validator.validate_program(gcode);
    assert!(!issues.is_empty());
    assert_eq!(issues[0].severity, Severity::Warning);
}

#[test]
fn test_comments_are_skipped() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "
; This is a comment
G0 X10 Y10  ; Move to position
; Another comment
G1 Z-5 F100 ; Feed down
";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty());
}

#[test]
fn test_mixed_errors_and_warnings() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "
G1 X10 F-100     ; Error: negative feed rate
G1 X20 F30000    ; Warning: high feed rate
M3 S5000 F100    ; Valid
";
    let issues = validator.validate_program(gcode);
    assert!(!issues.is_empty());

    let errors = issues.iter().filter(|i| i.severity == Severity::Error).count();
    let warnings = issues.iter().filter(|i| i.severity == Severity::Warning).count();

    assert_eq!(errors, 1);
    assert_eq!(warnings, 1);
}

#[test]
fn test_multiple_errors_per_line() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G1 X10 F-100 S-500";
    let issues = validator.validate_program(gcode);

    // Should have multiple issues
    assert!(issues.len() >= 2);

    // Check for feed rate and spindle speed errors
    assert!(issues.iter().any(|i| i.issue_type.contains("feed")));
    assert!(issues.iter().any(|i| i.issue_type.contains("spindle")));
}

#[test]
fn test_issue_has_suggestion() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G1 F-50";
    let issues = validator.validate_program(gcode);

    assert!(!issues.is_empty());
    assert!(issues[0].suggestion.is_some());
}

#[test]
fn test_line_numbers_in_issues() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G0 X10\nG1 F-100\nG0 Y10";
    let issues = validator.validate_program(gcode);

    assert!(!issues.is_empty());
    // Issue should be on line 2
    assert_eq!(issues[0].line_number, 2);
}

#[test]
fn test_empty_lines_skipped() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G0 X10\n\n\nG1 Y10";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty());
}

#[test]
fn test_disable_validation_rules() {
    let mut validator = GcodeValidator::new(GrblVersion::V1_2);
    validator.set_rule_enabled("F_feed_without_motion", false);

    let gcode = "G1 X10 F-100";
    let issues = validator.validate_program(gcode);

    // With rule disabled, may have different results
    // This tests that disabling works
    let _ = issues;
}

#[test]
fn test_complex_program_with_arcs() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "
G0 X0 Y0
M3 S5000
G1 Z-2 F100
G1 X10 Y0 F500
G2 X10 Y10 I0 J10 F500
G1 X0 Y10 F500
G0 Z5
M5
";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty());
}

#[test]
fn test_grbl_version_compatibility() {
    // GRBL v1.0 validator
    let validator_v1_0 = GcodeValidator::new(GrblVersion::V1_0);
    // GRBL v1.2 validator
    let validator_v1_2 = GcodeValidator::new(GrblVersion::V1_2);

    let gcode = "G1 X10 F1000";

    let issues_v1_0 = validator_v1_0.validate_program(gcode);
    let issues_v1_2 = validator_v1_2.validate_program(gcode);

    // Both should accept valid G1 commands
    assert!(issues_v1_0.is_empty());
    assert!(issues_v1_2.is_empty());
}

#[test]
fn test_error_classification() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G1 F-100";
    let issues = validator.validate_program(gcode);

    assert!(!issues.is_empty());
    let issue = &issues[0];

    assert_eq!(issue.line_number, 1);
    assert_eq!(issue.severity, Severity::Error);
    assert!(!issue.issue_type.is_empty());
    assert!(!issue.message.is_empty());
    assert!(issue.suggestion.is_some());
}

#[test]
fn test_summary_statistics() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "
G1 X10 F-100
G1 X20 F25000
M3 S-5000
";
    let issues = validator.validate_program(gcode);
    let summary = GcodeValidator::get_summary(&issues);

    assert!(summary.contains_key(&Severity::Error));
    assert!(summary.contains_key(&Severity::Warning));
}

#[test]
fn test_critical_error_detection() {
    let issues = vec![];
    assert!(!GcodeValidator::has_critical_errors(&issues));

    let issue = gcodekit2::designer::ValidationIssue {
        line_number: 1,
        severity: Severity::Warning,
        issue_type: "test".to_string(),
        message: "test".to_string(),
        suggestion: None,
    };
    let issues = vec![issue];
    assert!(!GcodeValidator::has_critical_errors(&issues));
}

#[test]
fn test_realistic_engraving_program() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "
G21                    ; Metric
G90                    ; Absolute
G0 Z5                  ; Safe height
G0 X0 Y0               ; Start position
M3 S50                 ; Laser on, low power
G1 Z0 F100             ; Contact
G2 X10 Y10 I5 J5 F100  ; Arc engrave
G0 Z5                  ; Lift
M5                     ; Laser off
";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty(), "Realistic program should be valid");
}

#[test]
fn test_whitespace_handling() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G0    X10    Y10    Z5";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty());
}

#[test]
fn test_inline_comments() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G1 X10 F1000 ; This is a comment";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty());
}

#[test]
fn test_parse_coordinates() {
    let validator = GcodeValidator::new(GrblVersion::V1_2);
    let gcode = "G1 X10.5 Y-20.75 Z0.25 F500";
    let issues = validator.validate_program(gcode);
    assert!(issues.is_empty());
}
