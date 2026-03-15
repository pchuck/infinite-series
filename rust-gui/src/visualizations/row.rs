//! Row visualization

use crate::app::NumberVisualizerApp;
use crate::draw_number::draw_number;
use crate::helpers::{find_hovered_row, LayoutData, HOVER_THRESHOLD_DEFAULT, MARGIN_SMALL};
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Generate positions for row layout.
///
/// Numbers are arranged in a single horizontal line from left to right.
/// Returns a vector of (number, x, y) tuples where y is always 0.
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
    max_number: usize,
) -> (f32, f32, f32) {
    let max_x = max_number as f32;
    let available_width = rect.width() - 2.0 * MARGIN_SMALL;
    let scale = available_width / max_x;

    let center_y = rect.center().y;
    let start_x = rect.left() + MARGIN_SMALL + scale / 2.0;

    (start_x, center_y, scale)
}

/// Draw the row visualization.
///
/// Renders all numbers as circles in a horizontal line, with highlights shown in highlight color.
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

/// Find the number at the given mouse position.
///
/// Returns the closest number within the hover threshold, or None if no number is close enough.
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

pub struct Row;

impl Visualizer for Row {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::Row
    }

    fn name(&self) -> &'static str {
        "Row"
    }

    fn description(&self) -> &'static str {
        VisualizationType::Row.description()
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
    fn test_generate_positions_linear() {
        let positions = generate_positions(5);
        assert_eq!(positions[0], (1, 1.0, 0.0));
        assert_eq!(positions[1], (2, 2.0, 0.0));
        assert_eq!(positions[2], (3, 3.0, 0.0));
        assert_eq!(positions[3], (4, 4.0, 0.0));
        assert_eq!(positions[4], (5, 5.0, 0.0));
    }

    #[test]
    fn test_compute_layout_centered_vertically() {
        let positions = generate_positions(10);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 300.0));
        let (start_x, center_y, scale) = compute_layout(&positions, rect, 10);

        assert_eq!(center_y, 150.0, "row should be vertically centered");
        assert!(start_x > rect.left(), "start_x should be within rect");
        assert!(scale > 0.0, "scale should be positive");
    }

    #[test]
    fn test_compute_layout_all_points_fit() {
        let max_n = 50;
        let positions = generate_positions(max_n);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 300.0));
        let (start_x, _, scale) = compute_layout(&positions, rect, max_n);

        for (_, x, _) in &positions {
            let screen_x = start_x + *x * scale;
            assert!(
                screen_x >= rect.left() && screen_x <= rect.right(),
                "point maps outside rect horizontally"
            );
        }
    }

    #[test]
    fn test_find_hovered_at_first_point() {
        let max_n = 10;
        let positions = generate_positions(max_n);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 300.0));
        let layout: LayoutData = compute_layout(&positions, rect, max_n);

        // First point is at x=1.0, y=0.0, maps to (start_x + 1*scale, center_y)
        let (start_x, center_y, scale) = layout;
        let screen_x = start_x + 1.0 * scale;
        let hovered = find_hovered_row(
            egui::Pos2::new(screen_x, center_y),
            &positions,
            layout,
            HOVER_THRESHOLD_DEFAULT,
        );
        assert_eq!(hovered, Some(1));
    }

    #[test]
    fn test_find_hovered_miss() {
        let max_n = 10;
        let positions = generate_positions(max_n);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 300.0));
        let layout: LayoutData = compute_layout(&positions, rect, max_n);

        let hovered = find_hovered_row(
            egui::Pos2::new(-1000.0, -1000.0),
            &positions,
            layout,
            HOVER_THRESHOLD_DEFAULT,
        );
        assert_eq!(hovered, None);
    }
}
