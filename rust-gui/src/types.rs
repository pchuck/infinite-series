//! Series and visualization types

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SeriesType {
    #[default]
    Primes,
    Fibonacci,
    Lucas,
    Triangular,
    Collatz,
    PowersOf2,
    Catalan,
    Hexagonal,
    Happy,
}

impl SeriesType {
    pub const ALL: &'static [SeriesType] = &[
        SeriesType::Primes,
        SeriesType::Fibonacci,
        SeriesType::Lucas,
        SeriesType::Triangular,
        SeriesType::Collatz,
        SeriesType::PowersOf2,
        SeriesType::Catalan,
        SeriesType::Hexagonal,
        SeriesType::Happy,
    ];
}

impl std::fmt::Display for SeriesType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeriesType::Primes => write!(f, "Primes"),
            SeriesType::Fibonacci => write!(f, "Fibonacci"),
            SeriesType::Lucas => write!(f, "Lucas"),
            SeriesType::Triangular => write!(f, "Triangular"),
            SeriesType::Collatz => write!(f, "Collatz"),
            SeriesType::PowersOf2 => write!(f, "Powers of 2"),
            SeriesType::Catalan => write!(f, "Catalan"),
            SeriesType::Hexagonal => write!(f, "Hexagonal"),
            SeriesType::Happy => write!(f, "Happy"),
        }
    }
}

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
    Helix3D,
}

impl VisualizationType {
    pub const ALL: &'static [VisualizationType] = &[
        VisualizationType::UlamSpiral,
        VisualizationType::SacksSpiral,
        VisualizationType::Grid,
        VisualizationType::Row,
        VisualizationType::PrimeWheel,
        VisualizationType::PrimeDensity,
        VisualizationType::RiemannZeta,
        VisualizationType::HexagonalLattice,
        VisualizationType::TriangularLattice,
        VisualizationType::FermatsSpiral,
        VisualizationType::SacksMobiusSpiral,
        VisualizationType::UlamMobiusSpiral,
        VisualizationType::PrimeDensityGradient,
        VisualizationType::Helix3D,
    ];

    pub const GENERAL: &'static [VisualizationType] = &[
        VisualizationType::UlamSpiral,
        VisualizationType::SacksSpiral,
        VisualizationType::Grid,
        VisualizationType::Row,
        VisualizationType::HexagonalLattice,
        VisualizationType::TriangularLattice,
        VisualizationType::FermatsSpiral,
        VisualizationType::Helix3D,
    ];

    pub fn available_for(series: SeriesType) -> &'static [VisualizationType] {
        match series {
            SeriesType::Primes => Self::ALL,
            SeriesType::Fibonacci
            | SeriesType::Lucas
            | SeriesType::Triangular
            | SeriesType::Collatz
            | SeriesType::PowersOf2
            | SeriesType::Catalan
            | SeriesType::Hexagonal
            | SeriesType::Happy => Self::GENERAL,
        }
    }

    pub fn is_primes_only(self) -> bool {
        matches!(
            self,
            Self::PrimeWheel
                | Self::PrimeDensity
                | Self::RiemannZeta
                | Self::SacksMobiusSpiral
                | Self::UlamMobiusSpiral
                | Self::PrimeDensityGradient
        )
    }

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
                | Self::Helix3D
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
            Self::UlamSpiral => "Classic diagonal pattern on a square grid spiral",
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
            Self::Helix3D => "3D spiral helix with highlighted numbers spiking outward",
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
            VisualizationType::Helix3D => write!(f, "3D Helix"),
        }
    }
}
