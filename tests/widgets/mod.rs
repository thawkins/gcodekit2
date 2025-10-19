//! Widgets module tests

use gcodekit2::widgets::{ConnectionWidget, JogWidget, JogStepSize, OverridesWidget};
use gcodekit2::widgets::gcode_loading::{GcodeFile, GcodeLoading};

// Connection widget tests

#[test]
fn test_connection_widget_creation() {
    let widget = ConnectionWidget::new();
    assert!(!widget.connected);
    assert!(!widget.available_ports.is_empty());
}

#[test]
fn test_connection_widget_connect() {
    let mut widget = ConnectionWidget::new();
    assert!(widget.connect("/dev/ttyACM0".to_string()).is_ok());
    assert!(widget.connected);
}

#[test]
fn test_connection_widget_disconnect() {
    let mut widget = ConnectionWidget::new();
    let _ = widget.connect("/dev/ttyACM0".to_string());
    assert!(widget.disconnect().is_ok());
    assert!(!widget.connected);
}

#[test]
fn test_connection_status() {
    let mut widget = ConnectionWidget::new();
    widget.connect("/dev/ttyACM0".to_string()).unwrap();
    let status = widget.get_status();
    assert!(status.contains("Connected"));
}

#[test]
fn test_baud_rate_setting() {
    let mut widget = ConnectionWidget::new();
    widget.set_baud_rate(9600);
    assert_eq!(widget.baud_rate, 9600);
}

// Jog widget tests

#[test]
fn test_jog_widget_creation() {
    let widget = JogWidget::new();
    assert_eq!(widget.step_size, JogStepSize::Normal);
}

#[test]
fn test_step_size_values() {
    assert_eq!(JogStepSize::Small.value(), 0.1);
    assert_eq!(JogStepSize::Normal.value(), 1.0);
    assert_eq!(JogStepSize::Large.value(), 10.0);
    assert_eq!(JogStepSize::Huge.value(), 50.0);
}

#[test]
fn test_jog_x_positive() {
    let mut widget = JogWidget::new();
    let cmd = widget.jog_x_positive();
    assert!(cmd.contains("X"));
    assert!(cmd.contains("1.00"));
}

#[test]
fn test_jog_x_negative() {
    let mut widget = JogWidget::new();
    let cmd = widget.jog_x_negative();
    assert!(cmd.contains("X-1.00"));
}

#[test]
fn test_jog_y_positive() {
    let mut widget = JogWidget::new();
    let cmd = widget.jog_y_positive();
    assert!(cmd.contains("Y1.00"));
}

#[test]
fn test_jog_z_positive() {
    let mut widget = JogWidget::new();
    let cmd = widget.jog_z_positive();
    assert!(cmd.contains("Z1.00"));
}

#[test]
fn test_set_step_size() {
    let mut widget = JogWidget::new();
    widget.set_step_size(JogStepSize::Large);
    assert_eq!(widget.step_size, JogStepSize::Large);
    let cmd = widget.jog_x_positive();
    assert!(cmd.contains("X10.00"));
}

#[test]
fn test_unlock_command() {
    let widget = JogWidget::new();
    assert_eq!(widget.unlock(), "$X");
}

#[test]
fn test_resume_command() {
    let widget = JogWidget::new();
    assert_eq!(widget.resume(), "~");
}

#[test]
fn test_step_sizes_list() {
    let sizes = JogWidget::step_sizes();
    assert_eq!(sizes.len(), 4);
}

// Overrides widget tests

#[test]
fn test_overrides_creation() {
    let widget = OverridesWidget::new();
    assert_eq!(widget.feed_rate_override, 100);
    assert_eq!(widget.spindle_power_override, 100);
    assert!(!widget.laser_mode);
}

#[test]
fn test_feed_rate_adjustment() {
    let mut widget = OverridesWidget::new();
    widget.set_feed_rate(150);
    assert_eq!(widget.feed_rate_override, 150);
}

#[test]
fn test_feed_rate_clamping() {
    let mut widget = OverridesWidget::new();
    widget.set_feed_rate(250);
    assert_eq!(widget.feed_rate_override, 200);
    
    widget.set_feed_rate(10);
    assert_eq!(widget.feed_rate_override, 50);
}

#[test]
fn test_spindle_power_adjustment() {
    let mut widget = OverridesWidget::new();
    widget.set_spindle_power(75);
    assert_eq!(widget.spindle_power_override, 75);
}

