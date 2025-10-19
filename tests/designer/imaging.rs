//! Tests for image processing module
//!
//! Tests grayscale conversion, dithering algorithms, and edge detection

use gcodekit2::designer::imaging::{
    apply_dithering, detect_edges_sobel, DitherMethod,
    ImageConfig,
};

#[test]
fn test_image_config_defaults() {
    let config = ImageConfig::default();
    assert_eq!(config.resolution, 10.0);
    assert_eq!(config.power, 50);
    assert_eq!(config.feed_rate, 1000.0);
    assert_eq!(
        config.dither_method,
        DitherMethod::FloydSteinberg
    );
}

#[test]
fn test_image_config_custom() {
    let config = ImageConfig {
        resolution: 5.0,
        power: 75,
        feed_rate: 500.0,
        dither_method: DitherMethod::Ordered,
    };
    assert_eq!(config.resolution, 5.0);
    assert_eq!(config.power, 75);
    assert_eq!(config.dither_method, DitherMethod::Ordered);
}

#[test]
fn test_dithering_none() {
    // Simple 2x2 grayscale image
    let grayscale = vec![50, 150, 200, 100];
    let result =
        apply_dithering(&grayscale, 2, 2, DitherMethod::None, 128)
            .unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(result[0], false); // 50 < 128
    assert_eq!(result[1], true);  // 150 >= 128
    assert_eq!(result[2], true);  // 200 >= 128
    assert_eq!(result[3], false); // 100 < 128
}

#[test]
fn test_dithering_ordered() {
    let grayscale = vec![100, 150, 200, 50];
    let result =
        apply_dithering(&grayscale, 2, 2, DitherMethod::Ordered, 128)
            .unwrap();

    assert_eq!(result.len(), 4);
    // Ordered dithering applies Bayer matrix
}

#[test]
fn test_dithering_floyd_steinberg() {
    let grayscale = vec![100, 150, 200, 50];
    let result =
        apply_dithering(
            &grayscale,
            2,
            2,
            DitherMethod::FloydSteinberg,
            128,
        )
        .unwrap();

    assert_eq!(result.len(), 4);
}

#[test]
fn test_dithering_jarvis_judice_ninke() {
    let grayscale = vec![100, 150, 200, 50];
    let result =
        apply_dithering(
            &grayscale,
            2,
            2,
            DitherMethod::JarvisJudiceNinke,
            128,
        )
        .unwrap();

    assert_eq!(result.len(), 4);
}

#[test]
fn test_dithering_stucki() {
    let grayscale = vec![100, 150, 200, 50];
    let result =
        apply_dithering(&grayscale, 2, 2, DitherMethod::Stucki, 128)
            .unwrap();

    assert_eq!(result.len(), 4);
}

#[test]
#[ignore]
fn test_edge_detection_sobel() {
    // Create a vertical edge: white on left,
    // black on right
    let grayscale = vec![
        255, 255, 255, 0, 0, 0, 255, 255, 255,
    ];
    let result =
        detect_edges_sobel(&grayscale, 3, 3)
            .unwrap();

    assert_eq!(result.len(), 9);
    // Center pixel should detect the edge
    assert!(result[4] > 0);
}

#[test]
fn test_dithering_all_methods() {
    let grayscale = vec![75, 125, 175, 50];

    let methods = vec![
        DitherMethod::None,
        DitherMethod::Ordered,
        DitherMethod::FloydSteinberg,
        DitherMethod::JarvisJudiceNinke,
        DitherMethod::Stucki,
    ];

    for method in methods {
        let result = apply_dithering(
            &grayscale,
            2,
            2,
            method,
            128,
        )
        .unwrap();
        assert_eq!(result.len(), 4);
    }
}

#[test]
fn test_dithering_extreme_values() {
    // Test with pure white and pure black
    let grayscale = vec![0, 255, 0, 255];
    let result =
        apply_dithering(&grayscale, 2, 2, DitherMethod::None, 128)
            .unwrap();

    assert_eq!(result[0], false); // 0 < 128
    assert_eq!(result[1], true);  // 255 >= 128
    assert_eq!(result[2], false); // 0 < 128
    assert_eq!(result[3], true);  // 255 >= 128
}

#[test]
fn test_dithering_large_image() {
    let width = 10;
    let height = 10;
    let grayscale =
        vec![128; width * height];
    let result =
        apply_dithering(&grayscale, width, height, DitherMethod::None, 128)
            .unwrap();

    assert_eq!(result.len(), width * height);
}
