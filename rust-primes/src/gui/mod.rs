pub mod app;
pub mod config;
pub mod draw_number;
pub mod helpers;
pub mod types;
pub mod visualizations;

pub use types::VisualizationType;

pub const MARGIN_SMALL: f32 = 20.0;
pub const SACKS_THETA_MULTIPLIER: f32 = 0.5;
pub const SACKS_MOBIUS_RADIUS_MULTIPLIER: f32 = 0.8;
pub const GOLDEN_ANGLE: f32 = 2.39996_f32;
pub const HOVER_THRESHOLD_DEFAULT: f32 = 0.7;
pub const HOVER_THRESHOLD_LARGE: f32 = 1.5;
