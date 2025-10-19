//! Materials module tests

use gcodekit2::materials::{Material, MaterialDatabase, MaterialType};

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
