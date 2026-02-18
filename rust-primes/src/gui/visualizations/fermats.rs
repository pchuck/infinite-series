//! Fermat's spiral visualization

use crate::gui::draw_number::draw_number;
use crate::gui::GOLDEN_ANGLE;
use crate::gui::HOVER_THRESHOLD_DEFAULT;
use crate::gui::MARGIN_SMALL;
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    (1..=max_n)
        .map(|n| {
            let n_f = n as f32;
            let r = n_f.sqrt();
            let theta = n_f * GOLDEN_ANGLE;
            let x = r * theta.cos();
            let y = r * theta.sin();
            (n, x, y)
        })
        .collect()
}

pub fn draw(app: &crate::gui::app::PrimeVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let positions = generate_positions(app.config.max_number);

    if positions.is_empty() {
        return;
    }

    let mut max_r = 0.0f32;
    for (_, x, y) in &positions {
        let r = (x * x + y * y).sqrt();
        max_r = max_r.max(r);
    }

    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_r > 0.0 { available / max_r } else { 1.0 };

    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let painter = ui.painter();

    for (n, x, y) in &positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y - *y * scale;
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

    let mut max_r = 0.0f32;
    for (_, x, y) in &positions {
        let r = (x * x + y * y).sqrt();
        max_r = max_r.max(r);
    }

    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_r > 0.0 { available / max_r } else { 1.0 };

    let center_x = rect.center().x;
    let center_y = rect.center().y;

    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in &positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y - *y * scale;

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
