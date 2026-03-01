//! Shared helper functions for visualizations
//!
//! This module provides utility functions and constants used across
//! multiple visualization modules for layout calculations, color generation,
//! and stroke width determination.

use crate::constants::{gap, stroke};
use eframe::egui;

// Re-export constants for backward compatibility
pub use crate::constants::spiral::*;
pub use crate::constants::visualization::*;

/// Layout data returned by compute_layout functions: (offset_x, offset_y, scale).
///
/// - For center-based: (center_x, center_y, scale)
/// - For offset-based: (start_x, start_y, scale)
/// - For row-based: (start_x, center_y, scale)
pub type LayoutData = (f32, f32, f32);

/// Find the closest number to mouse position using center-based layout.
///
/// Layout provides: (center_x, center_y, scale)
pub fn find_hovered_center_based(
    mouse_pos: egui::Pos2,
    positions: &[(usize, f32, f32)],
    layout: LayoutData,
    threshold: f32,
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let (center_x, center_y, scale) = layout;
    let threshold_sq = (scale * threshold).powi(2);
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

/// Find the closest number to mouse position using offset-based layout.
///
/// This is functionally identical to `find_hovered_center_based` - both compute
/// screen position as `offset + logical_position * scale`. The name reflects
/// the typical usage pattern (offset from top-left for grid layouts).
pub fn find_hovered_offset_based(
    mouse_pos: egui::Pos2,
    positions: &[(usize, f32, f32)],
    layout: LayoutData,
    threshold: f32,
) -> Option<usize> {
    find_hovered_center_based(mouse_pos, positions, layout, threshold)
}

/// Find the closest number for row visualization (fixed Y coordinate).
pub fn find_hovered_row(
    mouse_pos: egui::Pos2,
    positions: &[(usize, f32, f32)],
    layout: LayoutData,
    threshold: f32,
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let (start_x, center_y, scale) = layout;
    let threshold_sq = (scale * threshold).powi(2);
    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, _) in positions {
        let screen_x = start_x + *x * scale;
        let dx = mouse_pos.x - screen_x;
        let dy = mouse_pos.y - center_y;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq < min_distance_sq && distance_sq < threshold_sq {
            min_distance_sq = distance_sq;
            closest_n = Some(*n);
        }
    }

    closest_n
}

