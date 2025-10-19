//! Materials Database Module
//!
//! Provides material properties and cutting parameters for different materials
//! and tools used in laser engraving and CNC machining.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Material properties for cutting/engraving operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub material_type: MaterialType,
    pub feed_rate: f64,      // mm/min
    pub spindle_speed: u32,  // RPM
    pub cut_depth: f64,      // mm per pass
    pub laser_power: u32,    // 0-100% for laser
    pub description: String,
}

/// Material type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialType {
    Wood,
    Plastic,
    Metal,
    Acrylic,
    Fabric,
    Paper,
    Rubber,
    Stone,
    Glass,
    Leather,
}

impl MaterialType {
    /// Get all material types
    pub fn all() -> &'static [MaterialType] {
        &[
            MaterialType::Wood,
            MaterialType::Plastic,
            MaterialType::Metal,
            MaterialType::Acrylic,
            MaterialType::Fabric,
            MaterialType::Paper,
            MaterialType::Rubber,
            MaterialType::Stone,
            MaterialType::Glass,
            MaterialType::Leather,
        ]
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            MaterialType::Wood => "Wood",
            MaterialType::Plastic => "Plastic",
            MaterialType::Metal => "Metal",
            MaterialType::Acrylic => "Acrylic",
            MaterialType::Fabric => "Fabric",
            MaterialType::Paper => "Paper",
            MaterialType::Rubber => "Rubber",
            MaterialType::Stone => "Stone",
            MaterialType::Glass => "Glass",
            MaterialType::Leather => "Leather",
        }
    }
}

impl Material {
    /// Create a new material
    pub fn new(
        name: String,
        material_type: MaterialType,
        feed_rate: f64,
        spindle_speed: u32,
        cut_depth: f64,
        laser_power: u32,
    ) -> Self {
        Material {
            name,
            material_type,
            feed_rate,
            spindle_speed,
            cut_depth,
            laser_power,
            description: String::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }
}

/// Materials database
pub struct MaterialDatabase {
    materials: HashMap<String, Material>,
}

impl MaterialDatabase {
    /// Create a new materials database with defaults
    pub fn new() -> Self {
        let mut db = MaterialDatabase {
            materials: HashMap::new(),
        };
        db.load_defaults();
        db
    }

    /// Load default materials
    fn load_defaults(&mut self) {
        self.add_material(
            Material::new(
                "Wood (Soft)".to_string(),
                MaterialType::Wood,
                1000.0,
                1500,
                3.0,
                80,
            )
            .with_description("Pine, Basswood, Balsa".to_string()),
        );

        self.add_material(
            Material::new(
                "Wood (Hard)".to_string(),
                MaterialType::Wood,
                600.0,
                1200,
                2.0,
                90,
            )
            .with_description("Oak, Maple, Walnut".to_string()),
        );

        self.add_material(
            Material::new(
                "Acrylic".to_string(),
                MaterialType::Acrylic,
                800.0,
                1800,
                2.5,
                70,
            )
            .with_description("Cast and extruded acrylic".to_string()),
        );

        self.add_material(
            Material::new(
                "Plastic (PVC)".to_string(),
                MaterialType::Plastic,
                600.0,
                1500,
                1.5,
                60,
            )
            .with_description("PVC and similar plastics".to_string()),
        );

        self.add_material(
            Material::new(
                "Metal (Aluminum)".to_string(),
                MaterialType::Metal,
                500.0,
                2000,
                1.0,
                100,
            )
            .with_description("Aluminum alloys".to_string()),
        );

        self.add_material(
            Material::new(
                "Leather".to_string(),
                MaterialType::Leather,
                800.0,
                1000,
                0.5,
                50,
            )
            .with_description("Natural leather".to_string()),
        );

        self.add_material(
            Material::new(
                "Fabric (Cotton)".to_string(),
                MaterialType::Fabric,
                900.0,
                1200,
                1.0,
                40,
            )
            .with_description("Natural cotton fabrics".to_string()),
        );
    }

    /// Add a material to the database
    pub fn add_material(&mut self, material: Material) {
        self.materials.insert(material.name.clone(), material);
    }

    /// Get a material by name
    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    /// Get all materials of a specific type
    pub fn get_by_type(&self, mat_type: MaterialType) -> Vec<&Material> {
        self.materials
            .values()
            .filter(|m| m.material_type == mat_type)
            .collect()
    }

    /// List all material names
    pub fn list_materials(&self) -> Vec<String> {
        self.materials.keys().cloned().collect()
    }

    /// Get materials count
    pub fn count(&self) -> usize {
        self.materials.len()
    }

    /// Remove a material
    pub fn remove_material(&mut self, name: &str) -> Option<Material> {
        self.materials.remove(name)
    }

    /// Update a material
    pub fn update_material(&mut self, name: &str, material: Material) -> bool {
        if self.materials.contains_key(name) {
            self.materials.insert(name.to_string(), material);
            true
        } else {
            false
        }
    }
}

impl Default for MaterialDatabase {
    fn default() -> Self {
        Self::new()
    }
}
