# AGENTS.md - Prime Number Generator Project

This project implements high-performance prime number generators in Python, Go, and Rust.

## Build/Lint/Test Commands

### Rust
- **Build debug**: `cd rust-primes && cargo build`
- **Build release**: `cd rust-primes && cargo build --release`
- **Run tests**: `cd rust-primes && cargo test`
- **Single test**: `cd rust-primes && cargo test test_sieve_small`
- **Lint**: `cd rust-primes && cargo clippy`
- **Format**: `cd rust-primes && cargo fmt`
- **Run application**: `cd rust-primes && ./target/release/primes -n 1000000 --quiet`

### Go
- **Build**: `cd golang-primes && go build -o primes ./cmd/primes`
- **Run tests**: `cd golang-primes && go test ./...`
- **Single test**: `cd golang-primes && go test -run TestGeneratePrimes`
- **Lint**: `cd golang-primes && go vet ./...`
- **Format**: `cd golang-primes && go fmt ./...`
- **Run application**: `cd golang-primes && ./primes --quiet 1000000`

### Python
- **Run tests**: `cd python-primes && python -m pytest test_generators.py -v`
- **Single test**: `cd python-primes && python -m pytest test_generators.py::TestGeneratePrimes::test_small_input -v`
- **Type check**: `cd python-primes && mypy prime_generator.py test_generators.py`
- **Lint**: `cd python-primes && ruff check .`
- **Run application**: `cd python-primes && python prime_generator.py 1000000 --quiet`

### Makefile (Rust)
- **All targets**: `cd rust-primes && make help`
- **Quick benchmark**: `cd rust-primes && make run-release-quiet`

## Code Style Guidelines

### General Principles
- Write clear, self-documenting code with descriptive names
- Keep functions focused on a single task
- Use constants for magic numbers (e.g., `DEFAULT_SEGMENT_SIZE = 1_000_000`)
- Progress/stat messages to stderr, data output to stdout

### Imports and Dependencies

**Rust**:
- Group imports: std:: imports first, then external crates
- Use `use` statements at module level, not inline
- Example:
  ```rust
  use std::sync::Arc;
  use std::thread;
  ```

**Go**:
- Standard library imports first, then third-party
- Group by package, alphabetize within groups
- Example:
  ```go
  import (
      "math"
      "runtime"
      "sync"
  )
  ```

**Python**:
- Standard library first, then third-party (e.g., `tqdm`)
- Use `from typing import ...` for type hints
- Example:
  ```python
  from typing import List, Optional, Callable
  ```

### Naming Conventions

| Element | Python | Go | Rust |
|---------|--------|----|----|
| Functions | `snake_case` | `CamelCase` | `snake_case` |
| Constants | `UPPER_SNAKE_CASE` | `PascalCase` | `UPPER_SNAKE_CASE` |
| Variables | `snake_case` | `snake_case` | `snake_case` |
| Types/Classes | `PascalCase` | `PascalCase` | `PascalCase` |
| Private methods | `_leading_underscore` | `lowercase` | `leading_underscore` |

### Type Annotations

**Python**: Required for all public functions
```python
def sieve_of_eratosthenes(n: int, show_progress: bool = False) -> List[int]:
```

**Go**: Use primitive types, explicit about `int` vs explicit sizes
```go
func SieveOfEratosthenes(n int) []int {
```

**Rust**: Full type annotations required, use `usize` for indices
```rust
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
```

### Error Handling

**Rust**: Use `Result` and `?` operator, no unwrap in production code
```rust
pub fn generate_primes(n: usize) -> Result<Vec<usize>, String> {
    if n < 2 {
        return Err("n must be >= 2");
    }
    Ok(primes)
}
```

**Go**: Return errors for user input, use nil checks
```go
func GeneratePrimes(n int) ([]int, error) {
    if n < 2 {
        return nil, fmt.Errorf("n must be >= 2")
    }
    return primes, nil
}
```

**Python**: Raise `ValueError` with descriptive messages
```python
def generate_primes(n: int) -> List[int]:
    if n < 2:
        raise ValueError("n must be >= 2")
    return primes
```

### Algorithm Selection
- Classic Sieve: n < 1M
- Segmented Sieve: 1M <= n < 100M
- Parallel Segmented Sieve: n >= 100M

### Progress Callbacks
- Callback receives iteration count, not percentage
- Check for `None` before calling
- Use Arc/dyn Fn for thread-safe callbacks in Rust

### Testing
- Tests verify correctness across implementations
- Compare segmented/parallel results against classic sieve
- Test edge cases: n=0, n=1, n=2, small primes

### Progress Bar Implementation
- Rust: Custom ANSI progress bar, no external dependencies
- Go: Custom implementation, stdout/stderr separation
- Python: Optional tqdm with fallback to simple output
