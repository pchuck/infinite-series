//! 3D Cylinder visualization - numbers spiral around a cylinder
//! Highlighted numbers spike outward from the cylinder surface

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Draw the 3D cylinder visualization.
///
/// Renders numbers spiraling around a cylinder surface.
/// Highlighted numbers (primes, Fibonacci, etc.) spike outward from the surface.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;

    draw_3d_scene(app, ui, rect, "cylinder_3d", |n, is_highlighted| {
        let t = (n - 1) as f32 / max_n as f32;
        let angle = t * shapes::CYLINDER_TURNS * std::f32::consts::TAU;
        let height = t * shapes::CYLINDER_HEIGHT - shapes::CYLINDER_HEIGHT / 2.0;

        let spike = if is_highlighted { 15.0 } else { 0.0 };
        let r = shapes::CYLINDER_RADIUS + spike;

        let x = r * angle.cos();
        let z = r * angle.sin();

        Point3D::new(x, height, z)
    });
}
