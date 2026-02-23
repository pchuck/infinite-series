//! 3D Icosahedron visualization - numbers distributed on 20 triangular faces
//! Highlighted numbers bulge outward from the surface

use crate::helpers::MARGIN_SMALL;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use eframe::egui;

const SCALE: f32 = 80.0;

fn icosahedron_vertices() -> Vec<[f32; 3]> {
    let phi = (1.0 + 5.0f32.sqrt()) / 2.0;

    vec![
        [0.0, 1.0, phi],
        [0.0, 1.0, -phi],
        [0.0, -1.0, phi],
        [0.0, -1.0, -phi],
        [1.0, phi, 0.0],
        [1.0, -phi, 0.0],
        [-1.0, phi, 0.0],
        [-1.0, -phi, 0.0],
        [phi, 0.0, 1.0],
        [phi, 0.0, -1.0],
        [-phi, 0.0, 1.0],
        [-phi, 0.0, -1.0],
    ]
}

fn icosahedron_faces() -> Vec<[usize; 3]> {
    vec![
        [0, 2, 8],
        [0, 8, 4],
        [0, 4, 6],
        [0, 6, 10],
        [0, 10, 2],
        [1, 9, 3],
        [1, 4, 9],
        [1, 6, 4],
        [1, 11, 6],
        [1, 3, 11],
        [2, 5, 8],
        [2, 7, 5],
        [2, 10, 7],
        [3, 5, 7],
        [3, 7, 11],
        [4, 8, 9],
        [5, 9, 8],
        [6, 11, 10],
        [7, 10, 11],
        [9, 5, 3],
    ]
}

fn point_on_triangle(
    vertices: &[[f32; 3]],
    face: &[usize; 3],
    u: f32,
    v: f32,
    spike: f32,
) -> Point3D {
    let a = vertices[face[0]];
    let b = vertices[face[1]];
    let c = vertices[face[2]];

    let w = 1.0 - u - v;

    let x = w * a[0] + u * b[0] + v * c[0];
    let y = w * a[1] + u * b[1] + v * c[1];
    let z = w * a[2] + u * b[2] + v * c[2];

    let len = (x * x + y * y + z * z).sqrt();
    let normal = [x / len, y / len, z / len];

    Point3D::new(
        (x + normal[0] * spike) * SCALE,
        (y + normal[1] * spike) * SCALE,
        (z + normal[2] * spike) * SCALE,
    )
}

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("icosahedron_3d"), egui::Sense::drag());

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
    let vertices = icosahedron_vertices();
    let faces = icosahedron_faces();
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    let mut projected: Vec<(f32, f32, f32, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let t = (n - 1) as f32;
        let face_idx = ((n - 1) * 20 / max_n) % 20;
        let local = (t * golden_ratio).fract();

        let u = local * 0.9 + 0.05;
        let v = (local * golden_ratio).fract() * 0.9 + 0.05;

        let (u, v) = if u + v > 0.95 {
            (0.95 - u, 0.95 - v)
        } else {
            (u, v)
        };

        let is_highlighted = highlights.contains(&n);
        let spike = if is_highlighted { 0.2 } else { 0.0 };

        let point = point_on_triangle(&vertices, &faces[face_idx], u, v, spike);
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
