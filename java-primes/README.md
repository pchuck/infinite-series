# Java Prime Generator

High-performance prime number generator using Sieve of Eratosthenes in Java.

## Features

- Classic Sieve of Eratosthenes (n < 10M)
- Segmented Sieve (n >= 10M)
- Parallel Segmented Sieve (n >= 500M with `--parallel`)
- Progress bar support

## Build

```bash
make build
```

## Run

```bash
make run N=1000          # Generate primes < 1000
make run-quiet N=1000000 # Count only
make run-progress         # With progress bar
```

## Options

- `-p, --progress`  Show progress indicator
- `--parallel`      Use parallel processing (for large n)
- `-q, --quiet`     Output count only
- `-h, --help`      Show help

## Test

```bash
make test
```

## Direct Usage

```bash
java -cp out net.ultrametrics.primes.App 1000000 --quiet
```
