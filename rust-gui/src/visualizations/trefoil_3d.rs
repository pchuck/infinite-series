//! 3D Trefoil Knot visualization - numbers along a mathematical knot
//! Highlighted numbers bulge outward from the knot tube

use crate::constants::shapes;
use crate::visualizations::shared_3d::{draw_3d_scene, Point3D};
use eframe::egui;

/// Calculate a point on the trefoil knot curve.
///
/// Uses a parametric equation to trace the trefoil knot path.
fn trefoil_point(t: f32) -> (f32, f32, f32) {
    let angle = t * std::f32::consts::TAU;

    let x = angle.sin() + (2.0 * angle).sin() / 2.0;
    let y = angle.cos() - (2.0 * angle).cos() / 2.0;
    let z = -(3.0 * angle).sin() / 2.0;

    (
        x * shapes::KNOT_RADIUS,
        y * shapes::KNOT_RADIUS,
        z * shapes::KNOT_RADIUS,
    )
}

/// Draw the 3D trefoil knot visualization.
///
/// Renders numbers along the path of a trefoil knot (the simplest non-trivial mathematical knot).
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the knot tube.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    draw_3d_scene(app, ui, rect, "trefoil_3d", |n, is_highlighted| {
        let t = (n - 1) as f32 / max_n as f32;
        let phi = (n as f32 * golden_ratio).fract() * std::f32::consts::TAU;

        let (kx, ky, kz) = trefoil_point(t);

        let angle = t * std::f32::consts::TAU;
        let tx = (angle + std::f32::consts::FRAC_PI_2).cos();
        let ty = (angle + std::f32::consts::FRAC_PI_2).sin();
        let tz = 0.3 * (3.0 * angle).cos();

        let tx_len = (tx * tx + ty * ty + tz * tz).sqrt();
        let tx = tx / tx_len;
        let ty = ty / tx_len;
        let tz = tz / tx_len;

        let bx = -tz;
        let by = 0.0;
        let bz = -tx;

        let bx_len = (bx * bx + by * by + bz * bz).sqrt();
        let bx = bx / bx_len;
        let by = by / bx_len;
        let bz = bz / bx_len;

        let tube_r = if is_highlighted {
            shapes::KNOT_TUBE_RADIUS + 8.0
        } else {
            shapes::KNOT_TUBE_RADIUS
        };

        let x = kx + tube_r * (phi.cos() * bx + phi.sin() * tx);
        let y = ky + tube_r * (phi.cos() * by + phi.sin() * ty);
        let z = kz + tube_r * (phi.cos() * bz + phi.sin() * tz);

        Point3D::new(x, y, z)
    });
}
