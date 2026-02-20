//! Infinite series generators
//!
//! Provides generators for various infinite sequences:
//! - Fibonacci: F₀=0, F₁=1, Fₙ=Fₙ₋₁+Fₙ₋₂
//! - Lucas: L₀=2, L₁=1, Lₙ=Lₙ₋₁+Lₙ₋₂
//! - Triangular: Tₙ = n(n+1)/2
//! - Collatz: Stopping times (steps to reach 1)
//! - Powers of 2: 2ⁿ

pub mod collatz;
pub mod fibonacci;
pub mod lucas;
pub mod powers;
pub mod triangular;

pub use collatz::{collatz_stopping_time, generate_collatz_times, generate_collatz_times_up_to};
pub use fibonacci::{generate_fibonacci, generate_fibonacci_up_to, is_fibonacci};
pub use lucas::{generate_lucas, generate_lucas_up_to, is_lucas};
pub use powers::{generate_powers_of_2, generate_powers_of_2_up_to, is_power_of_2};
pub use triangular::{generate_triangular, generate_triangular_up_to, is_triangular};
