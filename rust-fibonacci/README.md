# Rust Fibonacci Number Generator

High-performance Fibonacci number generator with CLI interface.

## Quick Start

### Make
```bash
make help
make release            # Build optimized
make run                # Run (n=100)
make run-release        # Count first 100 fibs
make test               # Test
```

### Cargo

```bash
# Build
cargo build

# Run CLI
cargo run -- -n 100

# Run tests
cargo test
```

## CLI Usage

```bash
# Generate first 10 Fibonacci numbers
cargo run -- -c 10

# Generate first 100 (quiet mode - count only)
cargo run -- -c 100 --quiet

# With progress bar
cargo run -- -c 1000 --progress
```

### CLI Options

| Option | Description |
|--------|-------------|
| `-c, --count` | Number of Fibonacci numbers to generate |
| `-q, --quiet` | Only print count (no number list) |
| `-P, --progress` | Show progress bar |

## Algorithm

Uses iterative accumulation - O(n) time complexity, O(n) space.

## Project Structure

```
rust-fibonacci/
├── Cargo.toml
├── Makefile
├── README.md
└── src/
    ├── lib.rs          # Core implementation
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
