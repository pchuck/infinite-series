//! 3D Pyramid visualization - evenly distributed point cloud on all faces
//! Highlighted numbers spike outward from the pyramid faces

use crate::helpers::MARGIN_SMALL;
use eframe::egui;

const PYRAMID_HEIGHT: f32 = 150.0;
const PYRAMID_BASE: f32 = 120.0;
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

    let slant = (half * half + PYRAMID_HEIGHT * PYRAMID_HEIGHT).sqrt();
    let tri_area = 0.5 * PYRAMID_BASE * slant / 2.0f32.sqrt() * 2.0;
    let base_area = PYRAMID_BASE * PYRAMID_BASE;
    let _total_area = 4.0 * tri_area + base_area;
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

        Point3D {
            x: x + (nx / len) * spike,
            y: y + (ny / len) * spike,
            z: z + (nz / len) * spike,
        }
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

        Point3D {
            x,
            y: -h - spike,
            z,
        }
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

    let mut projected: Vec<(f32, f32, f32, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let t = (n - 1) as f32;

        let seed = t / max_n as f32;
        let u = (t * golden_ratio).fract();
        let v = (t * golden_ratio * golden_ratio).fract();

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 12.0 } else { 0.0 };

        let point = point_on_pyramid_surface(seed, u, v, spike);
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
