//! 3D Sphere visualization - numbers distributed on sphere surface
//! Highlighted numbers bulge outward from the surface

use crate::draw_number::get_prime_pair_color;
use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const SPHERE_RADIUS: f32 = 100.0;

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
    let response = ui.interact(rect, egui::Id::new("sphere_3d"), egui::Sense::drag());

    if response.dragged() {
        let delta = response.drag_delta();
        let (mut rotation_x, mut rotation_y) = app.get_rotation();
        rotation_y -= delta.x * DRAG_SENSITIVITY;
        rotation_x -= delta.y * DRAG_SENSITIVITY;
        rotation_x = rotation_x.clamp(-1.5, 1.5);
        app.set_rotation(rotation_x, rotation_y);
    }

    let (rotation_x, rotation_y) = app.get_rotation();

    let max_n = app.config.max_number;
    if max_n == 0 {
        return;
    }

    let highlights = app.highlights();

    let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let (nx, ny, nz) = fibonacci_sphere_point(n - 1, max_n);

        let is_highlighted = highlights.contains(&n);
        let radius = if is_highlighted {
            SPHERE_RADIUS + 15.0
        } else {
            SPHERE_RADIUS
        };

        let point = Point3D::new(nx * radius, ny * radius, nz * radius);
        let (px, py, pz) = project_3d_to_2d(&point, rotation_y, rotation_x);

        projected.push((px, py, pz, n, is_highlighted));
    }

    projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut max_coord = 0.0f32;
    for (x, y, _, _, _) in &projected {
        max_coord = max_coord.max(x.abs()).max(y.abs());
    }

    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_coord > 0.0 {
        available / max_coord
    } else {
        1.0
    };

    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let painter = ui.painter();

    for (x, y, depth, n, is_highlighted) in &projected {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;
        let df = depth_factor(*depth);

        if *is_highlighted {
            let size = (app.config.highlight_size as f32 * df) / 2.0;
            let base_color = get_prime_pair_color(*n, highlights, &app.config, app.series_type)
                .unwrap_or(app.config.highlight_color);
            let color = adjust_brightness(base_color, df);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        } else if app.config.non_highlight_size > 0 {
            let size = (app.config.non_highlight_size as f32 * df) / 2.0;
            let color = adjust_brightness(app.config.non_highlight_color, df);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        }
    }
}
