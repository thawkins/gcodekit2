//! Materials module tests

use gcodekit2::materials::{
    Material, MaterialDatabase, MaterialType, SpeedsFeedsCalculator, ToolMaterial,
    ToolSpec,
};

#[test]
fn test_material_creation() {
    let mat = Material::new(
        "Test".to_string(),
        MaterialType::Wood,
        1000.0,
        1500,
        3.0,
        80,
    );
    assert_eq!(mat.name, "Test");
    assert_eq!(mat.feed_rate, 1000.0);
    assert_eq!(mat.spindle_speed, 1500);
}

#[test]
fn test_material_database_defaults() {
    let db = MaterialDatabase::new();
    assert!(db.count() > 5);
    assert!(db.get_material("Wood (Soft)").is_some());
}

#[test]
fn test_get_by_type() {
    let db = MaterialDatabase::new();
    let wood_materials = db.get_by_type(MaterialType::Wood);
    assert!(wood_materials.len() >= 2);
}

#[test]
fn test_add_material() {
    let mut db = MaterialDatabase::new();
    let initial_count = db.count();
    let mat = Material::new(
        "Custom".to_string(),
        MaterialType::Stone,
        500.0,
        800,
        2.0,
        75,
    );
    db.add_material(mat);
    assert_eq!(db.count(), initial_count + 1);
}

#[test]
fn test_remove_material() {
    let mut db = MaterialDatabase::new();
    let initial_count = db.count();
    db.remove_material("Wood (Soft)");
    assert_eq!(db.count(), initial_count - 1);
}

#[test]
fn test_list_materials() {
    let db = MaterialDatabase::new();
    let list = db.list_materials();
    assert!(list.contains(&"Wood (Soft)".to_string()));
}

#[test]
fn test_material_type_str() {
    assert_eq!(MaterialType::Wood.as_str(), "Wood");
    assert_eq!(MaterialType::Acrylic.as_str(), "Acrylic");
}

// Speeds and Feeds Calculator Tests

#[test]
fn test_speeds_feeds_calculator_creation() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::new(db);
    assert!(calc.quick_lookup("Wood (Soft)").is_some());
}

#[test]
fn test_calculate_speeds_feeds_wood() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::new(db);
    let tool = ToolSpec {
        diameter: 6.35,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 5000,
    };

    let result = calc.calculate("Wood (Soft)", &tool).unwrap();
    assert!(result.spindle_speed > 0);
    assert!(result.feed_rate > 0.0);
    assert!(result.spindle_speed <= 5000);
    assert!(result.chip_load > 0.0);
    assert!(result.surface_speed > 0.0);
}

#[test]
fn test_calculate_speeds_feeds_metal() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::new(db);
    let tool = ToolSpec {
        diameter: 6.35,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 3000,
    };

    let result = calc.calculate("Metal (Aluminum)", &tool).unwrap();
    assert!(result.spindle_speed > 0);
    assert!(result.feed_rate > 0.0);
    assert!(result.spindle_speed <= 3000);
}

#[test]
fn test_carbide_tool_speed_factor() {
    assert_eq!(ToolMaterial::HSS.speed_factor(), 1.0);
    assert_eq!(ToolMaterial::Carbide.speed_factor(), 3.0);
    assert_eq!(ToolMaterial::Diamond.speed_factor(), 5.0);
}

#[test]
fn test_carbide_tool_higher_speed_than_hss() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);
    let material = db.get_material("Wood (Soft)").unwrap();

    let hss_tool = ToolSpec {
        diameter: 6.35,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 5000,
    };

    let carbide_tool = ToolSpec {
        diameter: 6.35,
        flutes: 2,
        tool_material: ToolMaterial::Carbide,
        max_rpm: 15000,
    };

    let hss_result = calc.calculate_from_material(material, &hss_tool);
    let carbide_result = calc.calculate_from_material(material, &carbide_tool);

    assert!(carbide_result.spindle_speed > hss_result.spindle_speed);
}

#[test]
fn test_quick_lookup() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);

    let result = calc.quick_lookup("Acrylic");
    assert!(result.is_some());
    let (speed, feed) = result.unwrap();
    assert_eq!(speed, 1800);
    assert!(feed > 0.0);
}

#[test]
fn test_invalid_material_error() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);
    let tool = ToolSpec {
        diameter: 6.35,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 5000,
    };

    let result = calc.calculate("NonexistentMaterial", &tool);
    assert!(result.is_err());
}

#[test]
fn test_suggest_materials_by_type() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);

    let materials = calc.suggest_materials(MaterialType::Wood);
    assert!(!materials.is_empty());
    assert!(materials.iter().any(|m| m.contains("Wood")));
}

#[test]
fn test_different_materials_different_speeds() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);
    let tool = ToolSpec {
        diameter: 6.35,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 5000,
    };

    let wood_result = calc.calculate("Wood (Soft)", &tool).unwrap();
    let metal_result = calc.calculate("Metal (Aluminum)", &tool).unwrap();

    // Wood should have higher speed than metal
    assert!(wood_result.spindle_speed > metal_result.spindle_speed);
}

#[test]
fn test_rpm_respects_tool_max() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);
    let tool = ToolSpec {
        diameter: 50.0, // Large diameter
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 500, // Low max RPM
    };

    let result = calc.calculate("Wood (Soft)", &tool).unwrap();
    assert!(result.spindle_speed <= 500);
}

#[test]
fn test_large_tool_lower_rpm() {
    let db = MaterialDatabase::new();
    let calc = SpeedsFeedsCalculator::with_ref(&db);
    let material = db.get_material("Wood (Soft)").unwrap();

    let small_tool = ToolSpec {
        diameter: 3.0,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 10000,
    };

    let large_tool = ToolSpec {
        diameter: 12.0,
        flutes: 2,
        tool_material: ToolMaterial::HSS,
        max_rpm: 10000,
    };

    let small_result = calc.calculate_from_material(material, &small_tool);
    let large_result = calc.calculate_from_material(material, &large_tool);

    assert!(small_result.spindle_speed > large_result.spindle_speed);
}
