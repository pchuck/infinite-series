//! Visualization types and their UI properties

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum VisualizationType {
    #[default]
    UlamSpiral,
    SacksSpiral,
    Grid,
    Row,
    PrimeWheel,
    PrimeDensity,
    RiemannZeta,
    HexagonalLattice,
    TriangularLattice,
    FermatsSpiral,
    SacksMobiusSpiral,
    UlamMobiusSpiral,
    PrimeDensityGradient,
}

impl VisualizationType {
    pub fn uses_point_rendering(self) -> bool {
        matches!(
            self,
            Self::UlamSpiral
                | Self::SacksSpiral
                | Self::Grid
                | Self::Row
                | Self::PrimeWheel
                | Self::HexagonalLattice
                | Self::TriangularLattice
                | Self::FermatsSpiral
                | Self::SacksMobiusSpiral
                | Self::UlamMobiusSpiral
        )
    }

    pub fn uses_modulo(self) -> bool {
        matches!(self, Self::PrimeWheel)
    }

    pub fn uses_num_zeros(self) -> bool {
        matches!(self, Self::RiemannZeta)
    }

    pub fn uses_grid_size(self) -> bool {
        matches!(self, Self::PrimeDensityGradient)
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::UlamSpiral => "Classic diagonal prime pattern on a square grid spiral",
            Self::SacksSpiral => "Archimedean spiral (r = sqrt(n)) revealing curved patterns",
            Self::Grid => "Simple Cartesian square grid layout",
            Self::Row => "Single horizontal number line",
            Self::PrimeWheel => "Concentric rings colored by modulo residue",
            Self::PrimeDensity => "Graph of pi(x) vs x/ln(x) - Prime Number Theorem",
            Self::RiemannZeta => "Critical strip showing non-trivial zeros on sigma=0.5",
            Self::HexagonalLattice => "6-direction symmetric spiral on hexagonal grid",
            Self::TriangularLattice => "3-direction symmetric spiral on triangular grid",
            Self::FermatsSpiral => "Phyllotaxis spiral with golden angle (sunflower pattern)",
            Self::SacksMobiusSpiral => {
                "Archimedean spiral with gap-colored lines between consecutive primes"
            }
            Self::UlamMobiusSpiral => {
                "Square-grid spiral with gap-colored lines between consecutive primes"
            }
            Self::PrimeDensityGradient => "Heatmap grid showing local prime density",
        }
    }

    pub fn supports_hover(self) -> bool {
        matches!(
            self,
            Self::UlamSpiral
                | Self::SacksSpiral
                | Self::Grid
                | Self::Row
                | Self::FermatsSpiral
                | Self::HexagonalLattice
                | Self::TriangularLattice
        )
    }
}

impl std::fmt::Display for VisualizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualizationType::UlamSpiral => write!(f, "Ulam Spiral"),
            VisualizationType::SacksSpiral => write!(f, "Sacks Spiral"),
            VisualizationType::Grid => write!(f, "Grid"),
            VisualizationType::Row => write!(f, "Row"),
            VisualizationType::PrimeWheel => write!(f, "Prime Wheel"),
            VisualizationType::PrimeDensity => write!(f, "Prime Density"),
            VisualizationType::RiemannZeta => write!(f, "Riemann Zeta"),
            VisualizationType::HexagonalLattice => write!(f, "Hexagonal Lattice"),
            VisualizationType::TriangularLattice => write!(f, "Triangular Lattice"),
            VisualizationType::FermatsSpiral => write!(f, "Fermat's Spiral"),
            VisualizationType::SacksMobiusSpiral => write!(f, "Sacks Mobius Spiral"),
            VisualizationType::UlamMobiusSpiral => write!(f, "Ulam Mobius Spiral"),
            VisualizationType::PrimeDensityGradient => write!(f, "Prime Density Gradient"),
        }
    }
}
