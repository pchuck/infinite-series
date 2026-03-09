//! Visualization registry for runtime lookup

#![allow(dead_code)]

use crate::app::NumberVisualizerApp;
use crate::config::VisualizerConfig;
use crate::types::{SeriesType, VisualizationType};
use eframe::egui;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::visualizations::params::VizParams;
use crate::visualizations::traits::Visualizer;

pub use crate::visualizations::{
    Cone3D, Cube3D, Cylinder3D, Dodecahedron3D, FermatsSpiral, Grid, Helix3D, HexagonalLattice,
    Icosahedron3D, Klein3D, Mobius3D, PrimeDensity, PrimeDensityGradient, PrimeWheel, Pyramid3D,
    RiemannZeta, Row, SacksMobiusSpiral, SacksSpiral, Sphere3D, Torus3D, Trefoil3D,
    TriangularLattice, UlamMobiusSpiral, UlamSpiral,
};

/// Registry for looking up visualizations by type
pub struct VisualizationRegistry {
    visualizers: HashMap<VisualizationType, Box<dyn Visualizer>>,
}

impl VisualizationRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            visualizers: HashMap::new(),
        };
        registry.register(UlamSpiral);
        registry.register(SacksSpiral);
        registry.register(Grid);
        registry.register(Row);
        registry.register(FermatsSpiral);
        registry.register(HexagonalLattice);
        registry.register(TriangularLattice);
        registry.register(PrimeWheel);
        registry.register(PrimeDensity);
        registry.register(RiemannZeta);
        registry.register(SacksMobiusSpiral);
        registry.register(UlamMobiusSpiral);
        registry.register(PrimeDensityGradient);
        registry.register(Sphere3D);
        registry.register(Torus3D);
        registry.register(Cone3D);
        registry.register(Cylinder3D);
        registry.register(Cube3D);
        registry.register(Mobius3D);
        registry.register(Klein3D);
        registry.register(Pyramid3D);
        registry.register(Dodecahedron3D);
        registry.register(Icosahedron3D);
        registry.register(Trefoil3D);
        registry.register(Helix3D);
        registry
    }

    pub fn register(&mut self, viz: impl Visualizer + 'static) {
        let viz_type = viz.viz_type();
        self.visualizers.insert(viz_type, Box::new(viz));
    }

    pub fn get(&self, viz_type: VisualizationType) -> Option<&dyn Visualizer> {
        self.visualizers
            .get(&viz_type)
            .map(|b| b.as_ref() as &dyn Visualizer)
    }

    pub fn available_for(&self, series: SeriesType) -> Vec<&dyn Visualizer> {
        self.visualizers
            .values()
            .filter(|v| v.supports_series(series))
            .map(|b| b.as_ref() as &dyn Visualizer)
            .collect()
    }

    pub fn all(&self) -> Vec<&dyn Visualizer> {
        self.visualizers
            .values()
            .map(|b| b.as_ref() as &dyn Visualizer)
            .collect()
    }
}

/// Global registry instance
pub static REGISTRY: LazyLock<VisualizationRegistry> = LazyLock::new(VisualizationRegistry::new);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SeriesType;

    #[test]
    fn test_registry_contains_all_visualization_types() {
        for viz_type in VisualizationType::ALL {
            let viz = REGISTRY.get(*viz_type);
            assert!(viz.is_some(), "Missing visualization: {:?}", viz_type);
        }
    }

    #[test]
    fn test_available_for_primes() {
        let available = REGISTRY.available_for(SeriesType::Primes);
        assert!(!available.is_empty(), "Primes should have visualizations");
    }

    #[test]
    fn test_available_for_fibonacci() {
        let available = REGISTRY.available_for(SeriesType::Fibonacci);
        assert!(
            !available.is_empty(),
            "Fibonacci should have visualizations"
        );
    }

    #[test]
    fn test_viz_type_name_matches() {
        for viz in REGISTRY.all() {
            let viz_type = viz.viz_type();
            assert_eq!(
                format!("{}", viz_type),
                viz.name(),
                "Name should match Display impl for {:?}",
                viz_type
            );
        }
    }
}
