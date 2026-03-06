//! 3D Klein Bottle visualization - numbers on an immersed Klein bottle
//! Highlighted numbers bulge outward from the surface

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Draw the 3D Klein bottle visualization.
///
/// Renders numbers distributed on an immersed Klein bottle surface (a non-orientable surface).
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the surface.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    draw_3d_scene(app, ui, rect, "klein_3d", |n, is_highlighted| {
        let t = (n - 1) as f32 / max_n as f32;
        let u = t * std::f32::consts::TAU * 2.0;
        let v = (n as f32 * golden_ratio).fract() * std::f32::consts::TAU;

        let spike = if is_highlighted { 8.0 } else { 0.0 };

        let r = 4.0 - 2.0 * u.cos();
        let x = ((r * v.cos() + 4.0) * u.cos() + spike * v.cos() * u.cos()) * shapes::KLEIN_RADIUS
            / 4.0;
        let y = (r * v.sin() + spike * v.sin()) * shapes::KLEIN_RADIUS / 4.0;
        let z = ((r * v.cos() + 4.0) * u.sin() + spike * v.cos() * u.sin()) * shapes::KLEIN_RADIUS
            / 4.0;

        Point3D::new(x, y, z)
    });
}
