//! Individual visualization modules

pub mod cone_3d;
pub mod cube_3d;
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
pub mod prime_density;
pub mod prime_wheel;
pub mod pyramid_3d;
pub mod riemann;
pub mod row;
pub mod sacks;
pub mod sacks_mobius;
pub mod shared_3d;
pub mod sphere_3d;
pub mod torus_3d;
pub mod trefoil_3d;
pub mod triangular;
pub mod ulam;
pub mod ulam_mobius;

pub use cone_3d::draw as draw_cone_3d;
pub use cube_3d::draw as draw_cube_3d;
pub use cylinder_3d::draw as draw_cylinder_3d;
pub use density_gradient::draw as draw_density_gradient;
pub use dodecahedron_3d::draw as draw_dodecahedron_3d;
pub use fermats::{
    draw as draw_fermats, find_hovered as find_hovered_fermats,
    generate_positions as generate_fermats_positions,
};
pub use grid::{
    draw as draw_grid, find_hovered as find_hovered_grid,
    generate_positions as generate_grid_positions,
};
pub use helix_3d::draw as draw_helix_3d;
pub use hexagonal::{
    draw as draw_hexagonal, find_hovered as find_hovered_hexagonal,
    generate_positions as generate_hexagonal_positions,
};
pub use icosahedron_3d::draw as draw_icosahedron_3d;
pub use klein_3d::draw as draw_klein_3d;
pub use mobius_3d::draw as draw_mobius_3d;
pub use prime_density::draw as draw_prime_density;
pub use prime_wheel::{
    draw as draw_prime_wheel, find_hovered as find_hovered_prime_wheel,
    generate_positions as generate_prime_wheel_positions,
};
pub use pyramid_3d::draw as draw_pyramid_3d;
pub use riemann::draw as draw_riemann;
pub use row::{
    draw as draw_row, find_hovered as find_hovered_row,
    generate_positions as generate_row_positions,
};
pub use sacks::{
    draw as draw_sacks, find_hovered as find_hovered_sacks,
    generate_positions as generate_sacks_positions,
};
pub use sacks_mobius::draw as draw_sacks_mobius;
pub use sphere_3d::draw as draw_sphere_3d;
pub use torus_3d::draw as draw_torus_3d;
pub use trefoil_3d::draw as draw_trefoil_3d;
pub use triangular::{
    draw as draw_triangular, find_hovered as find_hovered_triangular,
    generate_positions as generate_triangular_positions,
};
pub use ulam::{
    draw as draw_ulam, find_hovered as find_hovered_ulam,
    generate_positions as generate_ulam_positions,
};
pub use ulam_mobius::draw as draw_ulam_mobius;
