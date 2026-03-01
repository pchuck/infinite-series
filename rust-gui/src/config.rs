//! Visualization configuration

use crate::types::VisualizationType;
use eframe::egui;
use std::collections::HashMap;

pub const MAX_NUMBER_MIN: usize = 100;
pub const MAX_NUMBER_MAX: usize = 100000;
pub const SHOW_NUMBERS_MAX: usize = 10000;
pub const DENSITY_INTERVALS: usize = 100;
pub const SIDE_PANEL_MIN_WIDTH: f32 = 250.0;

// UI Layout constants
pub const UI_MARGIN: f32 = 5.0;
pub const ERROR_BOX_HEIGHT: f32 = 30.0;
pub const HOVER_TEXT_OFFSET_Y: f32 = 20.0;
pub const FONT_SIZE_DEFAULT: f32 = 14.0;

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

#[derive(Clone, Default)]
pub struct PerVisualizationConfig {
    pub settings: HashMap<VisualizationType, VisualizationSettings>,
}

impl PerVisualizationConfig {
    pub fn get(&self, viz_type: VisualizationType) -> VisualizationSettings {
        self.settings
            .get(&viz_type)
            .copied()
            .unwrap_or_else(VisualizationSettings::default)
    }

    pub fn set(&mut self, viz_type: VisualizationType, settings: VisualizationSettings) {
        self.settings.insert(viz_type, settings);
    }
}

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
}

impl Default for VisualizerConfig {
    fn default() -> Self {
        Self {
            max_number: 10000,
            highlight_size: 2,
            non_highlight_size: 1,
            modulo: 30,
            show_numbers: false,
            highlight_color: egui::Color32::from_rgba_unmultiplied(255, 220, 80, 255),
            non_highlight_color: egui::Color32::from_rgba_unmultiplied(60, 60, 70, 180),
            background_color: egui::Color32::from_rgba_unmultiplied(20, 20, 30, 255),
            visualization: VisualizationType::UlamSpiral,
            num_zeros: 10,
            show_twin_primes: false,
            twin_color: egui::Color32::from_rgba_unmultiplied(255, 50, 50, 255),
            show_cousin_primes: false,
            cousin_color: egui::Color32::from_rgba_unmultiplied(255, 120, 120, 255),
            show_sexy_primes: false,
            sexy_color: egui::Color32::from_rgba_unmultiplied(255, 180, 180, 255),
            grid_size: 40,
        }
    }
}
