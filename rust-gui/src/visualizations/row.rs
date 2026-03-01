//! Row visualization

use crate::draw_number::draw_number;
use crate::helpers::{find_hovered_row, LayoutData, HOVER_THRESHOLD_DEFAULT, MARGIN_SMALL};
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    (1..=max_n).map(|n| (n, n as f32, 0.0)).collect()
}

/// Compute layout for row visualization.
///
/// Returns: (start_x, center_y, scale)
/// - start_x: Starting x position of the row
/// - center_y: Vertical center of the row
/// - scale: Pixels per unit
pub fn compute_layout(
    _positions: &[(usize, f32, f32)],
    rect: egui::Rect,
    max_n: usize,
) -> (f32, f32, f32) {
    let max_x = max_n as f32;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let scale = available_width / max_x;

    let center_y = rect.center().y;
    let start_x = rect.left() + MARGIN_SMALL + scale / 2.0;

    (start_x, center_y, scale)
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

    let (start_x, center_y, scale) = compute_layout(positions, rect, app.config.max_number);
    let painter = ui.painter();

    for (n, x, _) in positions {
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
    positions: &[(usize, f32, f32)],
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let layout: LayoutData = compute_layout(positions, rect, app.config.max_number);
    find_hovered_row(mouse_pos, positions, layout, HOVER_THRESHOLD_DEFAULT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_positions_count() {
        let positions = generate_positions(50);
        assert_eq!(positions.len(), 50);
    }

    #[test]
    fn test_generate_positions_linear() {
        let positions = generate_positions(5);
        assert_eq!(positions[0], (1, 1.0, 0.0));
        assert_eq!(positions[1], (2, 2.0, 0.0));
        assert_eq!(positions[2], (3, 3.0, 0.0));
        assert_eq!(positions[3], (4, 4.0, 0.0));
        assert_eq!(positions[4], (5, 5.0, 0.0));
    }
}
