//! 3D Spiral Helix visualization - numbers spiral upward like DNA
//! Highlighted numbers (primes, Fibonacci, etc.) spike outward from the helix

use crate::app::NumberVisualizerApp;
use crate::constants::helix;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use crate::visualizations::traits::Visualizer;
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

pub struct Helix3D;

impl Visualizer for Helix3D {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::Helix3D
    }

    fn name(&self) -> &'static str {
        "3D Helix"
    }

    fn description(&self) -> &'static str {
        VisualizationType::Helix3D.description()
    }

    fn supports_series(&self, _series: SeriesType) -> bool {
        true
    }

    fn supports_hover(&self) -> bool {
        false
    }

    fn uses_point_rendering(&self) -> bool {
        true
    }

    fn generate_positions(&self, _max_n: usize, _params: &VizParams) -> Vec<(usize, f32, f32)> {
        Vec::new()
    }

    fn draw(
        &self,
        app: &mut NumberVisualizerApp,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        _positions: &[(usize, f32, f32)],
    ) {
        draw(app, ui, rect);
    }
}
