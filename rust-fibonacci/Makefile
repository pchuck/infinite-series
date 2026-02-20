.PHONY: all build release run run-release test clean help

all: build

N ?= 100

build:
	cargo build

release:
	cargo build --release

run: build
	cargo run -- -c $(N)

run-release: release
	./target/release/fibonacci_cli -c 100 --quiet

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
	@echo "Rust Fibonacci Generator - Makefile Targets"
	@echo ""
	@echo "  make build        - Build debug version"
	@echo "  make release      - Build optimized release"
	@echo "  make run          - Run debug (N=$(N))"
	@echo "  make run-release  - Run release, count first 100"
	@echo "  make test         - Run all tests"
	@echo "  make test-verbose - Run tests with output"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make rebuild      - Clean and rebuild"
	@echo "  make setup        - Check/install Rust"
	@echo "  make help         - Show this help"
	@echo ""
	@echo "Variables:"
	@echo "  N=number          - Number of Fibonacci numbers (default: 100)"
