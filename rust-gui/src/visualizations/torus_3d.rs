//! 3D Torus visualization - numbers wrapped around a donut shape
//! Highlighted numbers bulge outward from the torus surface

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Draw the 3D torus visualization.
///
/// Renders numbers wrapped around a torus (donut) shape.
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the surface.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    draw_3d_scene(app, ui, rect, "torus_3d", |n, is_highlighted| {
        let t = (n - 1) as f32;
        let u = t / max_n as f32 * std::f32::consts::TAU * golden_ratio;
        let v = t * golden_ratio / max_n as f32 * std::f32::consts::TAU;

        let minor_r = if is_highlighted {
            shapes::TORUS_MINOR_RADIUS + 10.0
        } else {
            shapes::TORUS_MINOR_RADIUS
        };

        let x = (shapes::TORUS_MAJOR_RADIUS + minor_r * v.cos()) * u.cos();
        let y = minor_r * v.sin();
        let z = (shapes::TORUS_MAJOR_RADIUS + minor_r * v.cos()) * u.sin();

        Point3D::new(x, y, z)
    });
}
