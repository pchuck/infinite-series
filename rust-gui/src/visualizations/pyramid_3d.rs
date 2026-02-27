//! 3D Pyramid visualization - evenly distributed point cloud on all faces
//! Highlighted numbers spike outward from the pyramid faces

use crate::draw_number::get_prime_pair_color;
use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const PYRAMID_HEIGHT: f32 = 150.0;
const PYRAMID_BASE: f32 = 120.0;

fn point_on_pyramid_surface(seed: f32, u: f32, v: f32, spike: f32) -> Point3D {
    let half = PYRAMID_BASE / 2.0;
    let h = PYRAMID_HEIGHT / 2.0;

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

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("pyramid_3d"), egui::Sense::drag());

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
        let t = (n - 1) as f32;

        let seed = t / max_n as f32;
        let u = (t * golden_ratio).fract();
        let v = (t * golden_ratio * golden_ratio).fract();

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 12.0 } else { 0.0 };

        let point = point_on_pyramid_surface(seed, u, v, spike);
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
