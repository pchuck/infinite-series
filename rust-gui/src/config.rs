//! Visualization configuration

use crate::types::VisualizationType;
use eframe::egui;
use std::collections::HashMap;

// Re-export constants for backward compatibility
pub use crate::constants::layout::*;
pub use crate::constants::limits::*;

/// Per-visualization 3D rotation settings.
#[derive(Clone, Copy, Debug)]
pub struct VisualizationSettings {
    pub rotation_x: f32,
    pub rotation_y: f32,
}

impl Default for VisualizationSettings {
    fn default() -> Self {
        Self {
            rotation_x: 0.4,
            rotation_y: 0.0,
        }
    }
}

/// Stores per-visualization configuration settings.
#[derive(Clone, Default)]
pub struct PerVisualizationConfig {
    pub settings: HashMap<VisualizationType, VisualizationSettings>,
    pub position_cache: HashMap<VisualizationType, CachedPosition>,
}

#[derive(Clone, Debug)]
pub struct CachedPosition {
    pub positions: Vec<(usize, f32, f32)>,
    pub max_number: usize,
    pub modulo: usize,
}

#[allow(dead_code)]
impl PerVisualizationConfig {
    #[allow(dead_code)]
    /// Get the settings for a specific visualization type.
    ///
    /// Returns default settings if no custom settings have been stored for this type.
    pub fn get(&self, viz_type: VisualizationType) -> VisualizationSettings {
        self.settings
            .get(&viz_type)
            .copied()
            .unwrap_or_else(VisualizationSettings::default)
    }

    /// Store settings for a specific visualization type.
    pub fn set(&mut self, viz_type: VisualizationType, settings: VisualizationSettings) {
        self.settings.insert(viz_type, settings);
    }

    pub fn get_positions(&self, viz_type: VisualizationType) -> Option<&Vec<(usize, f32, f32)>> {
        self.position_cache.get(&viz_type).map(|c| &c.positions)
    }

    pub fn set_positions(
        &mut self,
        viz_type: VisualizationType,
        positions: Vec<(usize, f32, f32)>,
        max_number: usize,
        modulo: usize,
    ) {
        self.position_cache.insert(
            viz_type,
            CachedPosition {
                positions,
                max_number,
                modulo,
            },
        );
    }

    pub fn invalidate_positions(&mut self, viz_type: VisualizationType) {
        self.position_cache.remove(&viz_type);
    }

    pub fn invalidate_all_positions(&mut self) {
        self.position_cache.clear();
    }
}

/// Prime pair types for color blending.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimePairType {
    Twin,
    Cousin,
    Sexy,
}

/// Cached blended colors for prime pairs.
#[derive(Clone, Debug)]
pub struct PrimePairColors {
    pub twin: egui::Color32,
    pub cousin: egui::Color32,
    pub sexy: egui::Color32,
    pub twin_cousin: egui::Color32,
    pub twin_sexy: egui::Color32,
    pub cousin_sexy: egui::Color32,
    pub twin_cousin_sexy: egui::Color32,
}

impl PrimePairColors {
    /// Create a new PrimePairColors with the given base colors.
    pub fn new(twin: egui::Color32, cousin: egui::Color32, sexy: egui::Color32) -> Self {
        Self {
            twin,
            cousin,
            sexy,
            twin_cousin: blend_colors(twin, cousin),
            twin_sexy: blend_colors(twin, sexy),
            cousin_sexy: blend_colors(cousin, sexy),
            twin_cousin_sexy: blend_three_colors(twin, cousin, sexy),
        }
    }

    /// Get the color for a prime that belongs to the given pair types.
    pub fn get_color(&self, pair_types: &[PrimePairType]) -> egui::Color32 {
        if pair_types.is_empty() {
            return self.twin;
        }

        match pair_types.len() {
            1 => match pair_types[0] {
                PrimePairType::Twin => self.twin,
                PrimePairType::Cousin => self.cousin,
                PrimePairType::Sexy => self.sexy,
            },
            2 => {
                let mut has_twin = false;
                let mut has_cousin = false;
                let mut has_sexy = false;
                for &pt in pair_types {
                    match pt {
                        PrimePairType::Twin => has_twin = true,
                        PrimePairType::Cousin => has_cousin = true,
                        PrimePairType::Sexy => has_sexy = true,
                    }
                }
                if has_twin && has_cousin {
                    self.twin_cousin
                } else if has_twin && has_sexy {
                    self.twin_sexy
                } else {
                    self.cousin_sexy
                }
            }
            3 => self.twin_cousin_sexy,
            _ => self.twin,
        }
    }

    /// Recompute blended colors when base colors change.
    pub fn recompute(&mut self, twin: egui::Color32, cousin: egui::Color32, sexy: egui::Color32) {
        *self = Self::new(twin, cousin, sexy);
    }
}

fn blend_colors(c1: egui::Color32, c2: egui::Color32) -> egui::Color32 {
    let r = (c1.r() as f32 * 0.5 + c2.r() as f32 * 0.5) as u8;
    let g = (c1.g() as f32 * 0.5 + c2.g() as f32 * 0.5) as u8;
    let b = (c1.b() as f32 * 0.5 + c2.b() as f32 * 0.5) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, 255)
}

fn blend_three_colors(c1: egui::Color32, c2: egui::Color32, c3: egui::Color32) -> egui::Color32 {
    let r = (c1.r() as f32 / 3.0 + c2.r() as f32 / 3.0 + c3.r() as f32 / 3.0) as u8;
    let g = (c1.g() as f32 / 3.0 + c2.g() as f32 / 3.0 + c3.g() as f32 / 3.0) as u8;
    let b = (c1.b() as f32 / 3.0 + c2.b() as f32 / 3.0 + c3.b() as f32 / 3.0) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, 255)
}

