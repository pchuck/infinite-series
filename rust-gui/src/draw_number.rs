//! Number rendering for point-based visualizations

use crate::config::VisualizerConfig;
use crate::config::SHOW_NUMBERS_MAX;
use crate::types::SeriesType;
use eframe::egui;
use std::collections::HashSet;

pub const MIN_CIRCLE_RADIUS: f32 = 0.5;
pub const MIN_SIZE_FOR_TEXT: f32 = 6.0;
pub const TEXT_SIZE_FACTOR: f32 = 0.6;

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

    let show_text =
        config.show_numbers && size >= MIN_SIZE_FOR_TEXT && config.max_number <= SHOW_NUMBERS_MAX;

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
