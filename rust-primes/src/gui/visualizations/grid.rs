//! Grid visualization

use crate::gui::draw_number::draw_number;
use crate::gui::HOVER_THRESHOLD_DEFAULT;
use crate::gui::MARGIN_SMALL;
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

pub fn draw(app: &crate::gui::app::PrimeVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let positions = generate_positions(app.config.max_number);

    if positions.is_empty() {
        return;
    }

    let side = (app.config.max_number as f32).sqrt() as usize + 1;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let available_height = rect.height() - 2.0 * MARGIN_SMALL;

    let scale = available_width.min(available_height) / side as f32;

    let start_x = rect.left() + MARGIN_SMALL + scale / 2.0;
    let start_y = rect.top() + MARGIN_SMALL + scale / 2.0;
    let painter = ui.painter();

    for (n, x, y) in &positions {
        let screen_x = start_x + *x * scale;
        let screen_y = start_y + *y * scale;
        draw_number(*n, screen_x, screen_y, painter, &app.primes, &app.config);
    }
}

pub fn find_hovered(
    app: &crate::gui::app::PrimeVisualizerApp,
    mouse_pos: egui::Pos2,
    rect: egui::Rect,
) -> Option<usize> {
    let positions = generate_positions(app.config.max_number);
    if positions.is_empty() {
        return None;
    }

    let side = (app.config.max_number as f32).sqrt() as usize + 1;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let available_height = rect.height() - 2.0 * MARGIN_SMALL;
    let scale = available_width.min(available_height) / side as f32;

    let start_x = rect.left() + MARGIN_SMALL + scale / 2.0;
    let start_y = rect.top() + MARGIN_SMALL + scale / 2.0;

    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in &positions {
        let screen_x = start_x + *x * scale;
        let screen_y = start_y + *y * scale;

        let dx = mouse_pos.x - screen_x;
        let dy = mouse_pos.y - screen_y;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq < min_distance_sq && distance_sq < (scale * HOVER_THRESHOLD_DEFAULT).powi(2)
        {
            min_distance_sq = distance_sq;
            closest_n = Some(*n);
        }
    }

    closest_n
}
