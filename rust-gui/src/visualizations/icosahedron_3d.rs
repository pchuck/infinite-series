//! 3D Icosahedron visualization - numbers distributed on 20 triangular faces
//! Highlighted numbers bulge outward from the surface

use crate::app::NumberVisualizerApp;
use crate::constants::shapes;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Return the 12 vertices of a regular icosahedron centered at the origin.
///
/// Uses the golden ratio phi to construct the vertex coordinates.
fn icosahedron_vertices() -> Vec<[f32; 3]> {
    let phi = (1.0 + 5.0f32.sqrt()) / 2.0;

    vec![
        [0.0, 1.0, phi],
        [0.0, 1.0, -phi],
        [0.0, -1.0, phi],
        [0.0, -1.0, -phi],
        [1.0, phi, 0.0],
        [1.0, -phi, 0.0],
        [-1.0, phi, 0.0],
        [-1.0, -phi, 0.0],
        [phi, 0.0, 1.0],
        [phi, 0.0, -1.0],
        [-phi, 0.0, 1.0],
        [-phi, 0.0, -1.0],
    ]
}

/// Return the 20 triangular faces of an icosahedron.
///
/// Each face is defined by indices into the vertices array.
fn icosahedron_faces() -> Vec<[usize; 3]> {
    vec![
        [0, 2, 8],
        [0, 8, 4],
        [0, 4, 6],
        [0, 6, 10],
        [0, 10, 2],
        [1, 9, 3],
        [1, 4, 9],
        [1, 6, 4],
        [1, 11, 6],
        [1, 3, 11],
        [2, 5, 8],
        [2, 7, 5],
        [2, 10, 7],
        [3, 5, 7],
        [3, 7, 11],
        [4, 8, 9],
        [5, 9, 8],
        [6, 11, 10],
        [7, 10, 11],
        [9, 5, 3],
    ]
}

/// Calculate a point on a triangular face using barycentric coordinates.
///
/// Distributes points within a triangle defined by 3 vertices.
fn point_on_triangle(
    vertices: &[[f32; 3]],
    face: &[usize; 3],
    u: f32,
    v: f32,
    spike: f32,
) -> Point3D {
    let a = vertices[face[0]];
    let b = vertices[face[1]];
    let c = vertices[face[2]];

    let w = 1.0 - u - v;

    let x = w * a[0] + u * b[0] + v * c[0];
    let y = w * a[1] + u * b[1] + v * c[1];
    let z = w * a[2] + u * b[2] + v * c[2];

    let len = (x * x + y * y + z * z).sqrt();
    let normal = [x / len, y / len, z / len];

    Point3D::new(
        (x + normal[0] * spike) * shapes::ICOSAHEDRON_SCALE,
        (y + normal[1] * spike) * shapes::ICOSAHEDRON_SCALE,
        (z + normal[2] * spike) * shapes::ICOSAHEDRON_SCALE,
    )
}

/// Draw the 3D icosahedron visualization.
///
/// Renders numbers distributed on 20 triangular faces of an icosahedron (Platonic solid).
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the faces.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let vertices = icosahedron_vertices();
    let faces = icosahedron_faces();
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;
    let spike_distance = app.config.spike_distance;

    draw_3d_scene(app, ui, rect, "icosahedron_3d", |n, is_highlighted| {
        let t = (n - 1) as f32;
        let face_idx = ((n - 1) * 20 / max_n) % 20;
        let local = (t * golden_ratio).fract();

        let u = local * 0.9 + 0.05;
        let v = (local * golden_ratio).fract() * 0.9 + 0.05;

        let (u, v) = if u + v > 0.95 {
            (0.95 - u, 0.95 - v)
        } else {
            (u, v)
        };

        let spike = if is_highlighted {
            spike_distance / 50.0
        } else {
            0.0
        };
        point_on_triangle(&vertices, &faces[face_idx], u, v, spike)
    });
}

pub struct Icosahedron3D;

impl Visualizer for Icosahedron3D {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::Icosahedron3D
    }

    fn name(&self) -> &'static str {
        "3D Icosahedron"
    }

    fn description(&self) -> &'static str {
        VisualizationType::Icosahedron3D.description()
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
