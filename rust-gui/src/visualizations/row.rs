//! Row visualization

use crate::draw_number::draw_number;
use crate::helpers::HOVER_THRESHOLD_DEFAULT;
use crate::helpers::MARGIN_SMALL;
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    (1..=max_n).map(|n| (n, n as f32, 0.0)).collect()
}

pub fn draw(app: &crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let positions = generate_positions(app.config.max_number);

    if positions.is_empty() {
        return;
    }

    let max_x = app.config.max_number as f32;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let scale = available_width / max_x;

    // Center horizontally
    let row_width = max_x * scale;
    let offset_x = (available_width - row_width) / 2.0;

    let center_y = rect.center().y;
    let start_x = rect.left() + MARGIN_SMALL + offset_x + scale / 2.0;
    let painter = ui.painter();

    for (n, x, _) in &positions {
        let screen_x = start_x + *x * scale;
        draw_number(
            *n,
            screen_x,
            center_y,
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
) -> Option<usize> {
    let positions = generate_positions(app.config.max_number);
    if positions.is_empty() {
        return None;
    }

    let max_x = app.config.max_number as f32;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let scale = available_width / max_x;

    // Center horizontally (same as draw)
    let row_width = max_x * scale;
    let offset_x = (available_width - row_width) / 2.0;

    let center_y = rect.center().y;
    let start_x = rect.left() + MARGIN_SMALL + offset_x + scale / 2.0;

    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, _) in &positions {
        let screen_x = start_x + *x * scale;

        let dx = mouse_pos.x - screen_x;
        let dy = mouse_pos.y - center_y;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq < min_distance_sq && distance_sq < (scale * HOVER_THRESHOLD_DEFAULT).powi(2)
        {
            min_distance_sq = distance_sq;
            closest_n = Some(*n);
        }
    }

    closest_n
}
