//! Hexagonal lattice visualization

use crate::app::NumberVisualizerApp;
use crate::draw_number::draw_number;
use crate::helpers::{
    calculate_bounds, calculate_scale, find_hovered_centered, LayoutDataCentered,
    HOVER_THRESHOLD_LARGE, MARGIN_SMALL,
};
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Generate positions for hexagonal lattice.
///
/// Numbers spiral outward from the center in a hexagonal pattern (6 directions).
/// Returns a vector of (number, x, y) tuples where (0,0) is the center.
pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    let mut positions = Vec::with_capacity(max_n);

    if max_n == 0 {
        return positions;
    }

    let mut x = 0i32;
    let mut y = 0i32;

    let hex_directions: [(i32, i32); 6] = [(2, 0), (1, 2), (-1, 2), (-2, 0), (-1, -2), (1, -2)];

    let mut steps_in_direction = 1;
    let mut steps_since_turn = 0;
    let mut turn_count = 0;
    let mut dir_idx = 0;

    for n in 1..=max_n {
        positions.push((n, x as f32, y as f32));

        if n == max_n {
            break;
        }

        x += hex_directions[dir_idx].0;
        y += hex_directions[dir_idx].1;
        steps_since_turn += 1;

        if steps_since_turn == steps_in_direction {
            steps_since_turn = 0;
            dir_idx = (dir_idx + 1) % 6;
            turn_count += 1;
            if turn_count % 2 == 0 {
                steps_in_direction += 1;
            }
        }
    }

    positions
}

/// Compute layout for hexagonal lattice visualization.
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

/// Draw the hexagonal lattice visualization.
///
/// Renders all numbers as circles on a hexagonal spiral, with highlights shown in highlight color.
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

    let layout: LayoutDataCentered = compute_layout(positions, rect);
    find_hovered_centered(mouse_pos, positions, layout, HOVER_THRESHOLD_LARGE)
}

pub struct HexagonalLattice;

impl Visualizer for HexagonalLattice {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::HexagonalLattice
    }

    fn name(&self) -> &'static str {
        "Hexagonal Lattice"
    }

    fn description(&self) -> &'static str {
        VisualizationType::HexagonalLattice.description()
    }

    fn supports_series(&self, _series: SeriesType) -> bool {
        true
    }

    fn supports_hover(&self) -> bool {
        true
    }

    fn uses_point_rendering(&self) -> bool {
        true
    }

    fn generate_positions(&self, max_n: usize, _params: &VizParams) -> Vec<(usize, f32, f32)> {
        generate_positions(max_n)
    }

    fn draw(
        &self,
        app: &mut NumberVisualizerApp,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        positions: &[(usize, f32, f32)],
    ) {
        draw(app, ui, rect, positions);
    }

    fn find_hovered(
        &self,
        app: &NumberVisualizerApp,
        mouse_pos: egui::Pos2,
        rect: egui::Rect,
        positions: &[(usize, f32, f32)],
    ) -> Option<usize> {
        find_hovered(app, mouse_pos, rect, positions)
    }
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
    fn test_generate_positions_start_at_origin() {
        let positions = generate_positions(1);
        assert_eq!(positions[0], (1, 0.0, 0.0));
    }

    #[test]
    fn test_empty_positions() {
        let positions = generate_positions(0);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_compute_layout_centering() {
        let positions = generate_positions(50);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 400.0));
        let (center_x, center_y, scale, _, _) = compute_layout(&positions, rect);

        assert_eq!(center_x, 200.0);
        assert_eq!(center_y, 200.0);
        assert!(scale > 0.0, "scale should be positive");
    }

    #[test]
    fn test_compute_layout_all_points_fit() {
        let positions = generate_positions(100);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 400.0));
        let (center_x, center_y, scale, mid_x, mid_y) = compute_layout(&positions, rect);

        for (_, x, y) in &positions {
            let screen_x = center_x + (*x - mid_x) * scale;
            let screen_y = center_y - (*y - mid_y) * scale;
            assert!(
                screen_x >= rect.left() && screen_x <= rect.right(),
                "point maps outside rect horizontally"
            );
            assert!(
                screen_y >= rect.top() && screen_y <= rect.bottom(),
                "point maps outside rect vertically"
            );
        }
    }

    #[test]
    fn test_find_hovered_at_first_point() {
        let positions = generate_positions(50);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 400.0));
        let layout: LayoutDataCentered = compute_layout(&positions, rect);

        let (cx, cy, scale, mid_x, mid_y) = layout;
        let (_, x1, y1) = positions[0];
        let screen_x = cx + (x1 - mid_x) * scale;
        let screen_y = cy - (y1 - mid_y) * scale;

        let hovered = find_hovered_centered(
            egui::Pos2::new(screen_x, screen_y),
            &positions,
            layout,
            HOVER_THRESHOLD_LARGE,
        );
        assert_eq!(hovered, Some(1));
    }

    #[test]
    fn test_find_hovered_miss() {
        let positions = generate_positions(50);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 400.0));
        let layout: LayoutDataCentered = compute_layout(&positions, rect);

        let hovered = find_hovered_centered(
            egui::Pos2::new(-5000.0, -5000.0),
            &positions,
            layout,
            HOVER_THRESHOLD_LARGE,
        );
        assert_eq!(hovered, None);
    }
}
