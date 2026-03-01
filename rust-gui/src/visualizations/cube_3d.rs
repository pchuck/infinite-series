//! 3D Cube visualization - numbers distributed on cube surface
//! Highlighted numbers bulge outward from the faces

use crate::draw_number::get_prime_pair_color;
use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const CUBE_SIZE: f32 = 80.0;

/// Calculate a point on a cube face.
///
/// Returns a 3D point on one of the 6 cube faces based on face index and UV coordinates.
fn cube_face_point(face: usize, u: f32, v: f32, spike: f32) -> Point3D {
    let half = CUBE_SIZE / 2.0 + spike;
    match face % 6 {
        0 => Point3D::new(u * half, half, v * half),
        1 => Point3D::new(u * half, -half, v * half),
        2 => Point3D::new(half, u * half, v * half),
        3 => Point3D::new(-half, u * half, v * half),
        4 => Point3D::new(u * half, v * half, half),
        _ => Point3D::new(u * half, v * half, -half),
    }
}

/// Draw the 3D cube visualization.
///
/// Renders numbers distributed on the 6 faces of a cube.
/// Highlighted numbers (primes, Fibonacci, etc.) bulge outward from the faces.
/// Supports mouse drag for rotation.
pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("cube_3d"), egui::Sense::drag());

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
    let points_per_face = (max_n / 6).max(1);
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let t = (n - 1) as f32;
        let face = ((n - 1) * 6 / max_n) % 6;
        let local_t = (t % points_per_face as f32) / points_per_face as f32;

        let u = (local_t * golden_ratio).fract() * 2.0 - 1.0;
        let v = (local_t * golden_ratio * golden_ratio).fract() * 2.0 - 1.0;

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 12.0 } else { 0.0 };

        let point = cube_face_point(face, u, v, spike);
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
