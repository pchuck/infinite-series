# Threshold Benchmark Scripts

This directory contains tools to determine optimal algorithm threshold values.

## Files

- `benchmark_thresholds.rs` - Rust binary for benchmarking
- `benchmark_thresholds.py` - Python wrapper script

## Usage

### Direct Binary

```bash
cd rust-primes
cargo run --release --bin bench_thresholds
```

### Using Makefile

```bash
make benchmark-thresholds
```

### Using Python Wrapper

```bash
python scripts/benchmark_thresholds.py
```

## What It Does

The benchmark tests all three prime generation algorithms across a range of input sizes:

- **Classic Sieve**: For small inputs (n < 1M)
- **Segmented Sieve**: For medium inputs (1M ≤ n < 10M)  
- **Parallel Segmented Sieve**: For large inputs (n ≥ 5M)

It outputs timing data for each algorithm and determines optimal crossover points.

## Results

Based on benchmarks on AMD Ryzen 9 7900X:

- Classic → Segmented: n ≈ 1,000,000
- Segmented → Parallel: n ≈ 2,000,000

**Note**: Thresholds vary by hardware. Always run this benchmark on your target machine.

## Optimized Values

Current hardcoded thresholds in `src/lib.rs`:

```rust
pub const DEFAULT_SEGMENT_SIZE: usize = 1_000_000;
pub const PARALLEL_THRESHOLD: usize = 5_000_000;
```

These values were determined empirically using this benchmark script.

**Conservative choice**: Benchmark shows crossover at ~2M, but 5M provides extra margin
for thread startup overhead and cache effects on different hardware.
