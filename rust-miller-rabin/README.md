# Miller-Rabin Primality Tester

A standalone Rust application that implements the Miller-Rabin probabilistic primality test for very large integers, with support for both sequential and parallel processing.

## Features

- **Miller-Rabin** primality testing with deterministic bases for 64-bit numbers
- Uses trial division by small primes (2, 3, 5) for quick elimination of obvious composites
- Supports arbitrary-precision integers via `num-bigint`
- Efficient modular exponentiation implementation
- Parallel base testing using std::thread for large numbers

## Usage

```bash
# Test a specific number (sequential)
cargo run --release -- --number <N>

# Test with parallel base checking (multi-threaded Miller-Rabin)
cargo run --release -- --number <N> -p -t 8

# Batch test a range of numbers in parallel
cargo run --release -- --batch-test --start <START> --end <END>

# Test multiple numbers from a file (one per line)
cargo run --release -- --file <path>
```

## Examples

```bash
# Test if 104729 is prime (sequential)
cargo run --release -- --number 104729

# Test large prime with 8-thread parallel base checking
cargo run --release -- --number 2147483647 -p -t 8

# Batch test range 100000-105000 using 4 threads
cargo run --release -- --batch-test --start 100000 --end 105000 -t 4

# Test numbers from file (one per line)
echo -e "7\n11\n15\n104729" > /tmp/numbers.txt
cargo run --release -- --file /tmp/numbers.txt
```

## File Format for `--file`

Each line should contain one integer:
```text
7
11
15
104729
2147483647
```

Output shows each number's primality status and a summary.

## Parallelism

The implementation provides parallelism at two levels:

1. **Parallel Base Testing**: Miller-Rabin bases are distributed across threads when `-p` flag is used with `--number`. Each thread tests a subset of the deterministic bases independently.

2. **Batch Processing**: When testing number ranges (`--batch-test`), the range is divided into chunks and processed concurrently by multiple threads.

## Building

```bash
cargo build --release
./target/release/miller-rabin-tester --number <N>
```

## Makefile Targets

| Target | Description |
|--------|-------------|
| `make build` | Build debug binary |
| `make release` | Build optimized release binary |
| `make test` | Run unit tests |
| `make run` | Test number 7 (sequential) |
| `make batch` | Batch test range 100k-105m with 4 threads |
| `make benchmark` | Test large prime 2147483647 |
| `make parallel` | Parallel test 2147483647 with 8 threads |
| `make file-test` | Test numbers from a file (creates /tmp/mr_test_numbers.txt) |
| `make clean` | Remove build artifacts |