//! Shared utilities for 3D visualizations

use eframe::egui;

pub const PERSPECTIVE: f32 = 500.0;
pub const PROJECTION_OFFSET: f32 = 300.0;
pub const DEPTH_RANGE: f32 = 600.0;
pub const MIN_DEPTH_FACTOR: f32 = 0.3;
pub const MAX_DEPTH_FACTOR: f32 = 1.0;
pub const DRAG_SENSITIVITY: f32 = 0.01;

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

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

    let scale = PERSPECTIVE / (PERSPECTIVE + z2 + PROJECTION_OFFSET);

    (x1 * scale, y2 * scale, z2)
}

pub fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = (color.r() as f32 * factor).min(255.0) as u8;
    let g = (color.g() as f32 * factor).min(255.0) as u8;
    let b = (color.b() as f32 * factor).min(255.0) as u8;
    egui::Color32::from_rgb(r, g, b)
}

pub fn depth_factor(depth: f32) -> f32 {
    ((depth + PROJECTION_OFFSET) / DEPTH_RANGE).clamp(MIN_DEPTH_FACTOR, MAX_DEPTH_FACTOR)
}
