//! Visualization parameters for flexible configuration

#![allow(dead_code)]

use std::collections::HashMap;

/// Unified parameters for visualization configuration.
///
/// This allows adding new parameters without breaking existing visualizations.
#[derive(Clone, Default)]
pub struct VizParams {
    /// Modulo value for PrimeWheel visualization
    pub modulo: Option<usize>,
    /// Grid size for DensityGradient visualization
    pub grid_size: Option<usize>,
    /// Number of zeros to show for Riemann visualization
    pub num_zeros: Option<usize>,
    /// Custom parameters for future visualizations
    pub custom: HashMap<String, f32>,
}

impl VizParams {
    pub fn with_modulo(mut self, modulo: usize) -> Self {
        self.modulo = Some(modulo);
        self
    }

    pub fn with_grid_size(mut self, size: usize) -> Self {
        self.grid_size = Some(size);
        self
    }

    pub fn with_num_zeros(mut self, num: usize) -> Self {
        self.num_zeros = Some(num);
        self
    }

    pub fn with_custom(mut self, key: impl Into<String>, value: f32) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
}
