//! 3D Trefoil Knot visualization - numbers along a mathematical knot
//! Highlighted numbers bulge outward from the knot tube

use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const KNOT_RADIUS: f32 = 80.0;
const TUBE_RADIUS: f32 = 20.0;

fn trefoil_point(t: f32) -> (f32, f32, f32) {
    let angle = t * std::f32::consts::TAU;

    let x = angle.sin() + (2.0 * angle).sin() / 2.0;
    let y = angle.cos() - (2.0 * angle).cos() / 2.0;
    let z = -(3.0 * angle).sin() / 2.0;

    (x * KNOT_RADIUS, y * KNOT_RADIUS, z * KNOT_RADIUS)
}

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("trefoil_3d"), egui::Sense::drag());

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

    let mut projected: Vec<(f32, f32, f32, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
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

        let tube_r = if highlights.contains(&n) {
            TUBE_RADIUS + 8.0
        } else {
            TUBE_RADIUS
        };

        let x = kx + tube_r * (phi.cos() * bx + phi.sin() * tx);
        let y = ky + tube_r * (phi.cos() * by + phi.sin() * ty);
        let z = kz + tube_r * (phi.cos() * bz + phi.sin() * tz);

        let point = Point3D::new(x, y, z);
        let (px, py, pz) = project_3d_to_2d(&point, rotation_y, rotation_x);

        projected.push((px, py, pz, highlights.contains(&n)));
    }

    projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut max_coord = 0.0f32;
    for (x, y, _, _) in &projected {
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

    for (x, y, depth, is_highlighted) in &projected {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;
        let df = depth_factor(*depth);

        if *is_highlighted {
            let size = (app.config.highlight_size as f32 * df) / 2.0;
            let color = adjust_brightness(app.config.highlight_color, df);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        } else if app.config.non_highlight_size > 0 {
            let size = (app.config.non_highlight_size as f32 * df) / 2.0;
            let color = adjust_brightness(app.config.non_highlight_color, df);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        }
    }
}
