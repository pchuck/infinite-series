//! 3D Cone visualization - numbers spiral up a cone
//! Highlighted numbers spike outward from the cone surface

use crate::app::NumberVisualizerApp;
use crate::constants::shapes;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Draw the 3D cone visualization.
///
/// Renders numbers spiraling upward on a cone surface.
/// Highlighted numbers (primes, Fibonacci, etc.) spike outward from the surface.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let spike_distance = app.config.spike_distance;

    draw_3d_scene(app, ui, rect, "cone_3d", |n, is_highlighted| {
        let t = (n - 1) as f32 / max_n as f32;
        let angle = t * shapes::CONE_TURNS * std::f32::consts::TAU;
        let height = t * shapes::CONE_HEIGHT - shapes::CONE_HEIGHT / 2.0;
        let radius = shapes::CONE_BASE_RADIUS * (1.0 - t);

        let spike = if is_highlighted { spike_distance } else { 0.0 };
        let r = radius + spike;

        let x = r * angle.cos();
        let z = r * angle.sin();

        Point3D::new(x, height, z)
    });
}

pub struct Cone3D;

impl Visualizer for Cone3D {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::Cone3D
    }

    fn name(&self) -> &'static str {
        "3D Cone"
    }

    fn description(&self) -> &'static str {
        VisualizationType::Cone3D.description()
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
