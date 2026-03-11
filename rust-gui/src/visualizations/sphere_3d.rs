//! 3D Sphere visualization - numbers distributed on sphere surface
//! Highlighted numbers bulge outward from the surface

use crate::app::NumberVisualizerApp;
use crate::constants::shapes;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Calculate a point on a sphere using the Fibonacci sphere algorithm.
///
/// Uses the golden ratio to evenly distribute points on a sphere surface.
fn fibonacci_sphere_point(n: usize, total: usize) -> (f32, f32, f32) {
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;
    let theta = 2.0 * std::f32::consts::PI * n as f32 / golden_ratio;
    let phi = (1.0 - 2.0 * n as f32 / total as f32).acos();

    let x = phi.sin() * theta.cos();
    let y = phi.cos();
    let z = phi.sin() * theta.sin();

    (x, y, z)
}

/// Draw the 3D sphere visualization.
///
/// Renders numbers distributed evenly on a sphere surface using the Fibonacci sphere algorithm.
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the surface.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let spike_distance = app.config.spike_distance;

    draw_3d_scene(app, ui, rect, "sphere_3d", |n, is_highlighted| {
        let (nx, ny, nz) = fibonacci_sphere_point(n - 1, max_n);
        let radius = if is_highlighted {
            shapes::SPHERE_RADIUS + spike_distance
        } else {
            shapes::SPHERE_RADIUS
        };
        Point3D::new(nx * radius, ny * radius, nz * radius)
    });
}

pub struct Sphere3D;

impl Visualizer for Sphere3D {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::Sphere3D
    }

    fn name(&self) -> &'static str {
        "3D Sphere"
    }

    fn description(&self) -> &'static str {
        VisualizationType::Sphere3D.description()
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
