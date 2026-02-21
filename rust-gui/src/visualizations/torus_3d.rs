//! 3D Torus visualization - numbers wrapped around a donut shape
//! Highlighted numbers bulge outward from the torus surface

use crate::helpers::MARGIN_SMALL;
use eframe::egui;

const TORUS_MAJOR_RADIUS: f32 = 80.0;
const TORUS_MINOR_RADIUS: f32 = 30.0;
const DRAG_SENSITIVITY: f32 = 0.01;

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

fn project_3d_to_2d(point: &Point3D, rotation_y: f32, rotation_x: f32) -> (f32, f32, f32) {
    let cos_y = rotation_y.cos();
    let sin_y = rotation_y.sin();
    let x1 = point.x * cos_y - point.z * sin_y;
    let z1 = point.x * sin_y + point.z * cos_y;
    let y1 = point.y;

    let cos_x = rotation_x.cos();
    let sin_x = rotation_x.sin();
    let y2 = y1 * cos_x - z1 * sin_x;
    let z2 = y1 * sin_x + z1 * cos_x;

    let perspective = 500.0;
    let scale = perspective / (perspective + z2 + 300.0);

    (x1 * scale, y2 * scale, z2)
}

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("torus_3d"), egui::Sense::drag());

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
        let t = (n - 1) as f32;
        let u = t / max_n as f32 * std::f32::consts::TAU * golden_ratio;
        let v = t * golden_ratio / max_n as f32 * std::f32::consts::TAU;

        let is_highlighted = highlights.contains(&n);
        let minor_r = if is_highlighted {
            TORUS_MINOR_RADIUS + 10.0
        } else {
            TORUS_MINOR_RADIUS
        };

        let x = (TORUS_MAJOR_RADIUS + minor_r * v.cos()) * u.cos();
        let y = minor_r * v.sin();
        let z = (TORUS_MAJOR_RADIUS + minor_r * v.cos()) * u.sin();

        let point = Point3D { x, y, z };
        let (px, py, pz) = project_3d_to_2d(&point, rotation_y, rotation_x);

        projected.push((px, py, pz, is_highlighted));
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

        let depth_factor = (*depth + 300.0) / 600.0;
        let depth_factor = depth_factor.clamp(0.3, 1.0);

        if *is_highlighted {
            let size = (app.config.highlight_size as f32 * depth_factor) / 2.0;
            let color = adjust_brightness(app.config.highlight_color, depth_factor);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        } else if app.config.non_highlight_size > 0 {
            let size = (app.config.non_highlight_size as f32 * depth_factor) / 2.0;
            let color = adjust_brightness(app.config.non_highlight_color, depth_factor);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        }
    }
}

fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = (color.r() as f32 * factor).min(255.0) as u8;
    let g = (color.g() as f32 * factor).min(255.0) as u8;
    let b = (color.b() as f32 * factor).min(255.0) as u8;
    egui::Color32::from_rgb(r, g, b)
}
