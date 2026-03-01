.PHONY: all build release run clean help test test-release lint fmt

all: build

build:
	cargo build

release:
	cargo build --release

run: build
	cargo run

run-release: release
	./target/release/visualizer

test:
	cargo test

test-release:
	cargo test --release

lint:
	cargo clippy

fmt:
	cargo fmt

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
	@echo "Number Sequence Visualizer - Makefile Targets"
	@echo ""
	@echo "  make build         - Build debug version"
	@echo "  make release       - Build optimized release"
	@echo "  make run           - Run debug version"
	@echo "  make run-release   - Run release version"
	@echo "  make test          - Run unit tests"
	@echo "  make test-release  - Run unit tests (release)"
	@echo "  make lint          - Run clippy lints"
	@echo "  make fmt           - Format code"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make rebuild       - Clean and rebuild"
	@echo "  make setup         - Check/install Rust"
	@echo "  make help          - Show this help"
