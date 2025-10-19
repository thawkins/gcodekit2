//! Designer module tests

use gcodekit2::designer::{Design, Designer, Shape};

mod imaging;
mod backplot;
mod validator;
mod optimizer;

#[test]
fn test_design_creation() {
    let design = Design::new("Test Design".to_string());
    assert_eq!(design.name, "Test Design");
    assert!(design.shapes.is_empty());
}

#[test]
fn test_add_shape() {
    let mut design = Design::new("Test".to_string());
    let rect = Shape::rectangle(10.0, 20.0, 5.0, 5.0);
    let id = design.add_shape(rect);
    assert!(design.get_shape(&id).is_some());
}

#[test]
fn test_designer_creation() {
    let designer = Designer::new();
    assert_eq!(designer.list_designs().len(), 0);
}

#[test]
fn test_designer_new_design() {
    let mut designer = Designer::new();
    let id = designer.new_design("My Design".to_string());
    assert_eq!(designer.list_designs().len(), 1);
    assert!(designer.get_design(&id).is_some());
}

#[test]
fn test_designer_active_design() {
    let mut designer = Designer::new();
    let id = designer.new_design("Active Design".to_string());
    assert_eq!(designer.get_active_design().is_some(), true);
}

#[test]
fn test_generate_gcode() {
    let mut design = Design::new("Test Design".to_string());
    design.add_shape(Shape::rectangle(10.0, 20.0, 0.0, 0.0));
    let gcode = design.generate_gcode();
    assert!(gcode.contains("G21"));
    assert!(gcode.contains("G90"));
    assert!(gcode.contains("M3"));
}

// Shapes tests

#[test]
fn test_rectangle_creation() {
    let rect = Shape::rectangle(10.0, 20.0, 5.0, 5.0);
    if let Shape::Rectangle { width, height, x, y } = rect {
        assert_eq!(width, 10.0);
        assert_eq!(height, 20.0);
        assert_eq!(x, 5.0);
        assert_eq!(y, 5.0);
    }
}

#[test]
fn test_rectangle_area() {
    let rect = Shape::rectangle(10.0, 20.0, 0.0, 0.0);
    assert_eq!(rect.area(), 200.0);
}

#[test]
fn test_circle_area() {
    let circle = Shape::circle(5.0, 0.0, 0.0);
    let area = circle.area();
    assert!((area - 78.53981).abs() < 0.01);
}

#[test]
fn test_rectangle_bounds() {
    let rect = Shape::rectangle(10.0, 20.0, 5.0, 3.0);
    let (min_x, min_y, max_x, max_y) = rect.bounds();
    assert_eq!(min_x, 5.0);
    assert_eq!(min_y, 3.0);
    assert_eq!(max_x, 15.0);
    assert_eq!(max_y, 23.0);
}

#[test]
fn test_circle_bounds() {
    let circle = Shape::circle(5.0, 10.0, 10.0);
    let (min_x, min_y, max_x, max_y) = circle.bounds();
    assert_eq!(min_x, 5.0);
    assert_eq!(min_y, 5.0);
    assert_eq!(max_x, 15.0);
    assert_eq!(max_y, 15.0);
}

#[test]
fn test_rectangle_contains_point() {
    let rect = Shape::rectangle(10.0, 10.0, 0.0, 0.0);
    assert!(rect.contains_point(5.0, 5.0));
    assert!(!rect.contains_point(15.0, 5.0));
}

#[test]
fn test_circle_contains_point() {
    let circle = Shape::circle(5.0, 0.0, 0.0);
    assert!(circle.contains_point(0.0, 0.0));
    assert!(circle.contains_point(3.0, 3.0));
    assert!(!circle.contains_point(6.0, 6.0));
}

#[test]
fn test_rectangle_to_gcode() {
    let rect = Shape::rectangle(10.0, 10.0, 0.0, 0.0);
    let gcode = rect.to_gcode();
    assert!(gcode.contains("G0"));
    assert!(gcode.contains("G1"));
}

#[test]
fn test_circle_to_gcode() {
    let circle = Shape::circle(5.0, 0.0, 0.0);
    let gcode = circle.to_gcode();
    assert!(gcode.contains("G2"));
}

// Toolpath tests

#[test]
fn test_toolpath_creation() {
    use gcodekit2::designer::Toolpath;
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
    use gcodekit2::designer::Toolpath;
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
    use gcodekit2::designer::Toolpath;
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
    use gcodekit2::designer::Toolpath;
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
