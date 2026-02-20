# Number Sequence Visualizer

An interactive infinite number sequence visualizer, implemented in Rust and backed by high-performance parallel series generators.

![Rust Number Sequence Visualizer Screenshot](./resources/rust_prime_visualizer_sacks_spiral_screenshot.png)

## Quick Start

### Make
```bash
make run        # Run debug
make release    # Build optimized
make run-release
```

### Cargo
```bash
cargo run
```

## Supported Series

| Series | Description |
|--------|-------------|
| **Primes** | Prime numbers (2, 3, 5, 7, 11, ...) |
| **Fibonacci** | Fibonacci sequence (0, 1, 1, 2, 3, 5, 8, ...) |
| **Lucas** | Lucas sequence (2, 1, 3, 4, 7, 11, 18, ...) |
| **Triangular** | Triangular numbers (0, 1, 3, 6, 10, 15, 21, ...) |
| **Collatz** | Stopping times (0, 0, 1, 7, 2, 5, 8, 16, ...) |
| **Powers of 2** | Powers of 2 (1, 2, 4, 8, 16, 32, 64, ...) |

## Visualizations

### Available for All Series
| Visualization | Description |
|--------------|-------------|
| **Ulam Spiral** | Classic diagonal pattern on a square grid spiral |
| **Sacks Spiral** | Archimedean spiral (r = sqrt(n)) revealing curved patterns |
| **Grid** | Simple Cartesian square grid layout |
| **Row** | Single horizontal number line |
| **Hexagonal Lattice** | 6-direction symmetric spiral on hexagonal grid |
| **Triangular Lattice** | 3-direction symmetric spiral on triangular grid |
| **Fermat's Spiral** | Phyllotaxis spiral with golden angle (sunflower pattern) |

### Primes-Only Visualizations
| Visualization | Description |
|--------------|-------------|
| **Prime Wheel** | Concentric rings colored by modulo residue |
| **Prime Density** | Graph of pi(x) vs x/ln(x) - Prime Number Theorem |
| **Riemann Zeta** | Critical strip showing non-trivial zeros |
| **Sacks Mobius Spiral** | Archimedean spiral with gap-colored lines |
| **Ulam Mobius Spiral** | Square-grid spiral with gap-colored lines |
| **Prime Density Gradient** | Heatmap grid showing local prime density |

## Controls

- **Series Type**: Switch between Primes, Fibonacci, Lucas, Triangular, Collatz, and Powers of 2
- **Visualization**: Select the visualization type
- **Max Number**: Set the upper bound for the sequence
- **Display**: Adjust point sizes, colors, and visibility options

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Project Structure

```
rust-gui/
├── Cargo.toml
├── Makefile
├── README.md
└── src/
    ├── main.rs              # Entry point
    ├── app.rs               # Main application and UI
    ├── config.rs            # Visualization configuration
    ├── draw_number.rs       # Number rendering
    ├── helpers.rs           # Utility constants
    ├── types.rs             # Series and visualization types
    └── visualizations/
        ├── mod.rs
        ├── ulam.rs
        ├── sacks.rs
        ├── grid.rs
        ├── row.rs
        ├── hexagonal.rs
        ├── triangular.rs
        ├── fermats.rs
        ├── prime_wheel.rs
        ├── prime_density.rs
        ├── riemann.rs
        ├── sacks_mobius.rs
        ├── ulam_mobius.rs
        └── density_gradient.rs
```

## Dependencies

- `eframe` - GUI framework
- `primes` - Local path dependency (../rust-primes)
- `series` - Local path dependency (../rust-series)

## Algorithms / Credits

* **Ulam Spiral** - Classic diagonal prime pattern - primes form distinctive diagonal lines (Stanislaw Ulam, 1963) 
* **Sacks Spiral** - Archimedean spiral (radius = sqrt(n)) - reveals curved patterns in prime distribution (Robert Sacks, 1994, numberspiral.com) 
* **Prime Wheel** - Concentric rings by modulo - primes cluster on spokes coprime to the modulus 
* **Prime Density** - Graph of π(x) vs x/ln(x) - visualizes the Prime Number Theorem (prime counting function vs approximation) 
* **Riemann Zeta** - Critical strip plot showing non-trivial zeros on the critical line σ=0.5 - visualizes the connection between prime distribution and the Riemann Hypothesis
* **Hexagonal Lattice** - Hexagonal lattice spiral - symmetric 6-direction spiral on hexagonal grid (60° intervals)
* **Triangular Lattice** - Triangular lattice spiral - symmetric 3-direction spiral on triangular grid (120° intervals) 
* **Fermat's Spiral** - Phyllotaxis spiral - golden angle placement (r = sqrt(n), theta = n * 137.5°), same pattern as sunflower seed arrangements 
* **Sacks Mobius Spiral** - Archimedean spiral using prime index with gap-colored lines (white=close, gray=far) 
* **Ulam Mobius Spiral** - Square-grid spiral using prime index with gap-colored lines (white=close, gray=far) 
* **Prime Density Gradient** - Heatmap grid showing local prime density across the number space 