/// Main configuration for the number sequence visualizer.
#[derive(Clone)]
pub struct VisualizerConfig {
    pub max_number: usize,
    pub highlight_size: usize,
    pub non_highlight_size: usize,
    pub modulo: usize,
    pub show_numbers: bool,
    pub highlight_color: egui::Color32,
    pub non_highlight_color: egui::Color32,
    pub background_color: egui::Color32,
    pub visualization: VisualizationType,
    pub num_zeros: usize,
    pub show_twin_primes: bool,
    pub twin_color: egui::Color32,
    pub show_cousin_primes: bool,
    pub cousin_color: egui::Color32,
    pub show_sexy_primes: bool,
    pub sexy_color: egui::Color32,
    pub grid_size: usize,
    #[doc(hidden)]
    pub prime_pair_colors: PrimePairColors,
}

impl Default for VisualizerConfig {
    fn default() -> Self {
        let twin_color = egui::Color32::from_rgba_unmultiplied(255, 50, 50, 255);
        let cousin_color = egui::Color32::from_rgba_unmultiplied(255, 120, 120, 255);
        let sexy_color = egui::Color32::from_rgba_unmultiplied(255, 180, 180, 255);

        Self {
            max_number: MAX_NUMBER_DEFAULT,
            highlight_size: HIGHLIGHT_SIZE_DEFAULT,
            non_highlight_size: NON_HIGHLIGHT_SIZE_DEFAULT,
            modulo: MODULO_DEFAULT,
            show_numbers: false,
            highlight_color: egui::Color32::from_rgba_unmultiplied(255, 220, 80, 255),
            non_highlight_color: egui::Color32::from_rgba_unmultiplied(60, 60, 70, 180),
            background_color: egui::Color32::from_rgba_unmultiplied(20, 20, 30, 255),
            visualization: VisualizationType::UlamSpiral,
            num_zeros: NUM_ZEROS_DEFAULT,
            show_twin_primes: false,
            twin_color,
            show_cousin_primes: false,
            cousin_color,
            show_sexy_primes: false,
            sexy_color,
            grid_size: GRID_SIZE_DEFAULT,
            prime_pair_colors: PrimePairColors::new(twin_color, cousin_color, sexy_color),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_two_colors() {
        let c1 = egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255);
        let c2 = egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255);
        let blended = blend_colors(c1, c2);
        assert_eq!(blended.r(), 127);
        assert_eq!(blended.g(), 127);
        assert_eq!(blended.b(), 0);
    }

    #[test]
    fn test_blend_three_colors() {
        let c1 = egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255);
        let c2 = egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255);
        let c3 = egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255);
        let blended = blend_three_colors(c1, c2, c3);
        assert_eq!(blended.r(), 85);
        assert_eq!(blended.g(), 85);
        assert_eq!(blended.b(), 85);
    }

    #[test]
    fn test_prime_pair_colors_single_type() {
        let colors = PrimePairColors::new(
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255),
        );
        assert_eq!(
            colors.get_color(&[PrimePairType::Twin]),
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255)
        );
        assert_eq!(
            colors.get_color(&[PrimePairType::Cousin]),
            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255)
        );
        assert_eq!(
            colors.get_color(&[PrimePairType::Sexy]),
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255)
        );
    }

    #[test]
    fn test_prime_pair_colors_two_types() {
        let colors = PrimePairColors::new(
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255),
        );
        let twin_cousin = colors.get_color(&[PrimePairType::Twin, PrimePairType::Cousin]);
        assert_eq!(twin_cousin.r(), 127);
        assert_eq!(twin_cousin.g(), 127);
        assert_eq!(twin_cousin.b(), 0);

        let twin_sexy = colors.get_color(&[PrimePairType::Twin, PrimePairType::Sexy]);
        assert_eq!(twin_sexy.r(), 127);
        assert_eq!(twin_sexy.g(), 0);
        assert_eq!(twin_sexy.b(), 127);

        let cousin_sexy = colors.get_color(&[PrimePairType::Cousin, PrimePairType::Sexy]);
        assert_eq!(cousin_sexy.r(), 0);
        assert_eq!(cousin_sexy.g(), 127);
        assert_eq!(cousin_sexy.b(), 127);
    }

    #[test]
    fn test_prime_pair_colors_three_types() {
        let colors = PrimePairColors::new(
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255),
        );
        let blended = colors.get_color(&[
            PrimePairType::Twin,
            PrimePairType::Cousin,
            PrimePairType::Sexy,
        ]);
        assert_eq!(blended.r(), 85);
        assert_eq!(blended.g(), 85);
        assert_eq!(blended.b(), 85);
    }

    #[test]
    fn test_prime_pair_colors_order_independent() {
        let colors = PrimePairColors::new(
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255),
        );
        let c1 = colors.get_color(&[PrimePairType::Twin, PrimePairType::Cousin]);
        let c2 = colors.get_color(&[PrimePairType::Cousin, PrimePairType::Twin]);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_recompute_colors() {
        let mut colors = PrimePairColors::new(
            egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 255, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255),
        );
        colors.recompute(
            egui::Color32::from_rgba_unmultiplied(100, 0, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 100, 0, 255),
            egui::Color32::from_rgba_unmultiplied(0, 0, 100, 255),
        );
        let twin_cousin = colors.get_color(&[PrimePairType::Twin, PrimePairType::Cousin]);
        assert_eq!(twin_cousin.r(), 50);
        assert_eq!(twin_cousin.g(), 50);
        assert_eq!(twin_cousin.b(), 0);
    }
}
