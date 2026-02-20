//! Visualization configuration

use crate::types::VisualizationType;
use eframe::egui;

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
