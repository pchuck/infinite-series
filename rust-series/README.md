# Infinite Series Generator

High-performance generators for various infinite number series with CLI interface.

## Supported Series

| Series | Flag | Formula/Rule | Sequence |
|--------|------|--------------|----------|
| **Fibonacci** | `fib` | Fₙ = Fₙ₋₁ + Fₙ₋₂ | 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ... |
| **Lucas** | `lucas` | Lₙ = Lₙ₋₁ + Lₙ₋₂ | 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, ... |
| **Triangular** | `tri` | Tₙ = n(n+1)/2 | 0, 1, 3, 6, 10, 15, 21, 28, 36, 45, ... |
| **Collatz** | `collatz` | Steps to reach 1 | 0, 0, 1, 7, 2, 5, 8, 16, 3, 19, ... |
| **Powers of 2** | `pow2` | 2ⁿ | 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, ... |
| **Catalan** | `catalan` | Cₙ = (2n)!/(n!(n+1)!) | 1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, ... |
| **Hexagonal** | `hex` | Hₙ = n(2n-1) | 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, ... |
| **Happy** | `happy` | Digit-square sum reaches 1 | 1, 7, 10, 13, 19, 23, 28, 31, 32, 44, ... |

## Quick Start

### Make
```bash
make help
make release            # Build optimized
make run S=fib          # Fibonacci (default)
make run S=lucas        # Lucas numbers
make run S=tri          # Triangular numbers
make run S=collatz      # Collatz stopping times
make run S=pow2         # Powers of 2
make run S=catalan      # Catalan numbers
make run S=hex          # Hexagonal numbers
make run S=happy        # Happy numbers
make test               # Run all tests
```

### Cargo

```bash
# Build
cargo build

# Run CLI
cargo run -- -c 100 -s fib       # Fibonacci
cargo run -- -c 100 -s lucas     # Lucas
cargo run -- -c 100 -s tri       # Triangular
cargo run -- -c 100 -s collatz   # Collatz
cargo run -- -c 100 -s pow2      # Powers of 2
cargo run -- -c 100 -s catalan   # Catalan
cargo run -- -c 100 -s hex       # Hexagonal
cargo run -- -c 100 -s happy     # Happy

# Run tests
cargo test
```

## CLI Usage

```bash
# Generate first 10 Fibonacci numbers
cargo run -- -c 10 -s fib

# Generate first 20 Lucas numbers
cargo run -- -c 20 -s lucas

# Generate first 50 triangular numbers
cargo run -- -c 50 -s tri

# Generate collatz stopping times for 1-100
cargo run -- -c 100 -s collatz

# Generate first 15 Catalan numbers
cargo run -- -c 15 -s catalan

# Generate first 20 hexagonal numbers
cargo run -- -c 20 -s hex

# Generate first 20 happy numbers
cargo run -- -c 20 -s happy

# Generate first 20 powers of 2
cargo run -- -c 20 -s pow2

# Quiet mode - count only
cargo run -- -c 100 -s fib --quiet

# With progress bar
cargo run -- -c 1000 -s fib --progress
```

### CLI Options

| Option | Description |
|--------|-------------|
| `-c, --count` | Number of values to generate |
| `-s, --series` | Series type: fib, lucas, tri, collatz, pow2, catalan, hex, happy |
| `-q, --quiet` | Only print count (no number list) |
| `-P, --progress` | Show progress bar |

## Library API

```rust
use series::{
    generate_fibonacci, generate_fibonacci_up_to, is_fibonacci,
    generate_lucas, generate_lucas_up_to, is_lucas,
    generate_triangular, generate_triangular_up_to, is_triangular,
    collatz_stopping_time, generate_collatz_times, generate_collatz_times_up_to,
    generate_powers_of_2, generate_powers_of_2_up_to, is_power_of_2,
    generate_catalan, generate_catalan_up_to, is_catalan,
    generate_hexagonal, generate_hexagonal_up_to, is_hexagonal,
    generate_happy, generate_happy_up_to, is_happy,
};

// Generate first N values
let fibs = generate_fibonacci(10);
let lucas = generate_lucas(10);
let tri = generate_triangular(10);
let collatz = generate_collatz_times(10);
let powers = generate_powers_of_2(10);
let catalan = generate_catalan(10);
let hex = generate_hexagonal(10);
let happy = generate_happy(10);

// Generate up to a maximum value
let fibs_up_to = generate_fibonacci_up_to(1000);

// Check membership
assert!(is_fibonacci(21));
assert!(is_lucas(29));
assert!(is_triangular(15));
assert!(is_power_of_2(64));
assert!(is_catalan(42));
assert!(is_hexagonal(45));
assert!(is_happy(19));

// Single collatz stopping time
let steps = collatz_stopping_time(27);  // 111
```

## Project Structure

```
rust-series/
├── Cargo.toml
├── Makefile
├── README.md
└── src/
    ├── lib.rs          # Re-exports all series
    ├── fibonacci.rs    # Fibonacci implementation
    ├── lucas.rs        # Lucas implementation
    ├── triangular.rs   # Triangular implementation
    ├── collatz.rs      # Collatz stopping times
    ├── powers.rs       # Powers of 2 implementation
    ├── catalan.rs      # Catalan numbers
    ├── hexagonal.rs    # Hexagonal numbers
    ├── happy.rs        # Happy numbers
    ├── main.rs         # CLI entry point
    └── progress.rs     # Progress bar
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```
