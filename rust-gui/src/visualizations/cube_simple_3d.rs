//! 3D Cube Simple visualization
//! Evenly distributes points on all six faces of a cube

use crate::app::NumberVisualizerApp;
use crate::helpers::MARGIN_SMALL;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D, DRAG_SENSITIVITY,
};
use crate::visualizations::traits::Visualizer;
use eframe::egui;

const CUBE_SIZE: f32 = 80.0;

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

pub fn draw(app: &mut crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let response = ui.interact(rect, egui::Id::new("cube_simple_3d"), egui::Sense::drag());

    if response.dragged() {
        let delta = response.drag_delta();
        let (mut rotation_x, mut rotation_y) = app.get_rotation();
        rotation_y -= delta.x * DRAG_SENSITIVITY;
        rotation_x -= delta.y * DRAG_SENSITIVITY;
        rotation_x = rotation_x.clamp(-1.5, 1.5);
        app.set_rotation(rotation_x, rotation_y);
    }

    let (rot_x, rot_y) = app.get_rotation();
    let rotation_y = rot_y;
    let rotation_x = rot_x;

    let max_n = app.config.max_number;
    if max_n == 0 {
        return;
    }

    let highlights = app.highlights();

    let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        let face = ((n * 9973) / max_n) % 6;
        let u: f32 = ((n * 17 % 1000) as f32 / 1000.0) * 2.0 - 1.0;
        let v: f32 = ((n * 23 % 1000) as f32 / 1000.0) * 2.0 - 1.0;

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

    for (x, y, depth, _n, is_highlighted) in &projected {
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

pub struct CubeSimple3D;

impl Visualizer for CubeSimple3D {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::CubeSimple3D
    }

    fn name(&self) -> &'static str {
        "3D Cube Simple"
    }

    fn description(&self) -> &'static str {
        VisualizationType::CubeSimple3D.description()
    }

    fn supports_series(&self, _series: SeriesType) -> bool {
        true
    }

    fn supports_hover(&self) -> bool {
        false
    }

    fn uses_point_rendering(&self) -> bool {
        true
    }

    fn generate_positions(&self, _max_n: usize, _params: &VizParams) -> Vec<(usize, f32, f32)> {
        Vec::new()
    }

    fn draw(
        &self,
        app: &mut NumberVisualizerApp,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        _positions: &[(usize, f32, f32)],
    ) {
        draw(app, ui, rect);
    }
}
