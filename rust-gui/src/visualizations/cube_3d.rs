//! 3D Cube visualization - numbers distributed on cube surface
//! Highlighted numbers bulge outward from the faces

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Calculate a point on a cube face.
///
/// Returns a 3D point on one of the 6 cube faces based on face index and UV coordinates.
fn cube_face_point(face: usize, u: f32, v: f32, spike: f32) -> Point3D {
    let half = shapes::CUBE_SIZE / 2.0 + spike;
    match face % 6 {
        0 => Point3D::new(u * half, half, v * half),
        1 => Point3D::new(u * half, -half, v * half),
        2 => Point3D::new(half, u * half, v * half),
        3 => Point3D::new(-half, u * half, v * half),
        4 => Point3D::new(u * half, v * half, half),
        _ => Point3D::new(u * half, v * half, -half),
    }
}

/// Draw the 3D cube visualization.
///
/// Renders numbers distributed on the 6 faces of a cube.
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the faces.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let points_per_face = (max_n / 6).max(1);
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    draw_3d_scene(app, ui, rect, "cube_3d", |n, is_highlighted| {
        let t = (n - 1) as f32;
        let face = ((n - 1) * 6 / max_n) % 6;
        let local_t = (t % points_per_face as f32) / points_per_face as f32;

        let u = (local_t * golden_ratio).fract() * 2.0 - 1.0;
        let v = (local_t * golden_ratio * golden_ratio).fract() * 2.0 - 1.0;

        let spike = if is_highlighted { 12.0 } else { 0.0 };
        cube_face_point(face, u, v, spike)
    });
}
