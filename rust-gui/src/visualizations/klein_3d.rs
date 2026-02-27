//! 3D Klein Bottle visualization - numbers on an immersed Klein bottle
//! Highlighted numbers bulge outward from the surface

use crate::draw_number::get_prime_pair_color;
use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const BOTTLE_RADIUS: f32 = 60.0;

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("klein_3d"), egui::Sense::drag());

    if response.dragged() {
        let delta = response.drag_delta();
        app.helix_rotation_y -= delta.x * DRAG_SENSITIVITY;
        app.helix_rotation_x -= delta.y * DRAG_SENSITIVITY;
        app.helix_rotation_x = app.helix_rotation_x.clamp(-1.5, 1.5);
    }

    let rotation_y = app.helix_rotation_y;
    let rotation_x = app.helix_rotation_x;

    let max_n = app.config.max_number;
    if max_n == 0 {
        return;
    }

    let highlights = app.highlights();
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let t = (n - 1) as f32 / max_n as f32;
        let u = t * std::f32::consts::TAU * 2.0;
        let v = (n as f32 * golden_ratio).fract() * std::f32::consts::TAU;

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 8.0 } else { 0.0 };

        let r = 4.0 - 2.0 * u.cos();
        let x = (r * v.cos() + 4.0) * u.cos() + spike * v.cos() * u.cos();
        let y = r * v.sin() + spike * v.sin();
        let z = (r * v.cos() + 4.0) * u.sin() + spike * v.cos() * u.sin();

        let x = x * BOTTLE_RADIUS / 4.0;
        let y = y * BOTTLE_RADIUS / 4.0;
        let z = z * BOTTLE_RADIUS / 4.0;

        let point = Point3D::new(x, y, z);
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
