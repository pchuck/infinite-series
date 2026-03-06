//! 3D Spiral Helix visualization - numbers spiral upward like DNA
//! Highlighted numbers (primes, Fibonacci, etc.) spike outward from the helix

use crate::constants::helix;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Draw the 3D helix visualization.
///
/// Renders numbers spiraling upward in a helix pattern.
/// Highlighted numbers (primes, Fibonacci, etc.) spike outward from the helix.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let angle_step = helix::TURNS * std::f32::consts::TAU / max_n as f32;
    let height_step = helix::HEIGHT_FACTOR * helix::RADIUS / max_n as f32;

    draw_3d_scene(app, ui, rect, "helix_3d", |n, is_highlighted| {
        let t = (n - 1) as f32;
        let angle = t * angle_step;
        let height = t * height_step - helix::HEIGHT_FACTOR * helix::RADIUS / 2.0;

        let x = helix::RADIUS * angle.cos();
        let z = helix::RADIUS * angle.sin();

        let spike = if is_highlighted { 25.0 } else { 0.0 };
        let spike_x = x + spike * angle.cos();
        let spike_z = z + spike * angle.sin();

        Point3D::new(spike_x, height, spike_z)
    });
}
