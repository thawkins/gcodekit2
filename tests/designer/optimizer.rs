//! G-code optimizer integration tests

use gcodekit2::designer::{GcodeOptimizer, OptimizerOptions};

#[test]
fn test_optimizer_creation() {
    let optimizer = GcodeOptimizer::new();
    assert!(optimizer.options().truncate_decimals);
    assert!(!optimizer.options().convert_arcs);
}

#[test]
fn test_custom_options() {
    let mut options = OptimizerOptions::default();
    options.decimal_places = 3;
    options.convert_arcs = true;
    let optimizer = GcodeOptimizer::with_options(options);
    assert_eq!(optimizer.options().decimal_places, 3);
    assert!(optimizer.options().convert_arcs);
}

#[test]
fn test_simple_decimal_truncation() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X10.5555 Y20.7777 Z5.1234\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();
    assert!(result.contains("X10.55") || result.contains("X10.56"));
    assert!(result.contains("Y20.77") || result.contains("Y20.78"));
}

#[test]
fn test_complete_program_optimization() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G21\n\n\nG90\nG0 X10.5555 Y20.7777 Z5.1234\nG1 Z-2.5678 F1000.9999\nM5\n";
    let result = optimizer.optimize(gcode).unwrap();

    // Should remove empty lines
    assert!(!result.contains("\n\n\n"));

    // Should truncate decimals
    assert!(!result.contains("10.5555"));
    assert!(!result.contains("1000.9999"));

    // Should preserve commands
    assert!(result.contains("G21"));
    assert!(result.contains("G90"));
    assert!(result.contains("G0"));
    assert!(result.contains("G1"));
    assert!(result.contains("M5"));
}

#[test]
fn test_realistic_cutting_program() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G21                 ; Millimeters
G90                 ; Absolute mode
G0 X0.0000 Y0.0000 Z5.0000 ; Rapid to origin
M3 S5000.0000              ; Spindle on
G1 Z-2.5000 F100.0000      ; Feed down
G1 X10.5555 F500.0000      ; Cut to X
G1 Y20.7777                ; Cut to Y
G1 X0.0000                 ; Return to X
G1 Y0.0000                 ; Return to Y
G0 Z5.0000                 ; Rapid up
M5                         ; Spindle off
";
    let result = optimizer.optimize(gcode).unwrap();
    let stats = GcodeOptimizer::get_stats(gcode, &result);

    assert!(stats.size_reduction_bytes > 0);
    assert!(stats.size_reduction_percent > 0.0);
    assert!(stats.size_reduction_percent <= 100.0);
}

#[test]
fn test_preserve_inline_comments() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X10.5555 Y20.7777 ; Move to position\nG1 Z-5.1234 F100.9999 ; Feed down\n";
    let result = optimizer.optimize(gcode).unwrap();

    assert!(result.contains("; Move to position"));
    assert!(result.contains("; Feed down"));
}

#[test]
fn test_decimal_places_zero() {
    let mut options = OptimizerOptions::default();
    options.decimal_places = 0;
    let optimizer = GcodeOptimizer::with_options(options);

    let gcode = "G0 X10.9999 Y20.1111 Z5.5555\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    // Numbers should be integers
    assert!(!result.contains("10.9"));
    assert!(!result.contains("20.1"));
    assert!(!result.contains("5.5"));
}

#[test]
fn test_decimal_places_three() {
    let mut options = OptimizerOptions::default();
    options.decimal_places = 3;
    let optimizer = GcodeOptimizer::with_options(options);

    let gcode = "G0 X10.5555 Y20.7777\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("X10.555") || result.contains("X10.556"));
    assert!(result.contains("Y20.777") || result.contains("Y20.778"));
}

#[test]
fn test_negative_values() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X-10.5555 Y-20.7777 Z-5.1234\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("X-10.55") || result.contains("X-10.56"));
    assert!(result.contains("Y-20.77") || result.contains("Y-20.78"));
}

#[test]
fn test_preserve_g_and_m_codes() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G20\nG21\nG90\nG91\nG0 X10\nG1 Y20\nG2 X30 Y30 I5 J5\nG3 X40 Y40 I5 J5\nM3\nM4\nM5\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("G20"));
    assert!(result.contains("G21"));
    assert!(result.contains("G90"));
    assert!(result.contains("G91"));
    assert!(result.contains("G0"));
    assert!(result.contains("G1"));
    assert!(result.contains("G2"));
    assert!(result.contains("G3"));
    assert!(result.contains("M3"));
    assert!(result.contains("M4"));
    assert!(result.contains("M5"));
}

#[test]
fn test_remove_multiple_empty_lines() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X10\n\n\n\nG1 Y20\n\n\nG0 Z5\n";
    let result = optimizer.remove_redundant_whitespace(gcode);

    let consecutive_newlines = result.contains("\n\n");
    assert!(!consecutive_newlines);
}

