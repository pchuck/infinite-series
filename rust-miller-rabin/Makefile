.PHONY: help build test run release clean batch benchmark parallel all-tests file-test

help:
	@echo "Miller-Rabin Primality Tester"
	@echo ""
	@echo "Targets:"
	@echo "  build       - Build debug binary"
	@echo "  release     - Build optimized release binary"
	@echo "  test        - Run unit tests"
	@echo "  run         - Test number 7 (sequential)"
	@echo "  clean       - Clean build artifacts"
	@echo "  batch       - Batch test range 100000-105000 with 4 threads"
	@echo "  benchmark   - Benchmark large prime 2147483647"
	@echo "  parallel    - Parallel test 2147483647 with 8 threads"
	@echo "  file-test   - Test numbers from a file"

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

file-test:
	@echo "Create test file..."
	@echo -e "7\n11\n15\n104729" > /tmp/mr_test_numbers.txt
	@echo "Testing numbers from file:"
	cargo run --release -- -f /tmp/mr_test_numbers.txt

all-tests: test
	@echo "Running all tests including doc-tests..."
	cargo test --doc
