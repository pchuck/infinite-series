//! Prime density gradient visualization

use crate::helpers::MARGIN_SMALL;
use eframe::egui;

pub fn draw(app: &crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    if app.primes_vec.is_empty() {
        return;
    }

    let margin = MARGIN_SMALL;
    let graph_left = rect.left() + margin;
    let graph_right = rect.right() - margin;
    let graph_top = rect.top() + margin;
    let graph_bottom = rect.bottom() - margin;
    let graph_width = graph_right - graph_left;
    let graph_height = graph_bottom - graph_top;

    let painter = ui.painter();
    painter.rect_filled(
        egui::Rect::from_min_size(
            egui::Pos2::new(graph_left, graph_top),
            egui::vec2(graph_width, graph_height),
        ),
        0.0,
        app.config.background_color,
    );

    let grid_size = app.config.grid_size;
    let cell_width = graph_width / grid_size as f32;
    let cell_height = graph_height / grid_size as f32;

    let mut density_grid = vec![0.0_f32; grid_size * grid_size];

    for &p in &app.primes_vec {
        let x_frac = p as f32 / app.config.max_number as f32;
        let y_frac = (p * p % app.config.max_number) as f32 / app.config.max_number as f32;

        let grid_x = ((x_frac * grid_size as f32) as usize).min(grid_size - 1);
        let grid_y = ((y_frac * grid_size as f32) as usize).min(grid_size - 1);

        let idx = grid_y * grid_size + grid_x;
        density_grid[idx] += 1.0;
    }

    let max_density = density_grid.iter().cloned().fold(0.0_f32, f32::max);

    for gy in 0..grid_size {
        for gx in 0..grid_size {
            let idx = gy * grid_size + gx;
            let density = density_grid[idx];
            let normalized = if max_density > 0.0 {
                density / max_density
            } else {
                0.0
            };

            let r = (app.config.highlight_color.r() as f32 * normalized) as u8;
            let g = (app.config.highlight_color.g() as f32 * normalized) as u8;
            let b = (app.config.highlight_color.b() as f32 * normalized) as u8;

            let color = egui::Color32::from_rgba_unmultiplied(r, g, b, 255);

            let x = graph_left + gx as f32 * cell_width;
            let y = graph_top + gy as f32 * cell_height;

            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::Pos2::new(x, y),
                    egui::vec2(cell_width, cell_height),
                ),
                0.0,
                color,
            );
        }
    }
}
