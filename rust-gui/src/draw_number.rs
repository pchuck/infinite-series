//! Number rendering for point-based visualizations

use crate::config::{PrimePairType, VisualizerConfig};
use crate::constants::{drawing, limits};
use crate::types::SeriesType;
use eframe::egui;
use std::collections::HashSet;

// Re-export constants for backward compatibility
pub use crate::constants::drawing::*;

fn get_prime_pair_types(
    n: usize,
    highlights: &HashSet<usize>,
    config: &VisualizerConfig,
) -> Vec<PrimePairType> {
    let mut pair_types = Vec::new();

    if config.show_twin_primes
        && (highlights.contains(&(n + 2)) || (n > 2 && highlights.contains(&(n - 2))))
    {
        pair_types.push(PrimePairType::Twin);
    }

    if config.show_cousin_primes
        && (highlights.contains(&(n + 4)) || (n > 4 && highlights.contains(&(n - 4))))
    {
        pair_types.push(PrimePairType::Cousin);
    }

    if config.show_sexy_primes
        && (highlights.contains(&(n + 6)) || (n > 6 && highlights.contains(&(n - 6))))
    {
        pair_types.push(PrimePairType::Sexy);
    }

    pair_types
}

/// Get the color for a prime that belongs to one or more prime pair types.
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

    let pair_types = get_prime_pair_types(n, highlights, config);
    
    if pair_types.is_empty() {
        return None;
    }

    Some(config.prime_pair_colors.get_color(&pair_types))
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

    if !is_highlighted {
        let size = config.non_highlight_size as f32;
        if size == 0.0 {
            return;
        }
        let radius = size / 2.0;
        painter.circle_filled(
            egui::Pos2::new(x, y),
            radius.max(MIN_CIRCLE_RADIUS),
            config.non_highlight_color,
        );
        draw_number_text(n, x, y, painter, config);
        return;
    }

    let size = config.highlight_size as f32;
    if size == 0.0 {
        return;
    }

    let color = get_prime_pair_color(n, highlights, config, series_type)
        .unwrap_or(config.highlight_color);

    let radius = size / 2.0;
    painter.circle_filled(egui::Pos2::new(x, y), radius.max(MIN_CIRCLE_RADIUS), color);

    draw_number_text(n, x, y, painter, config);
}

fn draw_number_text(
    n: usize,
    x: f32,
    y: f32,
    painter: &egui::Painter,
    config: &VisualizerConfig,
) {
    let show_text = config.show_numbers
        && config.highlight_size as f32 >= drawing::MIN_SIZE_FOR_TEXT
        && config.max_number <= limits::SHOW_NUMBERS_MAX;

    if show_text {
        let text = format!("{}", n);
        let font_id = egui::FontId::proportional(config.highlight_size as f32 * TEXT_SIZE_FACTOR);
        painter.text(
            egui::Pos2::new(x, y),
            egui::Align2::CENTER_CENTER,
            text,
            font_id,
            config.background_color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PrimePairType, VisualizerConfig};
    use std::collections::HashSet;

    #[test]
    fn test_get_prime_pair_color_blend() {
        let highlights: HashSet<usize> = [2, 3, 5, 7, 11, 13].into_iter().collect();
        let mut config = VisualizerConfig::default();
        config.show_twin_primes = true;
        config.show_cousin_primes = true;
        config.show_sexy_primes = true;
        
        let color = get_prime_pair_color(7, &highlights, &config, SeriesType::Primes);
        assert!(color.is_some());
        
        let expected_color = config.prime_pair_colors.get_color(&[
            PrimePairType::Twin,
            PrimePairType::Cousin,
            PrimePairType::Sexy,
        ]);
        assert_eq!(color, Some(expected_color));
    }

    #[test]
    fn test_get_prime_pair_color_none_for_non_prime() {
        let highlights: HashSet<usize> = [2, 3, 5, 7].into_iter().collect();
        let config = VisualizerConfig::default();
        
        let color = get_prime_pair_color(4, &highlights, &config, SeriesType::Primes);
        assert!(color.is_none());
    }

    #[test]
    fn test_get_prime_pair_color_non_primes_series() {
        let highlights: HashSet<usize> = [1, 2, 3, 5, 8].into_iter().collect();
        let config = VisualizerConfig::default();
        
        let color = get_prime_pair_color(5, &highlights, &config, SeriesType::Fibonacci);
        assert!(color.is_none());
    }
}
