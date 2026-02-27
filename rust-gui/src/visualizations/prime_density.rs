//! Prime density graph visualization

use crate::config::DENSITY_INTERVALS;
use crate::helpers::MARGIN_SMALL;
use eframe::egui;

pub const MIN_MAX_N: usize = 10;

pub fn draw(app: &crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let max_n = app.config.max_number;
    if max_n < MIN_MAX_N {
        return;
    }

    let prime_count = app.primes_vec().len();

    let intervals = DENSITY_INTERVALS.max(max_n / DENSITY_INTERVALS);
    let interval_size = max_n / intervals;

    let mut pi_x: Vec<(f32, f32)> = Vec::with_capacity(intervals + 1);
    let mut x_ln_x: Vec<(f32, f32)> = Vec::with_capacity(intervals + 1);

    let mut count = 0;
    let mut prime_idx = 0;

    for i in 0..=intervals {
        let x = i * interval_size;
        if x < 2 {
            pi_x.push((x as f32, 0.0));
            x_ln_x.push((x as f32, 0.0));
            continue;
        }

        while prime_idx < prime_count && app.primes_vec()[prime_idx] <= x {
            count += 1;
            prime_idx += 1;
        }

        let ln_x = (x as f64).ln().max(1.0) as f32;
        let x_ln_x_val = x as f32 / ln_x;

        pi_x.push((x as f32, count as f32));
        x_ln_x.push((x as f32, x_ln_x_val));
    }

    let max_y = pi_x.last().map(|(_, y)| *y).unwrap_or(1.0).max(1.0);

    let graph_left = rect.left() + MARGIN_SMALL;
    let graph_right = rect.right() - MARGIN_SMALL;
    let graph_top = rect.top() + MARGIN_SMALL;
    let graph_bottom = rect.bottom() - MARGIN_SMALL;
    let graph_width = graph_right - graph_left;
    let graph_height = graph_bottom - graph_top;

    let painter = ui.painter();

    painter.line_segment(
        [
            egui::Pos2::new(graph_left, graph_top),
            egui::Pos2::new(graph_left, graph_bottom),
        ],
        egui::Stroke::new(2.0, egui::Color32::GRAY),
    );
    painter.line_segment(
        [
            egui::Pos2::new(graph_left, graph_bottom),
            egui::Pos2::new(graph_right, graph_bottom),
        ],
        egui::Stroke::new(2.0, egui::Color32::GRAY),
    );

    let max_x = max_n as f32;

    for i in 0..x_ln_x.len() - 1 {
        let (x1, y1) = x_ln_x[i];
        let (x2, y2) = x_ln_x[i + 1];

        let px1 = graph_left + (x1 / max_x) * graph_width;
        let py1 = graph_bottom - (y1 / max_y) * graph_height;
        let px2 = graph_left + (x2 / max_x) * graph_width;
        let py2 = graph_bottom - (y2 / max_y) * graph_height;

        painter.line_segment(
            [egui::Pos2::new(px1, py1), egui::Pos2::new(px2, py2)],
            egui::Stroke::new(2.0, app.config.non_highlight_color),
        );
    }

    for i in 0..pi_x.len() - 1 {
        let (x1, y1) = pi_x[i];
        let (x2, y2) = pi_x[i + 1];

        let px1 = graph_left + (x1 / max_x) * graph_width;
        let py1 = graph_bottom - (y1 / max_y) * graph_height;
        let px2 = graph_left + (x2 / max_x) * graph_width;
        let py2 = graph_bottom - (y2 / max_y) * graph_height;

        painter.line_segment(
            [egui::Pos2::new(px1, py1), egui::Pos2::new(px2, py2)],
            egui::Stroke::new(2.0, app.config.highlight_color),
        );
    }
}
