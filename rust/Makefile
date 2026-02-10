.PHONY: all build release run run-release test clean help

# Default target
all: build

N ?= 10000000

build:
	cargo build

release:
	cargo build --release

run: build
	./target/debug/primes -n 1000

run-release: release
	./target/release/primes -n 1000000 --quiet

run-release-parallel-progress: release
	@./target/release/primes -n $(N) -p -P > /dev/null

test:
	cargo test

test-verbose:
	cargo test -- --nocapture

clean:
	cargo clean

rebuild: clean build

setup:
	@echo "Checking Rust installation..."
	@rustc --version 2>/dev/null || (echo "Rust not found. Installing via rustup..." && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
	@echo "Updating Rust..."
	rustup update stable 2>/dev/null || true
	@echo "Rust setup complete."

help:
	@echo "Rust Prime Generator - Makefile Targets"
	@echo ""
	@echo "  make build                    - Build debug version"
	@echo "  make release                  - Build optimized release"
	@echo "  make run                      - Run debug (n=1000)"
	@echo "  make run-release              - Run release, count primes < 1M"
	@echo "  make run-release-parallel-progress - Run release parallel + progress (10M)"
	@echo "  make test                     - Run all tests"
	@echo "  make test-verbose             - Run tests with output"
	@echo "  make clean                    - Clean build artifacts"
	@echo "  make rebuild                  - Clean and rebuild"
	@echo "  make setup                    - Check/install Rust"
	@echo "  make help                     - Show this help"
