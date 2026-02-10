.PHONY: help build test run release clean

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

help:
	@echo "Miller-Rabin Primality Tester"
	@echo ""
	@echo "Targets:"
	@echo "  build      - Build debug binary"
	@echo "  release    - Build release binary (optimized)"
	@echo "  test       - Run all tests"
	@echo "  run        - Run with --number 7"
	@echo "  clean      - Clean build artifacts"
