//! Shape generation for CAM operations
//!
//! Generates basic shapes (rectangles, circles, lines) and converts them to G-code.

use serde::{Deserialize, Serialize};

/// Represents a basic 2D shape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    Circle {
        x: f64,
        y: f64,
        radius: f64,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
}

impl Shape {
    /// Convert shape to G-code commands
    pub fn to_gcode(&self, feed_rate: f64, spindle_speed: f64) -> Vec<String> {
        let mut gcode = vec![
            format!("G21 (mm units)"),
            format!("G90 (absolute positioning)"),
            format!("F{} (feed rate)", feed_rate),
            format!("S{} (spindle speed)", spindle_speed),
            format!("G0 Z5 (safe height)"),
        ];

        match self {
            Shape::Rectangle { x, y, width, height } => {
                gcode.push(format!("G0 X{} Y{} (move to start)", x, y));
                gcode.push(format!("G1 Z0 (plunge)"));
                gcode.push(format!("G1 X{} Y{}", x + width, y));
                gcode.push(format!("G1 X{} Y{}", x + width, y + height));
                gcode.push(format!("G1 X{} Y{}", x, y + height));
                gcode.push(format!("G1 X{} Y{}", x, y));
                gcode.push(format!("G0 Z5 (raise)"));
            }
            Shape::Circle { x, y, radius } => {
                gcode.push(format!("G0 X{} Y{} (move to start)", x, y));
                gcode.push(format!("G1 Z0 (plunge)"));
                gcode.push(format!("G2 X{} Y{} I{} J0 (circle)", x, y, radius));
                gcode.push(format!("G0 Z5 (raise)"));
            }
            Shape::Line { x1, y1, x2, y2 } => {
                gcode.push(format!("G0 X{} Y{} (move to start)", x1, y1));
                gcode.push(format!("G1 Z0 (plunge)"));
                gcode.push(format!("G1 X{} Y{} (line to end)", x2, y2));
                gcode.push(format!("G0 Z5 (raise)"));
            }
        }

        gcode.push(format!("M30 (end program)"));
        gcode
    }

    /// Get bounding box of the shape
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        match self {
            Shape::Rectangle { x, y, width, height } => {
                (*x, *y, x + width, y + height)
            }
            Shape::Circle { x, y, radius } => {
                (x - radius, y - radius, x + radius, y + radius)
            }
            Shape::Line { x1, y1, x2, y2 } => {
                let min_x = x1.min(*x2);
                let max_x = x1.max(*x2);
                let min_y = y1.min(*y2);
                let max_y = y1.max(*y2);
                (min_x, min_y, max_x, max_y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_bounds() {
        let rect = Shape::Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 20.0,
        };
        let (x1, y1, x2, y2) = rect.bounds();
        assert_eq!(x1, 0.0);
        assert_eq!(y1, 0.0);
        assert_eq!(x2, 10.0);
        assert_eq!(y2, 20.0);
    }

    #[test]
    fn test_circle_bounds() {
        let circle = Shape::Circle {
            x: 5.0,
            y: 5.0,
            radius: 3.0,
        };
        let (x1, y1, x2, y2) = circle.bounds();
        assert_eq!(x1, 2.0);
        assert_eq!(y1, 2.0);
        assert_eq!(x2, 8.0);
        assert_eq!(y2, 8.0);
    }

    #[test]
    fn test_rectangle_to_gcode() {
        let rect = Shape::Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
        let gcode = rect.to_gcode(100.0, 1000.0);
        assert!(gcode.iter().any(|line| line.contains("G21")));
        assert!(gcode.iter().any(|line| line.contains("G90")));
    }
}
