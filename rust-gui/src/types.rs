//! Series and visualization types

/// Represents different number sequences that can be visualized.
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

/// Represents different visualization layouts for number sequences.
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
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
    Sphere3D,
    Torus3D,
    Cone3D,
    Cylinder3D,
    Cube3D,
    Mobius3D,
    Klein3D,
    Pyramid3D,
    Dodecahedron3D,
    Icosahedron3D,
    Trefoil3D,
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
        VisualizationType::Sphere3D,
        VisualizationType::Torus3D,
        VisualizationType::Cone3D,
        VisualizationType::Cylinder3D,
        VisualizationType::Cube3D,
        VisualizationType::Mobius3D,
        VisualizationType::Klein3D,
        VisualizationType::Pyramid3D,
        VisualizationType::Dodecahedron3D,
        VisualizationType::Icosahedron3D,
        VisualizationType::Trefoil3D,
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
        VisualizationType::Sphere3D,
        VisualizationType::Torus3D,
        VisualizationType::Cone3D,
        VisualizationType::Cylinder3D,
        VisualizationType::Cube3D,
        VisualizationType::Mobius3D,
        VisualizationType::Klein3D,
        VisualizationType::Pyramid3D,
        VisualizationType::Dodecahedron3D,
        VisualizationType::Icosahedron3D,
        VisualizationType::Trefoil3D,
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
                | Self::Sphere3D
                | Self::Torus3D
                | Self::Cone3D
                | Self::Cylinder3D
                | Self::Cube3D
                | Self::Mobius3D
                | Self::Klein3D
                | Self::Pyramid3D
                | Self::Dodecahedron3D
                | Self::Icosahedron3D
                | Self::Trefoil3D
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
            Self::Sphere3D => "3D sphere with numbers distributed on surface, highlights bulge",
            Self::Torus3D => "3D torus (donut) with numbers wrapped around, highlights bulge",
            Self::Cone3D => "3D cone with numbers spiraling up, highlights spike outward",
            Self::Cylinder3D => "3D cylinder with numbers spiraling around, highlights spike",
            Self::Cube3D => "3D cube with numbers on faces, highlights bulge outward",
            Self::Mobius3D => "3D Mobius strip with numbers on twisted surface",
            Self::Klein3D => "3D Klein bottle (non-orientable surface), highlights bulge",
            Self::Pyramid3D => "3D pyramid with numbers on faces, highlights spike outward",
            Self::Dodecahedron3D => "3D dodecahedron (12 pentagonal faces), highlights bulge",
            Self::Icosahedron3D => "3D icosahedron (20 triangular faces), highlights bulge",
            Self::Trefoil3D => "3D trefoil knot (mathematical knot), highlights bulge",
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
                | Self::PrimeWheel
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
            VisualizationType::Sphere3D => write!(f, "3D Sphere"),
            VisualizationType::Torus3D => write!(f, "3D Torus"),
            VisualizationType::Cone3D => write!(f, "3D Cone"),
            VisualizationType::Cylinder3D => write!(f, "3D Cylinder"),
            VisualizationType::Cube3D => write!(f, "3D Cube"),
            VisualizationType::Mobius3D => write!(f, "3D Mobius Strip"),
            VisualizationType::Klein3D => write!(f, "3D Klein Bottle"),
            VisualizationType::Pyramid3D => write!(f, "3D Pyramid"),
            VisualizationType::Dodecahedron3D => write!(f, "3D Dodecahedron"),
            VisualizationType::Icosahedron3D => write!(f, "3D Icosahedron"),
            VisualizationType::Trefoil3D => write!(f, "3D Trefoil Knot"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_type_display() {
        assert_eq!(format!("{}", SeriesType::Primes), "Primes");
        assert_eq!(format!("{}", SeriesType::Fibonacci), "Fibonacci");
        assert_eq!(format!("{}", SeriesType::PowersOf2), "Powers of 2");
    }

    #[test]
    fn test_visualization_type_display() {
        assert_eq!(format!("{}", VisualizationType::UlamSpiral), "Ulam Spiral");
        assert_eq!(
            format!("{}", VisualizationType::SacksSpiral),
            "Sacks Spiral"
        );
    }

    #[test]
    fn test_visualization_is_primes_only() {
        assert!(VisualizationType::PrimeWheel.is_primes_only());
        assert!(VisualizationType::PrimeDensity.is_primes_only());
        assert!(VisualizationType::RiemannZeta.is_primes_only());
        assert!(!VisualizationType::UlamSpiral.is_primes_only());
        assert!(!VisualizationType::Grid.is_primes_only());
    }

    #[test]
    fn test_visualization_supports_hover() {
        assert!(VisualizationType::UlamSpiral.supports_hover());
        assert!(VisualizationType::SacksSpiral.supports_hover());
        assert!(!VisualizationType::PrimeDensity.supports_hover());
        assert!(!VisualizationType::RiemannZeta.supports_hover());
    }

    #[test]
    fn test_visualization_uses_point_rendering() {
        assert!(VisualizationType::UlamSpiral.uses_point_rendering());
        assert!(VisualizationType::PrimeWheel.uses_point_rendering());
        assert!(!VisualizationType::PrimeDensity.uses_point_rendering());
    }

    #[test]
    fn test_visualization_available_for() {
        let primes_viz = VisualizationType::available_for(SeriesType::Primes);
        assert!(primes_viz.contains(&VisualizationType::PrimeWheel));
        assert!(primes_viz.contains(&VisualizationType::RiemannZeta));

        let fib_viz = VisualizationType::available_for(SeriesType::Fibonacci);
        assert!(!fib_viz.contains(&VisualizationType::PrimeWheel));
        assert!(!fib_viz.contains(&VisualizationType::RiemannZeta));
    }
}
