//! Designer Module for CAM operations and shape manipulation
//!
//! Provides shape generation, boolean operations, and G-code generation
//! for laser engraving and CNC machining operations.

pub mod shapes;
pub mod toolpath;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub use shapes::Shape;
pub use toolpath::Toolpath;

/// Design document containing shapes and operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Design {
    pub id: String,
    pub name: String,
    pub shapes: HashMap<String, Shape>,
    pub toolpaths: Vec<Toolpath>,
    pub notes: String,
}

impl Design {
    /// Create a new design
    pub fn new(name: String) -> Self {
        Design {
            id: Uuid::new_v4().to_string(),
            name,
            shapes: HashMap::new(),
            toolpaths: Vec::new(),
            notes: String::new(),
        }
    }

    /// Add a shape to the design
    pub fn add_shape(&mut self, shape: Shape) -> String {
        let id = Uuid::new_v4().to_string();
        self.shapes.insert(id.clone(), shape);
        id
    }

    /// Remove a shape from the design
    pub fn remove_shape(&mut self, id: &str) -> Option<Shape> {
        self.shapes.remove(id)
    }

    /// Get a shape by ID
    pub fn get_shape(&self, id: &str) -> Option<&Shape> {
        self.shapes.get(id)
    }

    /// Generate G-code from all shapes
    pub fn generate_gcode(&self) -> String {
        let mut gcode = String::new();
        gcode.push_str("; Generated G-code from GCodeKit Design\n");
        gcode.push_str(&format!("; Design: {}\n", self.name));
        gcode.push_str("G21 ; Use millimeters\n");
        gcode.push_str("G90 ; Absolute positioning\n");
        gcode.push_str("M3 ; Start spindle\n\n");

        for (_, shape) in &self.shapes {
            gcode.push_str(&shape.to_gcode());
            gcode.push_str("\n\n");
        }

        gcode.push_str("M5 ; Stop spindle\n");
        gcode.push_str("G0 Z5 ; Rapid to safe height\n");

        gcode
    }

    /// Clear all shapes
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.toolpaths.clear();
    }
}

/// Designer for managing CAM operations
pub struct Designer {
    designs: HashMap<String, Design>,
    active_design: Option<String>,
}

impl Designer {
    /// Create a new designer
    pub fn new() -> Self {
        Designer {
            designs: HashMap::new(),
            active_design: None,
        }
    }

    /// Create a new design
    pub fn new_design(&mut self, name: String) -> String {
        let design = Design::new(name);
        let id = design.id.clone();
        self.designs.insert(id.clone(), design);
        self.active_design = Some(id.clone());
        id
    }

    /// Get active design
    pub fn get_active_design(&mut self) -> Option<&mut Design> {
        if let Some(id) = &self.active_design.clone() {
            self.designs.get_mut(id)
        } else {
            None
        }
    }

    /// Set active design
    pub fn set_active_design(&mut self, id: String) -> bool {
        if self.designs.contains_key(&id) {
            self.active_design = Some(id);
            true
        } else {
            false
        }
    }

    /// Delete a design
    pub fn delete_design(&mut self, id: &str) -> Option<Design> {
        if Some(id.to_string()) == self.active_design {
            self.active_design = None;
        }
        self.designs.remove(id)
    }

    /// List all design IDs
    pub fn list_designs(&self) -> Vec<String> {
        self.designs.keys().cloned().collect()
    }

    /// Get design by ID
    pub fn get_design(&self, id: &str) -> Option<&Design> {
        self.designs.get(id)
    }
}

impl Default for Designer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(designer.active_design, Some(id.clone()));
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
}
