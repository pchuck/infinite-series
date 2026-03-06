//! 3D Möbius Strip visualization - numbers on a twisted band
//! Highlighted numbers bulge outward from the strip surface

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Draw the 3D Möbius strip visualization.
///
/// Renders numbers distributed on a Möbius strip surface (a one-sided band with a half-twist).
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the surface.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    draw_3d_scene(app, ui, rect, "mobius_3d", |n, is_highlighted| {
        let t = (n - 1) as f32 / max_n as f32;
        let u = t * std::f32::consts::TAU;
        let v = ((n as f32 * golden_ratio).fract() - 0.5) * shapes::MOBIUS_WIDTH;

        let spike = if is_highlighted { 10.0 } else { 0.0 };

        let half_u = u / 2.0;
        let x = (shapes::MOBIUS_RADIUS + (v + spike) * half_u.cos()) * u.cos();
        let y = (v + spike) * half_u.sin();
        let z = (shapes::MOBIUS_RADIUS + (v + spike) * half_u.cos()) * u.sin();

        Point3D::new(x, y, z)
    });
}