#[test]
fn test_spindle_power_clamping() {
    let mut widget = OverridesWidget::new();
    widget.set_spindle_power(150);
    assert_eq!(widget.spindle_power_override, 100);
    
    widget.set_spindle_power(0);
    assert_eq!(widget.spindle_power_override, 0);
}

#[test]
fn test_increase_feed_rate() {
    let mut widget = OverridesWidget::new();
    widget.increase_feed_rate();
    assert_eq!(widget.feed_rate_override, 110);
}

#[test]
fn test_decrease_feed_rate() {
    let mut widget = OverridesWidget::new();
    widget.feed_rate_override = 120;
    widget.decrease_feed_rate();
    assert_eq!(widget.feed_rate_override, 110);
}

#[test]
fn test_toggle_laser_mode() {
    let mut widget = OverridesWidget::new();
    assert!(!widget.laser_mode);
    widget.toggle_laser_mode();
    assert!(widget.laser_mode);
}

#[test]
fn test_status_description() {
    let widget = OverridesWidget::new();
    let status = widget.get_status();
    assert!(status.contains("Feed: 100%"));
    assert!(status.contains("Power: 100%"));
}

// G-code Loading tests

#[test]
fn test_gcode_file_creation() {
    let gcode = "G0 X10\nG1 Y20\nG0 Z5".to_string();
    let file = GcodeFile::new("test.gcode".to_string(), "/tmp/test.gcode".to_string(), gcode);
    assert_eq!(file.name, "test.gcode");
    assert_eq!(file.lines, 3);
}

#[test]
fn test_gcode_file_validation() {
    let gcode = "G0 X10\nG1 Y20".to_string();
    let file = GcodeFile::new("valid.gcode".to_string(), "/tmp/valid.gcode".to_string(), gcode);
    assert!(file.validate().is_ok());
}

#[test]
fn test_gcode_file_validation_fails() {
    let gcode = "invalid content".to_string();
    let file = GcodeFile::new(
        "invalid.gcode".to_string(),
        "/tmp/invalid.gcode".to_string(),
        gcode,
    );
    assert!(file.validate().is_err());
}

#[test]
fn test_gcode_clean() {
    let gcode = "; Comment\nG0 X10\n\n  G1 Y20  \n; Another comment\nG0 Z5".to_string();
    let file = GcodeFile::new("test.gcode".to_string(), "/tmp/test.gcode".to_string(), gcode);
    let clean = file.get_clean_gcode();
    assert!(!clean.contains(";"));
    assert!(!clean.contains("\n\n"));
}

#[test]
fn test_gcode_loading_widget() {
    let widget = GcodeLoading::new();
    assert!(widget.loaded_file.is_none());
    assert_eq!(widget.queue_length(), 0);
}

#[test]
fn test_load_file() {
    let mut widget = GcodeLoading::new();
    let gcode = "G0 X10\nG1 Y20".to_string();
    let file = GcodeFile::new("test.gcode".to_string(), "/tmp/test.gcode".to_string(), gcode);
    assert!(widget.load_file(file).is_ok());
    assert!(widget.loaded_file.is_some());
}

#[test]
fn test_queue_file() {
    let mut widget = GcodeLoading::new();
    let gcode = "G0 X10".to_string();
    let file = GcodeFile::new("test.gcode".to_string(), "/tmp/test.gcode".to_string(), gcode);
    widget.queue_file(file);
    assert_eq!(widget.queue_length(), 1);
}

#[test]
fn test_get_next_line() {
    let mut widget = GcodeLoading::new();
    let gcode = "G0 X10\nG1 Y20\nG0 Z5".to_string();
    let file = GcodeFile::new("test.gcode".to_string(), "/tmp/test.gcode".to_string(), gcode);
    widget.load_file(file).unwrap();

    let line1 = widget.get_next_line();
    assert_eq!(line1, Some("G0 X10".to_string()));

    let line2 = widget.get_next_line();
    assert_eq!(line2, Some("G1 Y20".to_string()));
}

#[test]
fn test_progress() {
    let mut widget = GcodeLoading::new();
    let gcode = "G0 X10\nG1 Y20\nG0 Z5".to_string();
    let file = GcodeFile::new("test.gcode".to_string(), "/tmp/test.gcode".to_string(), gcode);
    widget.load_file(file).unwrap();
    
    assert!((widget.get_progress() - 0.0).abs() < 0.01);
    widget.get_next_line();
    assert!((widget.get_progress() - 33.33).abs() < 0.1);
}
