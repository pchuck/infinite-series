# Miller-Rabin Primality Tester

A standalone Rust application that implements the Miller-Rabin probabilistic primality test for very large integers.

## Features

- Miller-Rabin primality testing with deterministic bases for 64-bit numbers
- Uses trial division by small primes (2, 3, 5) for quick elimination of obvious composites
- Supports arbitrary-precision integers via `num-bigint`
- Efficient modular exponentiation implementation

## Usage

```bash
# Test a specific number
cargo run --release -- --number <N>

# Batch test a range of numbers
cargo run --release -- --batch-test --start <START> --end <END>
```

## Examples

```bash
# Test if 104729 is prime
cargo run --release -- --number 104729

# Test all numbers from 100000 to 105000 for primality
cargo run --release -- --batch-test --start 100000 --end 105000
```

## Building

```bash
cargo build --release
./target/release/miller-rabin-tester --number <N>
```