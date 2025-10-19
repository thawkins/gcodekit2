//! Speeds and Feeds Calculator Module
//!
//! Provides professional-grade calculation of optimal cutting speeds and feed rates
//! based on material properties, tool specifications, and machine capabilities.
//! Integrates with the materials database for quick parameter lookups.

use super::{Material, MaterialDatabase, MaterialType};

/// Tool specification for speeds/feeds calculation
#[derive(Debug, Clone)]
pub struct ToolSpec {
    /// Tool diameter in mm
    pub diameter: f64,
    /// Number of flutes/teeth
    pub flutes: u32,
    /// Tool material (HSS, Carbide, etc.)
    pub tool_material: ToolMaterial,
    /// Maximum RPM the tool can handle
    pub max_rpm: u32,
}

/// Tool material types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolMaterial {
    HSS,      // High-Speed Steel
    Carbide,  // Carbide
    Diamond,  // Diamond coating
}

impl ToolMaterial {
    /// Get relative speed factor compared to base speeds
    pub fn speed_factor(&self) -> f64 {
        match self {
            ToolMaterial::HSS => 1.0,
            ToolMaterial::Carbide => 3.0,
            ToolMaterial::Diamond => 5.0,
        }
    }
}

/// Speeds and feeds calculator result
#[derive(Debug, Clone)]
pub struct SpeedsFeedsResult {
    /// Recommended spindle speed in RPM
    pub spindle_speed: u32,
    /// Recommended feed rate in mm/min
    pub feed_rate: f64,
    /// Chip load per tooth in mm
    pub chip_load: f64,
    /// Surface speed in m/min (SFM * 3.28)
    pub surface_speed: f64,
    /// Notes about the calculation
    pub notes: String,
}

/// Speeds and feeds calculator
pub struct SpeedsFeedsCalculator {
    database: MaterialDatabase,
}

impl SpeedsFeedsCalculator {
    /// Create a new calculator with the given material database
    pub fn new(database: MaterialDatabase) -> Self {
        SpeedsFeedsCalculator { database }
    }

    /// Create a new calculator by reference (clones the database)
    pub fn with_ref(database: &MaterialDatabase) -> Self {
        SpeedsFeedsCalculator {
            database: database.clone(),
        }
    }

    /// Calculate speeds and feeds for a material and tool combination
    pub fn calculate(
        &self,
        material_name: &str,
        tool: &ToolSpec,
    ) -> anyhow::Result<SpeedsFeedsResult> {
        let material = self
            .database
            .get_material(material_name)
            .ok_or_else(|| anyhow::anyhow!("Material '{}' not found", material_name))?;

        let result = self.calculate_from_material(material, tool);
        Ok(result)
    }

    /// Calculate speeds and feeds directly from a material
    pub fn calculate_from_material(
        &self,
        material: &Material,
        tool: &ToolSpec,
    ) -> SpeedsFeedsResult {
        let surface_speed_sfm = self.get_surface_speed_sfm(material);
        let adjusted_speed = surface_speed_sfm * tool.tool_material.speed_factor();

        // Calculate spindle speed: RPM = (SFM * 3.82) / Diameter
        let rpm_calculated = (adjusted_speed * 3.82) / tool.diameter;
        let spindle_speed = (rpm_calculated.min(tool.max_rpm as f64) as u32).max(100);

        // Calculate chip load
        let chip_load = match material.material_type {
            MaterialType::Wood | MaterialType::Plastic | MaterialType::Acrylic => 0.05,
            MaterialType::Metal => 0.03,
            MaterialType::Leather | MaterialType::Fabric => 0.02,
            MaterialType::Paper | MaterialType::Rubber => 0.02,
            MaterialType::Stone | MaterialType::Glass => 0.01,
        };

        // Calculate feed rate: Feed = Chip Load * Flutes * RPM
        let feed_rate = chip_load * tool.flutes as f64 * spindle_speed as f64;

        // Convert surface speed to m/min
        let surface_speed = adjusted_speed * 0.3048;

        let notes = format!(
            "Base SFM: {:.1}, Tool Speed Factor: {:.1}x, Max RPM: {}",
            surface_speed_sfm,
            tool.tool_material.speed_factor(),
            tool.max_rpm
        );

        SpeedsFeedsResult {
            spindle_speed,
            feed_rate,
            chip_load,
            surface_speed,
            notes,
        }
    }

    /// Get recommended surface speed in SFM for a material
    fn get_surface_speed_sfm(&self, material: &Material) -> f64 {
        // Base SFM values for different materials (typical for HSS tools)
        match material.material_type {
            MaterialType::Wood => 200.0,
            MaterialType::Plastic => 150.0,
            MaterialType::Acrylic => 120.0,
            MaterialType::Metal => 80.0,
            MaterialType::Leather => 100.0,
            MaterialType::Fabric => 100.0,
            MaterialType::Paper => 150.0,
            MaterialType::Rubber => 80.0,
            MaterialType::Stone => 50.0,
            MaterialType::Glass => 40.0,
        }
    }

    /// Get quick parameters from material name
    pub fn quick_lookup(&self, material_name: &str) -> Option<(u32, f64)> {
        self.database
            .get_material(material_name)
            .map(|m| (m.spindle_speed, m.feed_rate))
    }

    /// Suggest materials by type
    pub fn suggest_materials(&self, mat_type: MaterialType) -> Vec<String> {
        self.database
            .get_by_type(mat_type)
            .iter()
            .map(|m| m.name.clone())
            .collect()
    }
}

