.PHONY: help build test run release clean batch benchmark parallel all-tests

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

run:
	cargo run --release -- --number 7

clean:
	cargo clean

batch:
	cargo run --release -- --batch-test --start 100000 --end 105000 -t 4

benchmark:
	cargo run --release -- --number 2147483647

parallel:
	cargo run --release -- --number 2147483647 -p -t 8

all-tests: test
	@echo "Running all tests including doc-tests..."
	cargo test --doc

help:
	@echo "Miller-Rabin Primality Tester"
	@echo ""
	@echo "Targets:"
	@echo "  build       - Build debug binary"
	@echo "  release     - Build release binary (optimized)"
	@echo "  test        - Run all tests"
	@echo "  run         - Run with --number 7"
	@echo "  clean       - Clean build artifacts"
	@echo "  batch       - Batch test range 100000-105000 with 4 threads"
	@echo "  benchmark   - Benchmark large number primality test"
