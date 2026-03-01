//! Number rendering for point-based visualizations

use crate::config::VisualizerConfig;
use crate::constants::{drawing, limits};
use crate::types::SeriesType;
use eframe::egui;
use std::collections::HashSet;

// Re-export constants for backward compatibility
pub use crate::constants::drawing::*;

/// Get the color for a prime pair (twin, cousin, or sexy prime).
///
/// Returns `None` if the number is not a prime pair or if the series type
/// is not primes.
pub fn get_prime_pair_color(
    n: usize,
    highlights: &HashSet<usize>,
    config: &VisualizerConfig,
    series_type: SeriesType,
) -> Option<egui::Color32> {
    if series_type != SeriesType::Primes {
        return None;
    }

    if !highlights.contains(&n) {
        return None;
    }

    if config.show_twin_primes
        && (highlights.contains(&(n + 2)) || (n > 2 && highlights.contains(&(n - 2))))
    {
        return Some(config.twin_color);
    }

    if config.show_cousin_primes
        && (highlights.contains(&(n + 4)) || (n > 4 && highlights.contains(&(n - 4))))
    {
        return Some(config.cousin_color);
    }

    if config.show_sexy_primes
        && (highlights.contains(&(n + 6)) || (n > 6 && highlights.contains(&(n - 6))))
    {
        return Some(config.sexy_color);
    }

    None
}

/// Draw a single number with appropriate highlighting.
///
/// Draws a circle at the specified position. If the number is in the highlights set,
/// it will be drawn with the highlight color and size. For prime series, special
/// colors are applied for twin, cousin, and sexy primes when enabled.
///
/// If `show_numbers` is enabled and the circle is large enough, the number text
/// will be drawn inside the circle.
pub fn draw_number(
    n: usize,
    x: f32,
    y: f32,
    painter: &egui::Painter,
    highlights: &HashSet<usize>,
    config: &VisualizerConfig,
    series_type: SeriesType,
) {
    let is_highlighted = highlights.contains(&n);

    let (is_twin_prime, is_cousin_prime, is_sexy_prime) =
        if series_type == SeriesType::Primes && is_highlighted {
            let twin = config.show_twin_primes
                && (highlights.contains(&(n + 2)) || (n > 2 && highlights.contains(&(n - 2))));
            let cousin = config.show_cousin_primes
                && !twin
                && (highlights.contains(&(n + 4)) || (n > 4 && highlights.contains(&(n - 4))));
            let sexy = config.show_sexy_primes
                && !twin
                && !cousin
                && (highlights.contains(&(n + 6)) || (n > 6 && highlights.contains(&(n - 6))));
            (twin, cousin, sexy)
        } else {
            (false, false, false)
        };

    let size = if is_highlighted {
        config.highlight_size as f32
    } else {
        config.non_highlight_size as f32
    };

    if size == 0.0 {
        return;
    }

    let color = if is_twin_prime {
        config.twin_color
    } else if is_cousin_prime {
        config.cousin_color
    } else if is_sexy_prime {
        config.sexy_color
    } else if is_highlighted {
        config.highlight_color
    } else {
        config.non_highlight_color
    };

    let radius = size / 2.0;
    painter.circle_filled(egui::Pos2::new(x, y), radius.max(MIN_CIRCLE_RADIUS), color);

    let show_text = config.show_numbers
        && size >= drawing::MIN_SIZE_FOR_TEXT
        && config.max_number <= limits::SHOW_NUMBERS_MAX;

    if show_text {
        let text = format!("{}", n);
        let font_id = egui::FontId::proportional(size * TEXT_SIZE_FACTOR);
        painter.text(
            egui::Pos2::new(x, y),
            egui::Align2::CENTER_CENTER,
            text,
            font_id,
            config.background_color,
        );
    }
}
