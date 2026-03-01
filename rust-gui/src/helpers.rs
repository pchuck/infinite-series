//! Shared helper functions for visualizations
//!
//! This module provides utility functions and constants used across
//! multiple visualization modules for layout calculations, color generation,
//! and stroke width determination.

use eframe::egui;

/// Small margin used for visualizations
pub const MARGIN_SMALL: f32 = 20.0;

/// Sacks spiral angle multiplier (r = sqrt(n), theta = n * multiplier)
pub const SACKS_THETA_MULTIPLIER: f32 = 0.5;

/// Sacks Mobius spiral radius multiplier
pub const SACKS_MOBIUS_RADIUS_MULTIPLIER: f32 = 0.8;

/// Golden angle in radians for Fermat's spiral (phyllotaxis pattern)
pub const GOLDEN_ANGLE: f32 = 2.39996_f32;

/// Default hover threshold for point-based visualizations
pub const HOVER_THRESHOLD_DEFAULT: f32 = 0.7;

/// Large hover threshold for sparse visualizations
pub const HOVER_THRESHOLD_LARGE: f32 = 1.5;

/// Tiny stroke width in pixels
pub const STROKE_WIDTH_TINY: f32 = 0.5;

/// Small stroke width in pixels
pub const STROKE_WIDTH_SMALL: f32 = 1.0;

/// Medium stroke width in pixels
pub const STROKE_WIDTH_MEDIUM: f32 = 1.5;

/// Large stroke width in pixels
pub const STROKE_WIDTH_LARGE: f32 = 2.0;

/// Extra large stroke width in pixels
pub const STROKE_WIDTH_XLARGE: f32 = 2.5;

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
pub const GAP_BRIGHTNESS_TWIN: u8 = 255;
pub const GAP_BRIGHTNESS_4: u8 = 220;
pub const GAP_BRIGHTNESS_6: u8 = 180;
pub const GAP_BRIGHTNESS_8: u8 = 150;
pub const GAP_BRIGHTNESS_10: u8 = 120;
pub const GAP_BRIGHTNESS_12: u8 = 100;
pub const GAP_BRIGHTNESS_14: u8 = 85;
pub const GAP_BRIGHTNESS_16: u8 = 70;
pub const GAP_BRIGHTNESS_SMALL: u8 = 60;
pub const GAP_BRIGHTNESS_MEDIUM: u8 = 45;
pub const GAP_BRIGHTNESS_LARGE: u8 = 30;
pub const GAP_BRIGHTNESS_XLARGE: u8 = 20;

pub fn gap_color(gap: usize) -> egui::Color32 {
    let brightness = match gap {
        2 => GAP_BRIGHTNESS_TWIN,
        4 => GAP_BRIGHTNESS_4,
        6 => GAP_BRIGHTNESS_6,
        8 => GAP_BRIGHTNESS_8,
        10 => GAP_BRIGHTNESS_10,
        12 => GAP_BRIGHTNESS_12,
        14 => GAP_BRIGHTNESS_14,
        16 => GAP_BRIGHTNESS_16,
        _ if gap <= 20 => GAP_BRIGHTNESS_SMALL,
        _ if gap <= 30 => GAP_BRIGHTNESS_MEDIUM,
        _ if gap <= 50 => GAP_BRIGHTNESS_LARGE,
        _ => GAP_BRIGHTNESS_XLARGE,
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
        STROKE_WIDTH_XLARGE
    } else if gap <= 6 {
        STROKE_WIDTH_LARGE
    } else if gap <= 10 {
        STROKE_WIDTH_MEDIUM
    } else if gap <= 20 {
        STROKE_WIDTH_SMALL
    } else {
        STROKE_WIDTH_TINY
    }
}
