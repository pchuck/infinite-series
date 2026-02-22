//! Prime wheel visualization

use crate::draw_number::draw_number;
use crate::helpers::MARGIN_SMALL;
use crate::types::SeriesType;
use eframe::egui;

pub fn generate_positions(max_n: usize, modulo: usize) -> Vec<(usize, f32, f32)> {
    (1..=max_n)
        .map(|n| {
            let remainder = n % modulo;
            let quotient = n / modulo;

            let theta = remainder as f32 * 2.0 * std::f32::consts::PI / modulo as f32
                - std::f32::consts::PI / 2.0;
            let r = quotient as f32 + 1.0;

            let x = r * theta.cos();
            let y = r * theta.sin();
            (n, x, y)
        })
        .collect()
}

pub fn draw(app: &crate::app::NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let positions = generate_positions(app.config.max_number, app.config.modulo);

    if positions.is_empty() {
        return;
    }

    let modulo = app.config.modulo as f32;
    let max_ring = (app.config.max_number / app.config.modulo) as f32 + 2.0;

    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_ring > 0.0 {
        available / max_ring
    } else {
        available
    };

    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let painter = ui.painter();

    for (n, _, _) in &positions {
        let remainder = *n % app.config.modulo;
        let quotient = *n / app.config.modulo;

        let theta =
            remainder as f32 * 2.0 * std::f32::consts::PI / modulo - std::f32::consts::PI / 2.0;
        let r = (quotient as f32 + 1.0) * scale;

        let screen_x = center_x + r * theta.cos();
        let screen_y = center_y + r * theta.sin();
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

    for spoke in 0..app.config.modulo {
        let theta = spoke as f32 * 2.0 * std::f32::consts::PI / modulo - std::f32::consts::PI / 2.0;
        let inner_r = scale;
        let outer_r = max_ring * scale;

        let start_x = center_x + inner_r * theta.cos();
        let start_y = center_y + inner_r * theta.sin();
        let end_x = center_x + outer_r * theta.cos();
        let end_y = center_y + outer_r * theta.sin();

        painter.line_segment(
            [
                egui::Pos2::new(start_x, start_y),
                egui::Pos2::new(end_x, end_y),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(50, 50, 50, 100)),
        );
    }
}
