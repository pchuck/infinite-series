//! Individual visualization modules
#![allow(unused_imports)]

//! Layout computation patterns:
//! - (f32, f32, f32) - center_x, center_y, scale: Simple centering for spirals
//! - (f32, f32, f32) - start_x, start_y, scale: Grid/linear layouts
//! - (f32, f32, f32, f32) - center_x, center_y, scale, extra: Polar/spiral with extra param
//! - (f32, f32, f32, f32, f32) - center_x, center_y, scale, mid_x, mid_y: Bounding box centering

pub mod cone_3d;
pub mod cube_quadratic_3d;
pub mod cube_simple_3d;
pub mod cylinder_3d;
pub mod density_gradient;
pub mod dodecahedron_3d;
pub mod fermats;
pub mod grid;
pub mod helix_3d;
pub mod hexagonal;
pub mod icosahedron_3d;
pub mod klein_3d;
pub mod mobius_3d;
pub mod params;
pub mod prime_density;
pub mod prime_wheel;
pub mod pyramid_3d;
pub mod registry;
pub mod riemann;
pub mod row;
pub mod sacks;
pub mod sacks_mobius;
pub mod shared_3d;
pub mod sphere_3d;
pub mod torus_3d;
pub mod traits;
pub mod trefoil_3d;
pub mod triangular;
pub mod ulam;
pub mod ulam_mobius;

pub use params::VizParams;
pub use registry::{VisualizationRegistry, REGISTRY};
pub use traits::{Visualizer, Visualizer3D};

pub use cone_3d::{draw as draw_cone_3d, Cone3D};
pub use cube_quadratic_3d::{draw as draw_cube_quadratic_3d, CubeQuadratic3D};
pub use cube_simple_3d::{draw as draw_cube_simple_3d, CubeSimple3D};
pub use cylinder_3d::{draw as draw_cylinder_3d, Cylinder3D};
pub use density_gradient::{draw as draw_density_gradient, PrimeDensityGradient};
pub use dodecahedron_3d::{draw as draw_dodecahedron_3d, Dodecahedron3D};
pub use fermats::{
    draw as draw_fermats, find_hovered as find_hovered_fermats,
    generate_positions as generate_fermats_positions, FermatsSpiral,
};
pub use grid::{
    draw as draw_grid, find_hovered as find_hovered_grid,
    generate_positions as generate_grid_positions, Grid,
};
pub use helix_3d::{draw as draw_helix_3d, Helix3D};
pub use hexagonal::{
    draw as draw_hexagonal, find_hovered as find_hovered_hexagonal,
    generate_positions as generate_hexagonal_positions, HexagonalLattice,
};
pub use icosahedron_3d::{draw as draw_icosahedron_3d, Icosahedron3D};
pub use klein_3d::{draw as draw_klein_3d, Klein3D};
pub use mobius_3d::{draw as draw_mobius_3d, Mobius3D};
pub use prime_density::{draw as draw_prime_density, PrimeDensity};
pub use prime_wheel::{
    draw as draw_prime_wheel, find_hovered as find_hovered_prime_wheel,
    generate_positions as generate_prime_wheel_positions, PrimeWheel,
};
pub use pyramid_3d::{draw as draw_pyramid_3d, Pyramid3D};
pub use riemann::{draw as draw_riemann, RiemannZeta};
pub use row::{
    draw as draw_row, find_hovered as find_hovered_row,
    generate_positions as generate_row_positions, Row,
};
pub use sacks::{
    draw as draw_sacks, find_hovered as find_hovered_sacks,
    generate_positions as generate_sacks_positions, SacksSpiral,
};
pub use sacks_mobius::{draw as draw_sacks_mobius, SacksMobiusSpiral};
pub use sphere_3d::{draw as draw_sphere_3d, Sphere3D};
pub use torus_3d::{draw as draw_torus_3d, Torus3D};
pub use trefoil_3d::{draw as draw_trefoil_3d, Trefoil3D};
pub use triangular::{
    draw as draw_triangular, find_hovered as find_hovered_triangular,
    generate_positions as generate_triangular_positions, TriangularLattice,
};
pub use ulam::{
    draw as draw_ulam, find_hovered as find_hovered_ulam,
    generate_positions as generate_ulam_positions, UlamSpiral,
};
pub use ulam_mobius::{draw as draw_ulam_mobius, UlamMobiusSpiral};
