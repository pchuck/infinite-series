//! Triangular lattice visualization

use crate::draw_number::draw_number;
use crate::helpers::{calculate_bounds, calculate_scale, HOVER_THRESHOLD_LARGE, MARGIN_SMALL};
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    let mut positions = Vec::with_capacity(max_n);

    if max_n == 0 {
        return positions;
    }

    let mut x = 0i32;
    let mut y = 0i32;

    let tri_directions: [(i32, i32); 3] = [(2, 0), (-1, 2), (-1, -2)];

    let mut steps_in_direction = 1;
    let mut steps_since_turn = 0;
    let mut turn_count = 0;
    let mut dir_idx = 0;

    for n in 1..=max_n {
        positions.push((n, x as f32, y as f32));

        if n == max_n {
            break;
        }

        x += tri_directions[dir_idx].0;
        y += tri_directions[dir_idx].1;
        steps_since_turn += 1;

        if steps_since_turn == steps_in_direction {
            steps_since_turn = 0;
            dir_idx = (dir_idx + 1) % 3;
            turn_count += 1;
            if turn_count % 2 == 0 {
                steps_in_direction += 1;
            }
        }
    }

    positions
}

/// Compute layout for triangular lattice visualization.
///
/// Returns: (center_x, center_y, scale, mid_x, mid_y)
/// - center_x, center_y: Center of the visualization
/// - scale: Pixels per unit
/// - mid_x, mid_y: Center of bounding box for content centering
pub fn compute_layout(
    positions: &[(usize, f32, f32)],
    rect: egui::Rect,
) -> (f32, f32, f32, f32, f32) {
    let (min_x, max_x, min_y, max_y) = calculate_bounds(positions);
    let range_x = max_x - min_x;
    let range_y = max_y - min_y;

    let scale = calculate_scale(rect, range_x, range_y, MARGIN_SMALL);

    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let mid_x = (min_x + max_x) / 2.0;
    let mid_y = (min_y + max_y) / 2.0;

    (center_x, center_y, scale, mid_x, mid_y)
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

    let (center_x, center_y, scale, mid_x, mid_y) = compute_layout(positions, rect);
    let painter = ui.painter();

    for (n, x, y) in positions {
        let screen_x = center_x + (*x - mid_x) * scale;
        let screen_y = center_y - (*y - mid_y) * scale;
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

    let (center_x, center_y, scale, mid_x, mid_y) = compute_layout(positions, rect);

    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in positions {
        let screen_x = center_x + (*x - mid_x) * scale;
        let screen_y = center_y - (*y - mid_y) * scale;

        let dx = mouse_pos.x - screen_x;
        let dy = mouse_pos.y - screen_y;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq < min_distance_sq && distance_sq < (scale * HOVER_THRESHOLD_LARGE).powi(2) {
            min_distance_sq = distance_sq;
            closest_n = Some(*n);
        }
    }

    closest_n
}
