.PHONY: all build release run run-release test clean help

# Default target
all: build

N ?= 10000000

build:
	cargo build

release:
	cargo build --release

run_cli_direct: 
	cargo run --bin primes_cli

run_gui_direct: 
	cargo run --bin primes_gui

run_cli: build
	./target/debug/primes_cli

run_gui: build
	./target/debug/primes_gui -n 1000


run-release_cli: release
	./target/release/primes_cli -n 1000000 --quiet

run-release-parallel_cli: release
	@./target/release/primes_cli -n $(N) -p -P > /dev/null

run-release_gui: release
	./target/release/primes_gui

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
	@echo "  make run_gui                  - Run debug gui"
	@echo "  make run-release_gui          - Run release gui"
	@echo "  make run_cli                  - Run debug (n=1000)"
	@echo "  make run-release_cli          - Run release, count primes < 1M"
	@echo "  make run-release-parallel_cli - Run parallel + progress(10M)"
	@echo "  make test                     - Run all tests"
	@echo "  make test-verbose             - Run tests with output"
	@echo "  make clean                    - Clean build artifacts"
	@echo "  make rebuild                  - Clean and rebuild"
	@echo "  make setup                    - Check/install Rust"
	@echo "  make help                     - Show this help"
