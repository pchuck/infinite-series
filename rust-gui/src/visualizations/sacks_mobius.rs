//! Sacks Mobius spiral visualization

use crate::draw_number::draw_number;
use crate::helpers::{
    gap_color, gap_stroke_width, MARGIN_SMALL, SACKS_MOBIUS_RADIUS_MULTIPLIER,
    SACKS_THETA_MULTIPLIER,
};
use crate::types::SeriesType;
use eframe::egui;

/// Draw the Sacks Mobius spiral visualization.
///
/// Renders prime numbers on an Archimedean spiral with lines colored by gap size.
/// Larger prime gaps produce darker, thinner lines to visualize prime distribution patterns.
pub fn draw(app: &crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    if app.primes_vec().len() < 2 {
        return;
    }

    let positions: Vec<(usize, f32, f32)> = app
        .primes_vec()
        .iter()
        .enumerate()
        .map(|(idx, &n)| {
            let idx_f = idx as f32;
            let r = idx_f * SACKS_MOBIUS_RADIUS_MULTIPLIER;
            let theta = idx_f * SACKS_THETA_MULTIPLIER;
            let x = r * theta.cos();
            let y = r * theta.sin();
            (n, x, y)
        })
        .collect();

    let mut max_coord = 0.0_f32;
    for (_, x, y) in &positions {
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

    for i in 0..positions.len() - 1 {
        let (_, x1, y1) = positions[i];
        let (_, x2, y2) = positions[i + 1];
        let gap = positions[i + 1].0 - positions[i].0;

        let screen_x1 = center_x + x1 * scale;
        let screen_y1 = center_y - y1 * scale;
        let screen_x2 = center_x + x2 * scale;
        let screen_y2 = center_y - y2 * scale;

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
        let screen_x = center_x + *x * scale;
        let screen_y = center_y - *y * scale;
        draw_number(
            *n,
            screen_x,
            screen_y,
            painter,
            app.primes_set(),
            &app.config,
            SeriesType::Primes,
        );
    }
}
