//! Prime wheel visualization

use crate::app::NumberVisualizerApp;
use crate::config::VisualizerConfig;
use crate::draw_number::draw_number;
use crate::helpers::{HOVER_THRESHOLD_DEFAULT, MARGIN_SMALL};
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Minimum modulo value for the prime wheel.
pub const MODULO_MIN: usize = 2;
/// Maximum modulo value for the prime wheel.
pub const MODULO_MAX: usize = 60;

/// Generate positions for the prime wheel visualization.
///
/// Numbers are arranged in concentric rings, with each ring representing a quotient
/// and positions within the ring determined by the remainder modulo the configured value.
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

/// Compute layout for prime wheel visualization.
///
/// Returns: (center_x, center_y, scale, modulo)
/// - center_x, center_y: Center of the wheel
/// - scale: Pixels per ring
/// - modulo: Used for spoke calculations (cast to f32)
pub fn compute_layout(
    _positions: &[(usize, f32, f32)],
    rect: egui::Rect,
    modulo: usize,
    max_number: usize,
) -> (f32, f32, f32, f32) {
    let modulo_f = modulo as f32;
    let max_ring = (max_number / modulo) as f32 + 2.0;

    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_ring > 0.0 {
        available / max_ring
    } else {
        available
    };

    let center_x = rect.center().x;
    let center_y = rect.center().y;

    (center_x, center_y, scale, modulo_f)
}

/// Draw the prime wheel visualization.
///
/// Renders numbers in concentric rings colored by their modulo residue.
/// Prime numbers are shown in highlight color on their respective spokes.
/// Uses cached positions and `compute_layout` for screen coordinate mapping.
pub fn draw(
    app: &crate::app::NumberVisualizerApp,
    ui: &mut egui::Ui,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) {
    if positions.is_empty() {
        return;
    }

    let (center_x, center_y, scale, modulo) =
        compute_layout(positions, rect, app.config.modulo, app.config.max_number);

    let max_ring = (app.config.max_number / app.config.modulo) as f32 + 2.0;
    let painter = ui.painter();

    for (n, x, y) in positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;
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

/// Find the number at the given mouse position.
///
/// Returns the closest number within the hover threshold (in screen pixels),
/// or None if no number is close enough.
pub fn find_hovered(
    app: &crate::app::NumberVisualizerApp,
    mouse_pos: egui::Pos2,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let (center_x, center_y, scale, _) =
        compute_layout(positions, rect, app.config.modulo, app.config.max_number);

    let threshold_sq = HOVER_THRESHOLD_DEFAULT * HOVER_THRESHOLD_DEFAULT;
    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;

        let dx = mouse_pos.x - screen_x;
        let dy = mouse_pos.y - screen_y;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq < min_distance_sq && distance_sq < threshold_sq {
            min_distance_sq = distance_sq;
            closest_n = Some(*n);
        }
    }

    closest_n
}

pub struct PrimeWheel;

impl Visualizer for PrimeWheel {
    fn viz_type(&self) -> VisualizationType {
        VisualizationType::PrimeWheel
    }

    fn name(&self) -> &'static str {
        "Prime Wheel"
    }

    fn description(&self) -> &'static str {
        VisualizationType::PrimeWheel.description()
    }

    fn supports_series(&self, series: SeriesType) -> bool {
        series == SeriesType::Primes
    }

    fn supports_hover(&self) -> bool {
        true
    }

    fn uses_point_rendering(&self) -> bool {
        true
    }

    fn generate_positions(&self, max_n: usize, params: &VizParams) -> Vec<(usize, f32, f32)> {
        let modulo = params.modulo.unwrap_or(30);
        generate_positions(max_n, modulo)
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

    fn config_ui(&self, ui: &mut egui::Ui, config: &mut VisualizerConfig, _series: SeriesType) {
        ui.label("Prime Wheel");
        ui.add(egui::Slider::new(&mut config.modulo, MODULO_MIN..=MODULO_MAX).text("Modulo"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_positions_count() {
        let positions = generate_positions(100, 6);
        assert_eq!(positions.len(), 100);
    }

    #[test]
    fn test_generate_positions_modulo_2() {
        let positions = generate_positions(10, 2);
        assert_eq!(positions.len(), 10);
        // All positions should have valid coordinates
        for (n, x, y) in &positions {
            assert!(*n >= 1 && *n <= 10);
            assert!(x.is_finite());
            assert!(y.is_finite());
        }
    }

    #[test]
    fn test_empty_positions() {
        let positions = generate_positions(0, 6);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_compute_layout_centering() {
        let positions = generate_positions(50, 6);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 400.0));
        let (center_x, center_y, scale, modulo_f) = compute_layout(&positions, rect, 6, 50);

        assert_eq!(center_x, 200.0);
        assert_eq!(center_y, 200.0);
        assert!(scale > 0.0, "scale should be positive");
        assert_eq!(modulo_f, 6.0);
    }

    #[test]
    fn test_compute_layout_scale_fits_rect() {
        let positions = generate_positions(100, 6);
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(400.0, 400.0));
        let (center_x, center_y, scale, _) = compute_layout(&positions, rect, 6, 100);

        for (_, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + *y * scale;
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
    fn test_positions_ring_structure() {
        let modulo = 6;
        let positions = generate_positions(12, modulo);
        // Numbers 1-6 should be in ring 0 (quotient=0 for n<modulo, except n=modulo)
        // Verify each number has correct ring assignment encoded in radius
        for (n, x, y) in &positions {
            let r = (x * x + y * y).sqrt();
            let expected_ring = (*n / modulo) as f32 + 1.0;
            assert!(
                (r - expected_ring).abs() < 0.01,
                "n={} should be at ring {}, got r={}",
                n,
                expected_ring,
                r
            );
        }
    }
}
