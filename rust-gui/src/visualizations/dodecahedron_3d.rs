//! 3D Dodecahedron visualization - numbers distributed on 12 pentagonal faces
//! Highlighted numbers bulge outward from the surface

use crate::draw_number::get_prime_pair_color;
use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const SCALE: f32 = 70.0;

fn dodecahedron_vertices() -> Vec<[f32; 3]> {
    let phi = (1.0 + 5.0f32.sqrt()) / 2.0;
    let inv_phi = 1.0 / phi;

    vec![
        [1.0, 1.0, 1.0],
        [1.0, 1.0, -1.0],
        [1.0, -1.0, 1.0],
        [1.0, -1.0, -1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [0.0, inv_phi, phi],
        [0.0, inv_phi, -phi],
        [0.0, -inv_phi, phi],
        [0.0, -inv_phi, -phi],
        [inv_phi, phi, 0.0],
        [inv_phi, -phi, 0.0],
        [-inv_phi, phi, 0.0],
        [-inv_phi, -phi, 0.0],
        [phi, 0.0, inv_phi],
        [phi, 0.0, -inv_phi],
        [-phi, 0.0, inv_phi],
        [-phi, 0.0, -inv_phi],
    ]
}

fn dodecahedron_faces() -> Vec<[usize; 5]> {
    vec![
        [0, 8, 10, 2, 16],
        [0, 16, 17, 1, 12],
        [0, 12, 14, 4, 8],
        [1, 17, 3, 11, 9],
        [1, 9, 5, 14, 12],
        [2, 10, 6, 15, 13],
        [2, 13, 3, 17, 16],
        [3, 13, 15, 7, 11],
        [4, 14, 5, 19, 18],
        [4, 18, 6, 10, 8],
        [5, 9, 11, 7, 19],
        [6, 18, 19, 7, 15],
    ]
}

fn point_on_pentagon(
    vertices: &[[f32; 3]],
    face: &[usize; 5],
    r: f32,
    theta: f32,
    spike: f32,
) -> Point3D {
    let v0 = vertices[face[0]];
    let v1 = vertices[face[1]];
    let v2 = vertices[face[2]];
    let v3 = vertices[face[3]];
    let v4 = vertices[face[4]];

    let center = [
        (v0[0] + v1[0] + v2[0] + v3[0] + v4[0]) / 5.0,
        (v0[1] + v1[1] + v2[1] + v3[1] + v4[1]) / 5.0,
        (v0[2] + v1[2] + v2[2] + v3[2] + v4[2]) / 5.0,
    ];

    let mut corners = [[0.0f32; 3]; 5];
    for i in 0..5 {
        let vi = vertices[face[i]];
        corners[i] = [vi[0] - center[0], vi[1] - center[1], vi[2] - center[2]];
    }

    let sector = ((theta / (2.0 * std::f32::consts::PI)) * 5.0).floor() as usize % 5;
    let sector_theta = theta - sector as f32 * 2.0 * std::f32::consts::PI / 5.0;
    let angle_norm = sector_theta / (2.0 * std::f32::consts::PI / 5.0);

    let c1 = corners[sector];
    let c2 = corners[(sector + 1) % 5];

    let x = center[0] + r * (c1[0] * (1.0 - angle_norm) + c2[0] * angle_norm);
    let y = center[1] + r * (c1[1] * (1.0 - angle_norm) + c2[1] * angle_norm);
    let z = center[2] + r * (c1[2] * (1.0 - angle_norm) + c2[2] * angle_norm);

    let len = (x * x + y * y + z * z).sqrt();
    let normal = [x / len, y / len, z / len];

    Point3D::new(
        (x + normal[0] * spike) * SCALE,
        (y + normal[1] * spike) * SCALE,
        (z + normal[2] * spike) * SCALE,
    )
}

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("dodecahedron_3d"), egui::Sense::drag());

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
    let vertices = dodecahedron_vertices();
    let faces = dodecahedron_faces();
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let t = (n - 1) as f32;
        let face_idx = ((n - 1) * 12 / max_n) % 12;

        let local = (t * golden_ratio).fract();
        let r = local.sqrt() * 0.9;
        let theta = (local * golden_ratio * 5.0).fract() * std::f32::consts::TAU;

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 0.15 } else { 0.0 };

        let point = point_on_pentagon(&vertices, &faces[face_idx], r, theta, spike);
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