/// Find the closest number for center-based layout with Y-axis flip (used by Fermats spiral).
pub fn find_hovered_center_flip_y(
    mouse_pos: egui::Pos2,
    positions: &[(usize, f32, f32)],
    layout: LayoutData,
    threshold: f32,
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let (center_x, center_y, scale) = layout;
    let threshold_sq = (scale * threshold).powi(2);
    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y - *y * scale;

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

/// Layout data for visualizations with bounding-box centering (hexagonal, triangular).
/// (center_x, center_y, scale, mid_x, mid_y)
pub type LayoutDataCentered = (f32, f32, f32, f32, f32);

/// Find the closest number for centered layout (bounding box based).
pub fn find_hovered_centered(
    mouse_pos: egui::Pos2,
    positions: &[(usize, f32, f32)],
    layout: LayoutDataCentered,
    threshold: f32,
) -> Option<usize> {
    if positions.is_empty() {
        return None;
    }

    let (center_x, center_y, scale, mid_x, mid_y) = layout;
    let threshold_sq = (scale * threshold).powi(2);
    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in positions {
        let screen_x = center_x + (*x - mid_x) * scale;
        let screen_y = center_y - (*y - mid_y) * scale;

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

/// Calculate bounding box from positions.
///
/// Returns: (min_x, max_x, min_y, max_y)
pub fn calculate_bounds(positions: &[(usize, f32, f32)]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for (_, x, y) in positions {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    (min_x, max_x, min_y, max_y)
}

/// Calculate scale factor to fit content in rect.
///
/// Returns the minimum of x and y scales to maintain aspect ratio.
pub fn calculate_scale(rect: egui::Rect, range_x: f32, range_y: f32, margin: f32) -> f32 {
    let available_width = rect.width() - 2.0 * margin;
    let available_height = rect.height() - 2.0 * margin;
    let scale_x = if range_x > 0.0 {
        available_width / range_x
    } else {
        1.0
    };
    let scale_y = if range_y > 0.0 {
        available_height / range_y
    } else {
        1.0
    };
    scale_x.min(scale_y)
}

/// Generate grayscale color based on gap size for Mobius spirals.
///
/// Larger gaps produce darker colors to emphasize prime gaps.
///
/// # Gap brightness mapping
/// - 2 (twin primes): 255 (brightest)
/// - 4-16: Progressive darkening
/// - 17-20: 60
/// - 21-30: 45
/// - 31-50: 30
/// - 51+: 20 (darkest)
pub fn gap_color(gap: usize) -> egui::Color32 {
    let brightness = match gap {
        2 => gap::BRIGHTNESS_TWIN,
        4 => gap::BRIGHTNESS_4,
        6 => gap::BRIGHTNESS_6,
        8 => gap::BRIGHTNESS_8,
        10 => gap::BRIGHTNESS_10,
        12 => gap::BRIGHTNESS_12,
        14 => gap::BRIGHTNESS_14,
        16 => gap::BRIGHTNESS_16,
        _ if gap <= 20 => gap::BRIGHTNESS_SMALL,
        _ if gap <= 30 => gap::BRIGHTNESS_MEDIUM,
        _ if gap <= 50 => gap::BRIGHTNESS_LARGE,
        _ => gap::BRIGHTNESS_XLARGE,
    };
    egui::Color32::from_rgba_unmultiplied(brightness, brightness, brightness, 255)
}

/// Generate stroke width based on gap size for Mobius spirals.
///
/// Larger gaps produce thinner lines to reduce visual clutter.
///
/// # Gap stroke width mapping
/// - <= 4: 2.5 (extra large)
/// - <= 6: 2.0 (large)
/// - <= 10: 1.5 (medium)
/// - <= 20: 1.0 (small)
/// - 21+: 0.5 (tiny)
pub fn gap_stroke_width(gap: usize) -> f32 {
    if gap <= 4 {
        stroke::XLARGE
    } else if gap <= 6 {
        stroke::LARGE
    } else if gap <= 10 {
        stroke::MEDIUM
    } else if gap <= 20 {
        stroke::SMALL
    } else {
        stroke::TINY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bounds() {
        let positions = vec![(1, 0.0, 0.0), (2, 10.0, 5.0), (3, -3.0, 8.0)];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&positions);
        assert_eq!(min_x, -3.0);
        assert_eq!(max_x, 10.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_y, 8.0);
    }

    #[test]
    fn test_calculate_bounds_empty() {
        let positions: Vec<(usize, f32, f32)> = vec![];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&positions);
        assert_eq!(min_x, f32::MAX);
        assert_eq!(max_x, f32::MIN);
        assert_eq!(min_y, f32::MAX);
        assert_eq!(max_y, f32::MIN);
    }

    #[test]
    fn test_calculate_scale() {
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(100.0, 100.0));
        let scale = calculate_scale(rect, 10.0, 10.0, 10.0);
        assert!((scale - 8.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_scale_zero_range() {
        let rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(100.0, 100.0));
        let scale = calculate_scale(rect, 0.0, 0.0, 10.0);
        assert_eq!(scale, 1.0);
    }

    #[test]
    fn test_gap_color_twin_prime() {
        let color = gap_color(2);
        assert_eq!(color.r(), 255);
    }

    #[test]
    fn test_gap_color_small_gaps() {
        assert_eq!(gap_color(4).r(), 220);
        assert_eq!(gap_color(6).r(), 180);
        assert_eq!(gap_color(8).r(), 150);
        assert_eq!(gap_color(10).r(), 120);
    }

    #[test]
    fn test_gap_color_medium_gaps() {
        assert_eq!(gap_color(18).r(), 60);
        assert_eq!(gap_color(25).r(), 45);
        assert_eq!(gap_color(40).r(), 30);
    }

    #[test]
    fn test_gap_color_large_gaps() {
        assert_eq!(gap_color(100).r(), 20);
    }

    #[test]
    fn test_gap_stroke_width() {
        use crate::constants::stroke;
        assert_eq!(gap_stroke_width(2), stroke::XLARGE);
        assert_eq!(gap_stroke_width(4), stroke::XLARGE);
        assert_eq!(gap_stroke_width(6), stroke::LARGE);
        assert_eq!(gap_stroke_width(10), stroke::MEDIUM);
        assert_eq!(gap_stroke_width(20), stroke::SMALL);
        assert_eq!(gap_stroke_width(21), stroke::TINY);
    }
}