#[test]
fn test_collapse_multiple_spaces() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0    X10    Y20    Z5    F100\n";
    let result = optimizer.remove_redundant_whitespace(gcode);

    assert!(result.contains("G0 X10 Y20 Z5 F100"));
}

#[test]
fn test_optimization_stats_large_reduction() {
    let original = "G0   X10.55555   Y20.77777   Z5.12345   F1000.9999\n\n\n\n";
    let optimized = "G0 X10.55 Y20.77 Z5.12 F1000.99\n";
    let stats = GcodeOptimizer::get_stats(original, optimized);

    assert!(stats.size_reduction_percent > 0.0);
    assert!(stats.size_reduction_percent < 100.0);
    assert!(stats.size_reduction_bytes > 0);
}

#[test]
fn test_feed_rate_precision() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G1 X10 F1234.5678\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("F1234.56"));
}

#[test]
fn test_spindle_speed_precision() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "M3 S5000.9999\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("S5000.99") || result.contains("S5000"));
}

#[test]
fn test_mixed_integer_and_decimal() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X10 Y20.5555 Z30.123 F1000\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("X10"));
    assert!(result.contains("Y20.55"));
    assert!(result.contains("Z30.12"));
    assert!(result.contains("F1000"));
}

#[test]
fn test_complex_engraving_program() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G21
G90
G0 X0.0000 Y0.0000 Z5.0000
M3 S50.0000
G1 Z0.0000 F100.0000
G2 X10.5555 Y10.5555 I5.2777 J5.2777 F100.0000
G1 X20.1111 Y20.1111 F100.0000
G3 X30.0000 Y30.0000 I5.0000 J5.0000 F100.0000
G0 Z5.0000
M5
";
    let result = optimizer.optimize(gcode).unwrap();

    // Verify optimization occurred
    let stats = GcodeOptimizer::get_stats(gcode, &result);
    assert!(stats.size_reduction_percent > 0.0);

    // Verify structure preserved
    assert!(result.contains("G21"));
    assert!(result.contains("G90"));
    assert!(result.contains("M3"));
    assert!(result.contains("M5"));
}

#[test]
fn test_arc_conversion_enabled() {
    let mut options = OptimizerOptions::default();
    options.convert_arcs = true;
    let optimizer = GcodeOptimizer::with_options(options);

    let gcode = "G0 X0 Y0\nG2 X10 Y10 I5 J5\nG0 Z5\n";
    let result = optimizer.optimize(gcode).unwrap();

    // Arc conversion should work
    assert!(!result.is_empty());
}

#[test]
fn test_full_line_comment_preservation() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "; Start of program
; Cut path for part
G0 X10.5555
; End of path
G0 Z5.1234
; End of program
";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("; Start of program"));
    assert!(result.contains("; Cut path for part"));
    assert!(result.contains("; End of path"));
    assert!(result.contains("; End of program"));
}

#[test]
fn test_very_small_numbers() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X0.0001 Y0.0002 Z0.0003\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    // Should truncate to 2 decimal places
    assert!(result.contains("X0") || result.contains("X0."));
}

#[test]
fn test_very_large_numbers() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G0 X12345.6789 Y98765.4321\n";
    let result = optimizer.truncate_decimal_precision(gcode).unwrap();

    assert!(result.contains("X12345.67"));
    assert!(result.contains("Y98765.43"));
}

#[test]
fn test_empty_program() {
    let optimizer = GcodeOptimizer::new();
    let result = optimizer.optimize("").unwrap();
    assert!(result.is_empty() || result == "\n");
}

#[test]
fn test_only_whitespace() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "\n\n\n   \n\n";
    let result = optimizer.remove_redundant_whitespace(gcode);

    // Should be cleaned up
    let non_empty_lines = result.lines().filter(|l| !l.trim().is_empty()).count();
    assert_eq!(non_empty_lines, 0);
}

#[test]
fn test_realistic_laser_engraving() {
    let optimizer = GcodeOptimizer::new();
    let gcode = "G21            ; Millimeters
G90            ; Absolute positioning
G0 X0.0000 Y0.0000   ; Home
M3 S100.0000         ; Laser on
G1 Z0.0000 F50.0000  ; Contact
G1 X50.5555 Y50.5555 F100.0000  ; Engrave line
G1 X100.1111 Y100.1111 F100.0000 ; Continue
G0 Z5.0000           ; Lift
M5                   ; Laser off
";
    let result = optimizer.optimize(gcode).unwrap();
    let stats = GcodeOptimizer::get_stats(gcode, &result);

    // Should have significant reduction
    assert!(stats.size_reduction_percent > 10.0);
    assert!(stats.size_reduction_percent < 100.0);
}
