//! Shape definition and manipulation for CAM operations

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Geometric shape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle {
        width: f64,
        height: f64,
        x: f64,
        y: f64,
    },
    Circle {
        radius: f64,
        x: f64,
        y: f64,
    },
    Polygon {
        points: Vec<(f64, f64)>,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
}

impl Shape {
    /// Create a rectangle shape
    pub fn rectangle(width: f64, height: f64, x: f64, y: f64) -> Self {
        Shape::Rectangle { width, height, x, y }
    }

    /// Create a circle shape
    pub fn circle(radius: f64, x: f64, y: f64) -> Self {
        Shape::Circle { radius, x, y }
    }

    /// Create a line shape
    pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Shape::Line { x1, y1, x2, y2 }
    }

    /// Create a polygon from points
    pub fn polygon(points: Vec<(f64, f64)>) -> Self {
        Shape::Polygon { points }
    }

    /// Calculate the area of the shape
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height, .. } => width * height,
            Shape::Circle { radius, .. } => PI * radius * radius,
            Shape::Polygon { points } => shoelace_area(points),
            Shape::Line { .. } => 0.0,
        }
    }

    /// Calculate bounding box (min_x, min_y, max_x, max_y)
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        match self {
            Shape::Rectangle { width, height, x, y } => {
                (*x, *y, x + width, y + height)
            }
            Shape::Circle { radius, x, y } => {
                (x - radius, y - radius, x + radius, y + radius)
            }
            Shape::Polygon { points } => {
                if points.is_empty() {
                    (0.0, 0.0, 0.0, 0.0)
                } else {
                    let mut min_x = points[0].0;
                    let mut min_y = points[0].1;
                    let mut max_x = points[0].0;
                    let mut max_y = points[0].1;

                    for (x, y) in points {
                        min_x = min_x.min(*x);
                        min_y = min_y.min(*y);
                        max_x = max_x.max(*x);
                        max_y = max_y.max(*y);
                    }
                    (min_x, min_y, max_x, max_y)
                }
            }
            Shape::Line { x1, y1, x2, y2 } => {
                (x1.min(*x2), y1.min(*y2), x1.max(*x2), y1.max(*y2))
            }
        }
    }

    /// Convert shape to G-code
    pub fn to_gcode(&self) -> String {
        match self {
            Shape::Rectangle { width, height, x, y } => {
                format!(
                    "G0 X{:.2} Y{:.2}\nG1 Z-1 F100\nG1 X{:.2} Y{:.2} F100\nG1 X{:.2} Y{:.2} F100\nG1 X{:.2} Y{:.2} F100\nG1 X{:.2} Y{:.2} F100\nG0 Z5",
                    x, y, x + width, y, x + width, y + height, x, y + height, x, y
                )
            }
            Shape::Circle { radius, x, y } => {
                format!(
                    "G0 X{:.2} Y{:.2}\nG1 Z-1 F100\nG2 X{:.2} Y{:.2} I{:.2} J{:.2} F100\nG0 Z5",
                    x + radius, y, x + radius, y, radius, 0.0
                )
            }
            Shape::Line { x1, y1, x2, y2 } => {
                format!(
                    "G0 X{:.2} Y{:.2}\nG1 Z-1 F100\nG1 X{:.2} Y{:.2} F100\nG0 Z5",
                    x1, y1, x2, y2
                )
            }
            Shape::Polygon { points } => {
                let mut gcode = String::new();
                if !points.is_empty() {
                    gcode.push_str(&format!("G0 X{:.2} Y{:.2}\n", points[0].0, points[0].1));
                    gcode.push_str("G1 Z-1 F100\n");
                    for (x, y) in points {
                        gcode.push_str(&format!("G1 X{:.2} Y{:.2} F100\n", x, y));
                    }
                    gcode.push_str(&format!(
                        "G1 X{:.2} Y{:.2} F100\nG0 Z5",
                        points[0].0, points[0].1
                    ));
                }
                gcode
            }
        }
    }

    /// Check if a point is inside the shape
    pub fn contains_point(&self, px: f64, py: f64) -> bool {
        match self {
            Shape::Rectangle { width, height, x, y } => {
                px >= *x && px <= x + width && py >= *y && py <= y + height
            }
            Shape::Circle { radius, x, y } => {
                let dx = px - x;
                let dy = py - y;
                dx * dx + dy * dy <= radius * radius
            }
            Shape::Polygon { points } => point_in_polygon(px, py, points),
            Shape::Line { .. } => false,
        }
    }
}

/// Calculate area using shoelace formula
fn shoelace_area(points: &[(f64, f64)]) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area += points[i].0 * points[j].1;
        area -= points[j].0 * points[i].1;
    }
    (area / 2.0).abs()
}

/// Ray casting algorithm for point in polygon
fn point_in_polygon(px: f64, py: f64, points: &[(f64, f64)]) -> bool {
    let mut inside = false;
    let mut j = points.len() - 1;

    for i in 0..points.len() {
        let (xi, yi) = points[i];
        let (xj, yj) = points[j];

        if (yi > py) != (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
