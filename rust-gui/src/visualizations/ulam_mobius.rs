//! Ulam Mobius spiral visualization

use crate::draw_number::draw_number;
use crate::helpers::{
    calculate_bounds, calculate_scale, gap_color, gap_stroke_width, MARGIN_SMALL,
};
use crate::types::SeriesType;
use crate::visualizations::ulam::generate_positions as generate_ulam_positions;
use eframe::egui;

pub fn draw(app: &crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    if app.primes_vec.len() < 2 {
        return;
    }

    let spiral_positions = generate_ulam_positions(app.primes_vec.len());
    let positions: Vec<(usize, f32, f32)> = app
        .primes_vec
        .iter()
        .enumerate()
        .map(|(idx, &n)| {
            let (_, x, y) = spiral_positions[idx];
            (n, x, y)
        })
        .collect();

    let (min_x, max_x, min_y, max_y) = calculate_bounds(&positions);
    let range_x = max_x - min_x;
    let range_y = max_y - min_y;

    let scale = calculate_scale(rect, range_x, range_y, MARGIN_SMALL);

    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let painter = ui.painter();

    for i in 0..positions.len() - 1 {
        let (_, x1, y1) = positions[i];
        let (_, x2, y2) = positions[i + 1];
        let gap = positions[i + 1].0 - positions[i].0;

        let screen_x1 = center_x + (x1 - (min_x + max_x) / 2.0) * scale;
        let screen_y1 = center_y - (y1 - (min_y + max_y) / 2.0) * scale;
        let screen_x2 = center_x + (x2 - (min_x + max_x) / 2.0) * scale;
        let screen_y2 = center_y - (y2 - (min_y + max_y) / 2.0) * scale;

        let color = gap_color(gap);
        let stroke_width = gap_stroke_width(gap);

        painter.line_segment(
            [
                egui::Pos2::new(screen_x1, screen_y1),
                egui::Pos2::new(screen_x2, screen_y2),
            ],
            egui::Stroke::new(stroke_width, color),
        );
    }

    for (n, x, y) in &positions {
        let screen_x = center_x + (*x - (min_x + max_x) / 2.0) * scale;
        let screen_y = center_y - (*y - (min_y + max_y) / 2.0) * scale;
        draw_number(
            *n,
            screen_x,
            screen_y,
            painter,
            &app.primes,
            &app.config,
            SeriesType::Primes,
        );
    }
}
