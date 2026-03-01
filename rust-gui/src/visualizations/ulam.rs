//! Ulam spiral visualization

use crate::draw_number::draw_number;
use crate::helpers::{find_hovered_center_based, HOVER_THRESHOLD_DEFAULT, MARGIN_SMALL};
use eframe::egui;

/// Generate positions for Ulam spiral (square spiral).
///
/// Numbers spiral outward from the center in a square pattern.
/// Returns a vector of (number, x, y) tuples where (0,0) is the center.
pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    let mut positions = Vec::with_capacity(max_n);

    if max_n == 0 {
        return positions;
    }

    let mut x = 0i32;
    let mut y = 0i32;
    let mut dx = 1i32;
    let mut dy = 0i32;
    let mut steps_in_direction = 1;
    let mut steps_since_turn = 0;
    let mut turn_count = 0;

    for n in 1..=max_n {
        positions.push((n, x as f32, y as f32));

        if n == max_n {
            break;
        }

        x += dx;
        y += dy;
        steps_since_turn += 1;

        if steps_since_turn == steps_in_direction {
            steps_since_turn = 0;

            let (new_dx, new_dy) = match turn_count % 4 {
                0 => (0, 1),
                1 => (-1, 0),
                2 => (0, -1),
                _ => (1, 0),
            };
            dx = new_dx;
            dy = new_dy;

            turn_count += 1;
            if turn_count % 2 == 0 {
                steps_in_direction += 1;
            }
        }
    }

    positions
}

/// Compute layout for Ulam spiral visualization.
///
/// Returns: (center_x, center_y, scale, max_coord)
/// - center_x, center_y: Center of the visualization
/// - scale: Pixels per unit
/// - max_coord: Maximum coordinate magnitude for bounds calculation
pub fn compute_layout(positions: &[(usize, f32, f32)], rect: egui::Rect) -> (f32, f32, f32, f32) {
    let mut max_coord = 0.0f32;
    for (_, x, y) in positions {
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

    (center_x, center_y, scale, max_coord)
}

/// Draw the Ulam spiral visualization.
///
/// Renders all numbers as circles on a square spiral, with highlights shown in highlight color.
pub fn draw(
    app: &crate::app::NumberVisualizerApp,
    ui: &mut egui::Ui,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) {
    if positions.is_empty() {
        return;
    }

    let (center_x, center_y, scale, _) = compute_layout(positions, rect);
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

/// Find the number at the given mouse position.
///
/// Returns the closest number within the hover threshold, or None if no number is close enough.
pub fn find_hovered(
    _app: &crate::app::NumberVisualizerApp,
    mouse_pos: egui::Pos2,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let (center_x, center_y, scale, _) = compute_layout(positions, rect);
    find_hovered_center_based(
        mouse_pos,
        positions,
        (center_x, center_y, scale),
        HOVER_THRESHOLD_DEFAULT,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_positions_count() {
        let positions = generate_positions(10);
        assert_eq!(positions.len(), 10);
    }

    #[test]
    fn test_generate_positions_start_at_center() {
        let positions = generate_positions(1);
        assert_eq!(positions[0], (1, 0.0, 0.0));
    }

    #[test]
    fn test_generate_positions_spiral_outward() {
        let positions = generate_positions(9);
        assert_eq!(positions[0], (1, 0.0, 0.0));
        assert_eq!(positions[8].0, 9);
    }

    #[test]
    fn test_empty_positions() {
        let positions = generate_positions(0);
        assert!(positions.is_empty());
    }
}
