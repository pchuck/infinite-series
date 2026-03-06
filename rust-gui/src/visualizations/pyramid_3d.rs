//! 3D Pyramid visualization - evenly distributed point cloud on all faces
//! Highlighted numbers spike outward from the pyramid faces

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Calculate a point on the pyramid surface.
///
/// Distributes points across 4 triangular faces and the square base.
fn point_on_pyramid_surface(seed: f32, u: f32, v: f32, spike: f32) -> Point3D {
    let half = shapes::PYRAMID_BASE / 2.0;
    let h = shapes::PYRAMID_HEIGHT / 2.0;

    let apex = [0.0f32, h, 0.0f32];
    let base_corners: [[f32; 3]; 4] = [
        [half, -h, -half],
        [half, -h, half],
        [-half, -h, half],
        [-half, -h, -half],
    ];

    let face = (seed * 5.0) as usize;
    let face_seed = (seed * 5.0).fract();

    if face < 4 {
        let c1 = base_corners[face];
        let c2 = base_corners[(face + 1) % 4];

        let r = face_seed.sqrt();
        let s = v;
        let t = 1.0 - r;
        let w = r * (1.0 - s);
        let u_tri = r * s;

        let x = t * apex[0] + w * c1[0] + u_tri * c2[0];
        let y = t * apex[1] + w * c1[1] + u_tri * c2[1];
        let z = t * apex[2] + w * c1[2] + u_tri * c2[2];

        let edge1 = [c1[0] - apex[0], c1[1] - apex[1], c1[2] - apex[2]];
        let edge2 = [c2[0] - apex[0], c2[1] - apex[1], c2[2] - apex[2]];

        let nx = -(edge1[1] * edge2[2] - edge1[2] * edge2[1]);
        let ny = -(edge1[2] * edge2[0] - edge1[0] * edge2[2]);
        let nz = -(edge1[0] * edge2[1] - edge1[1] * edge2[0]);
        let len = (nx * nx + ny * ny + nz * nz).sqrt();

        Point3D::new(
            x + (nx / len) * spike,
            y + (ny / len) * spike,
            z + (nz / len) * spike,
        )
    } else {
        let c1 = base_corners[0];
        let c2 = base_corners[1];
        let c3 = base_corners[2];
        let c4 = base_corners[3];

        let x = (1.0 - u) * (1.0 - v) * c1[0]
            + u * (1.0 - v) * c2[0]
            + u * v * c3[0]
            + (1.0 - u) * v * c4[0];
        let z = (1.0 - u) * (1.0 - v) * c1[2]
            + u * (1.0 - v) * c2[2]
            + u * v * c3[2]
            + (1.0 - u) * v * c4[2];

        Point3D::new(x, -h - spike, z)
    }
}

/// Draw the 3D pyramid visualization.
///
/// Renders numbers distributed across 4 triangular faces and the base of a pyramid.
/// Highlighted numbers (primes, Fibonacci, etc.) spike outward from the faces.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    draw_3d_scene(app, ui, rect, "pyramid_3d", |n, is_highlighted| {
        let t = (n - 1) as f32;
        let seed = t / max_n as f32;
        let u = (t * golden_ratio).fract();
        let v = (t * golden_ratio * golden_ratio).fract();

        let spike = if is_highlighted { 12.0 } else { 0.0 };
        point_on_pyramid_surface(seed, u, v, spike)
    });
}
