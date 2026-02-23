//! 3D Cylinder visualization - numbers spiral around a cylinder
//! Highlighted numbers spike outward from the cylinder surface

use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const CYLINDER_HEIGHT: f32 = 200.0;
const CYLINDER_RADIUS: f32 = 80.0;
const CYLINDER_TURNS: f32 = 8.0;

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("cylinder_3d"), egui::Sense::drag());

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

    let mut projected: Vec<(f32, f32, f32, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let t = (n - 1) as f32 / max_n as f32;
        let angle = t * CYLINDER_TURNS * std::f32::consts::TAU;
        let height = t * CYLINDER_HEIGHT - CYLINDER_HEIGHT / 2.0;

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 15.0 } else { 0.0 };
        let r = CYLINDER_RADIUS + spike;

        let x = r * angle.cos();
        let z = r * angle.sin();

        let point = Point3D::new(x, height, z);
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
