//! Grid visualization

use crate::draw_number::draw_number;
use crate::helpers::{
    find_hovered_offset_based, LayoutData, HOVER_THRESHOLD_DEFAULT, MARGIN_SMALL,
};
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    let side = (max_n as f32).sqrt() as usize + 1;
    (1..=max_n)
        .map(|n| {
            let row = (n - 1) / side;
            let col = (n - 1) % side;
            (n, col as f32, row as f32)
        })
        .collect()
}

/// Compute layout for grid visualization.
///
/// Returns: (start_x, start_y, scale)
/// - start_x, start_y: Top-left corner of the grid
/// - scale: Pixels per cell
pub fn compute_layout(
    _positions: &[(usize, f32, f32)],
    rect: egui::Rect,
    max_n: usize,
) -> (f32, f32, f32) {
    let side = (max_n as f32).sqrt() as usize + 1;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let available_height = rect.height() - 2.0 * MARGIN_SMALL;

    let scale = available_width.min(available_height) / side as f32;

    let grid_width = side as f32 * scale;
    let grid_height = side as f32 * scale;
    let offset_x = (available_width - grid_width) / 2.0;
    let offset_y = (available_height - grid_height) / 2.0;

    let start_x = rect.left() + MARGIN_SMALL + offset_x + scale / 2.0;
    let start_y = rect.top() + MARGIN_SMALL + offset_y + scale / 2.0;

    (start_x, start_y, scale)
}

pub fn draw(
    app: &crate::app::NumberVisualizerApp,
    ui: &mut egui::Ui,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) {
    if positions.is_empty() {
        return;
    }

    let (start_x, start_y, scale) = compute_layout(positions, rect, app.config.max_number);
    let painter = ui.painter();

    for (n, x, y) in positions {
        let screen_x = start_x + *x * scale;
        let screen_y = start_y + *y * scale;
        draw_number(
            *n,
            screen_x,
            screen_y,
            painter,
            app.highlights(),
            &app.config,
            app.series_type,
        );
    }
}

pub fn find_hovered(
    app: &crate::app::NumberVisualizerApp,
    mouse_pos: egui::Pos2,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let layout: LayoutData = compute_layout(positions, rect, app.config.max_number);
    find_hovered_offset_based(mouse_pos, positions, layout, HOVER_THRESHOLD_DEFAULT)
}
