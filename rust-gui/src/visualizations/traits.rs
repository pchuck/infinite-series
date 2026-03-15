//! Core traits for visualizations

#![allow(dead_code)]

use eframe::egui;

use crate::app::NumberVisualizerApp;
use crate::config::VisualizerConfig;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::shared_3d::Point3D;

/// Core trait for all visualizations
pub trait Visualizer: Send + Sync {
    /// Returns the visualization type identifier
    fn viz_type(&self) -> VisualizationType;

    /// Returns the display name
    fn name(&self) -> &'static str;

    /// Returns a description of the visualization
    fn description(&self) -> &'static str;

    /// Checks if this visualization supports the given series type
    fn supports_series(&self, series: SeriesType) -> bool;

    /// Whether this visualization supports hover detection
    fn supports_hover(&self) -> bool;

    /// Whether this visualization uses point-based rendering
    fn uses_point_rendering(&self) -> bool;

    /// Generate positions for all numbers from 1 to max_n
    fn generate_positions(&self, max_n: usize, params: &VizParams) -> Vec<(usize, f32, f32)>;

    /// Draw the visualization
    fn draw(
        &self,
        app: &mut NumberVisualizerApp,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        positions: &[(usize, f32, f32)],
    );

    /// Find the number at the given mouse position
    fn find_hovered(
        &self,
        _app: &NumberVisualizerApp,
        _mouse_pos: egui::Pos2,
        _rect: egui::Rect,
        _positions: &[(usize, f32, f32)],
    ) -> Option<usize> {
        None
    }

    /// Add configuration UI controls for this visualization
    fn config_ui(&self, _ui: &mut egui::Ui, _config: &mut VisualizerConfig, _series: SeriesType) {}
}

/// Trait for 3D visualizations
pub trait Visualizer3D: Visualizer {
    /// Generate a 3D point for a number
    fn generate_point(&self, n: usize, is_highlighted: bool) -> Point3D;

    /// Draw the 3D visualization
    fn draw_3d(&self, app: &mut NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect);
}
