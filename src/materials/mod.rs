//! Materials database
//!
//! Stores material properties and cutting parameters for different materials.

use serde::{Deserialize, Serialize};

/// Material properties for cutting operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub material_type: MaterialType,
    pub feed_rate: f64,  // mm/min
    pub spindle_speed: f64,  // RPM or laser power
    pub depth_per_pass: f64,  // mm
}

/// Different material types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialType {
    Wood,
    Plastic,
    Acrylic,
    Metal,
    Composite,
}

impl Material {
    /// Create a new material definition
    pub fn new(
        name: String,
        material_type: MaterialType,
        feed_rate: f64,
        spindle_speed: f64,
        depth_per_pass: f64,
    ) -> Self {
        Self {
            name,
            material_type,
            feed_rate,
            spindle_speed,
            depth_per_pass,
        }
    }
}

/// Material database
pub struct MaterialDatabase {
    materials: Vec<Material>,
}

impl MaterialDatabase {
    /// Create a new material database
    pub fn new() -> Self {
        let mut db = Self {
            materials: Vec::new(),
        };
        db.populate_defaults();
        db
    }

    /// Populate database with default materials
    fn populate_defaults(&mut self) {
        self.materials.push(Material::new(
            "Plywood 3mm".to_string(),
            MaterialType::Wood,
            1000.0,
            10000.0,
            2.0,
        ));
        self.materials.push(Material::new(
            "Acrylic 3mm".to_string(),
            MaterialType::Acrylic,
            800.0,
            12000.0,
            2.5,
        ));
        self.materials.push(Material::new(
            "Aluminum".to_string(),
            MaterialType::Metal,
            600.0,
            3000.0,
            1.0,
        ));
    }

    /// Get a material by name
    pub fn get(&self, name: &str) -> Option<&Material> {
        self.materials.iter().find(|m| m.name == name)
    }

    /// Get all materials
    pub fn all(&self) -> &[Material] {
        &self.materials
    }

    /// Add a custom material
    pub fn add(&mut self, material: Material) {
        self.materials.push(material);
    }
}

impl Default for MaterialDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat = Material::new(
            "Test".to_string(),
            MaterialType::Wood,
            1000.0,
            10000.0,
            2.0,
        );
        assert_eq!(mat.name, "Test");
        assert_eq!(mat.feed_rate, 1000.0);
    }

    #[test]
    fn test_database_defaults() {
        let db = MaterialDatabase::new();
        assert!(db.get("Plywood 3mm").is_some());
        assert!(db.get("Acrylic 3mm").is_some());
    }
}
