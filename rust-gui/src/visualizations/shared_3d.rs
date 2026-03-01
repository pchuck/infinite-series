//! Shared utilities for 3D visualizations

use crate::constants::projection;
use eframe::egui;

// Re-export constants for backward compatibility
pub use crate::constants::projection::*;

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    /// Create a new 3D point with the given coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Project a 3D point to 2D screen coordinates with perspective.
///
/// Applies Y-axis rotation followed by X-axis rotation, then perspective projection.
///
/// # Returns
/// A tuple of (screen_x, screen_y, depth) where depth is used for sorting.
pub fn project_3d_to_2d(point: &Point3D, rotation_y: f32, rotation_x: f32) -> (f32, f32, f32) {
    let cos_y = rotation_y.cos();
    let sin_y = rotation_y.sin();
    let x1 = point.x * cos_y - point.z * sin_y;
    let z1 = point.x * sin_y + point.z * cos_y;
    let y1 = point.y;

    let cos_x = rotation_x.cos();
    let sin_x = rotation_x.sin();
    let y2 = y1 * cos_x - z1 * sin_x;
    let z2 = y1 * sin_x + z1 * cos_x;

    let scale = projection::PERSPECTIVE / (projection::PERSPECTIVE + z2 + projection::OFFSET);

    (x1 * scale, y2 * scale, z2)
}

/// Adjust the brightness of a color by a multiplicative factor.
///
/// Each RGB component is multiplied by the factor and clamped to 255.
pub fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = (color.r() as f32 * factor).min(255.0) as u8;
    let g = (color.g() as f32 * factor).min(255.0) as u8;
    let b = (color.b() as f32 * factor).min(255.0) as u8;
    egui::Color32::from_rgb(r, g, b)
}

/// Calculate a depth-based brightness factor for 3D rendering.
///
/// Returns a value between MIN_DEPTH_FACTOR and MAX_DEPTH_FACTOR based on the depth.
/// Used to darken objects that are farther away.
pub fn depth_factor(depth: f32) -> f32 {
    ((depth + projection::OFFSET) / projection::DEPTH_RANGE)
        .clamp(projection::MIN_DEPTH_FACTOR, projection::MAX_DEPTH_FACTOR)
}
