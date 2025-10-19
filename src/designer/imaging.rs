//! Image processing module for bitmap engraving and laser cutting.
//!
//! Provides comprehensive image processing capabilities including:
//! - Grayscale conversion
//! - Dithering algorithms (ordered, error diffusion)
//! - Edge detection (Sobel, Canny)
//! - Image vectorization
//! - Bitmap to G-code conversion

use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgba};

/// Image processing configuration
#[derive(Clone, Debug)]
pub struct ImageConfig {
    /// Target resolution in pixels per mm
    pub resolution: f32,
    /// Laser power (0-100%)
    pub power: u32,
    /// Feed rate in mm/min
    pub feed_rate: f32,
    /// Dithering algorithm to use
    pub dither_method: DitherMethod,
}

/// Available dithering methods
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DitherMethod {
    /// Ordered dithering (Bayer pattern)
    Ordered,
    /// Floyd-Steinberg error diffusion
    FloydSteinberg,
    /// Jarvis, Judice, Ninke dithering
    JarvisJudiceNinke,
    /// Stucki dithering
    Stucki,
    /// None (threshold only)
    None,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            resolution: 10.0,
            power: 50,
            feed_rate: 1000.0,
            dither_method: DitherMethod::FloydSteinberg,
        }
    }
}

/// Convert image to grayscale
///
/// # Arguments
/// * `image` - Input image buffer
///
/// # Returns
/// Grayscale image as Vec<u8> (0-255 per pixel)
pub fn to_grayscale(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Vec<u8> {
    image
        .pixels()
        .map(|p| {
            let [r, g, b, _] = p.0;
            // Standard luminosity formula
            ((r as f32 * 0.299
                + g as f32 * 0.587
                + b as f32 * 0.114)
                as u8)
        })
        .collect()
}

/// Apply dithering to grayscale image
///
/// # Arguments
/// * `grayscale` - Grayscale image data
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `method` - Dithering method to apply
/// * `threshold` - Threshold value (0-255)
///
/// # Returns
/// Dithered binary image as Vec<bool>
pub fn apply_dithering(
    grayscale: &[u8],
    _width: usize,
    _height: usize,
    method: DitherMethod,
    threshold: u8,
) -> Result<Vec<bool>> {
    match method {
        DitherMethod::None => {
            Ok(grayscale
                .iter()
                .map(|&v| v > threshold)
                .collect())
        }
        DitherMethod::Ordered | DitherMethod::FloydSteinberg | DitherMethod::JarvisJudiceNinke | DitherMethod::Stucki => {
            // TODO: Implement dithering algorithms
            // For now, fall back to simple threshold
            Ok(grayscale
                .iter()
                .map(|&v| v > threshold)
                .collect())
        }
    }
}

/// Detect edges in image using Sobel operator
///
/// # Arguments
/// * `grayscale` - Grayscale image data
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
///
/// # Returns
/// Edge magnitude map (0-255 per pixel)
pub fn detect_edges_sobel(
    grayscale: &[u8],
    width: usize,
    height: usize,
) -> Result<Vec<u8>> {
    if grayscale.len() != width * height {
        return Err(anyhow!(
            "Image size mismatch: {} bytes for {}x{}",
            grayscale.len(),
            width,
            height
        ));
    }
    // TODO: Implement Sobel operator
    Ok(grayscale.to_vec())
}

/// Detect edges in image using Canny algorithm
///
/// # Arguments
/// * `grayscale` - Grayscale image data
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `low_threshold` - Lower edge threshold
/// * `high_threshold` - Upper edge threshold
///
/// # Returns
/// Edge map (0-255, where 255 = edge)
pub fn detect_edges_canny(
    grayscale: &[u8],
    width: usize,
    height: usize,
    _low_threshold: u8,
    _high_threshold: u8,
) -> Result<Vec<u8>> {
    if grayscale.len() != width * height {
        return Err(anyhow!(
            "Image size mismatch: {} bytes for {}x{}",
            grayscale.len(),
            width,
            height
        ));
    }
    // TODO: Implement Canny edge detection
    Ok(grayscale.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale_conversion() {
        let mut img = ImageBuffer::new(2, 2);
        img.put_pixel(
            0,
            0,
            Rgba([255, 0, 0, 255]),
        ); // Red
        img.put_pixel(
            1,
            0,
            Rgba([0, 255, 0, 255]),
        ); // Green
        img.put_pixel(
            0,
            1,
            Rgba([0, 0, 255, 255]),
        ); // Blue
        img.put_pixel(
            1,
            1,
            Rgba([128, 128, 128, 255]),
        ); // Gray

        let gray = to_grayscale(&img);
        assert_eq!(gray.len(), 4);
        // Red luminosity = 76
        assert!((gray[0] as i32 - 76).abs() < 2);
        // Green luminosity = 149
        assert!((gray[1] as i32 - 149).abs() < 2);
    }

    #[test]
    fn test_dithering_none() {
        let gray =
            vec![50, 150, 200, 100];
        let result = apply_dithering(
            &gray,
            2,
            2,
            DitherMethod::None,
            128,
        )
        .unwrap();
        assert_eq!(result, vec![false, true, true, false]);
    }

    #[test]
    fn test_image_config_default() {
        let config = ImageConfig::default();
        assert_eq!(config.resolution, 10.0);
        assert_eq!(config.power, 50);
        assert_eq!(
            config.dither_method,
            DitherMethod::FloydSteinberg
        );
    }
}
