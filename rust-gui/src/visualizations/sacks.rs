//! Sacks spiral visualization

use crate::draw_number::draw_number;
use crate::helpers::SACKS_THETA_MULTIPLIER;
use crate::helpers::{find_hovered_center_based, HOVER_THRESHOLD_DEFAULT, MARGIN_SMALL};
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    (1..=max_n)
        .map(|n| {
            let n_f = n as f32;
            let r = n_f.sqrt();
            let theta = n_f * SACKS_THETA_MULTIPLIER;
            let x = r * theta.cos();
            let y = r * theta.sin();
            (n, x, y)
        })
        .collect()
}

/// Compute layout for Sacks spiral visualization.
///
/// Returns: (center_x, center_y, scale)
/// - center_x, center_y: Center of the spiral
/// - scale: Pixels per unit radius
pub fn compute_layout(positions: &[(usize, f32, f32)], rect: egui::Rect) -> (f32, f32, f32) {
    let mut max_r = 0.0f32;
    for (_, x, y) in positions {
        let r = (x * x + y * y).sqrt();
        max_r = max_r.max(r);
    }

    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_r > 0.0 { available / max_r } else { 1.0 };

    let center_x = rect.center().x;
    let center_y = rect.center().y;

    (center_x, center_y, scale)
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

    let (center_x, center_y, scale) = compute_layout(positions, rect);
    let painter = ui.painter();

    for (n, x, y) in positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;
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
    _app: &crate::app::NumberVisualizerApp,
    mouse_pos: egui::Pos2,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let layout = compute_layout(positions, rect);
    find_hovered_center_based(mouse_pos, positions, layout, HOVER_THRESHOLD_DEFAULT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_positions_count() {
        let positions = generate_positions(100);
        assert_eq!(positions.len(), 100);
    }

    #[test]
    fn test_generate_positions_monotonic_radius() {
        let positions = generate_positions(100);
        let mut prev_r: f32 = 0.0;
        for (_, x, y) in positions {
            let r = (x * x + y * y).sqrt();
            assert!(r >= prev_r - 0.001, "radius should not decrease");
            prev_r = r;
        }
    }

    #[test]
    fn test_empty_positions() {
        let positions = generate_positions(0);
        assert!(positions.is_empty());
    }
}
