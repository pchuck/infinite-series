.PHONY: all build release run clean help test test-release lint fmt serve web-serve web-build

all: build

# Native GUI builds (default)
build: 
	cargo build

release:
	cargo build --release

run: build
	cargo run

run-release: release
	./target/release/visualizer

# Web/WASM builds
web-build:
	trunk build --release

web-serve:
	trunk serve --address 0.0.0.0 --port 8080

# Test native
test:
	cargo test

test-release:
	cargo test --release

# Lint and format
lint:
	cargo clippy

fmt:
	cargo fmt

# Clean builds
clean:
	cargo clean
	trunk clean 2>/dev/null || true

rebuild: clean build

setup:
	@echo "Checking Rust installation..."
	@rustc --version 2>/dev/null || (echo "Rust not found. Installing via rustup..." && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
	@echo "Updating Rust..."
	rustup update stable 2>/dev/null || true
	@echo "Adding wasm32 target..."
	rustup target add wasm32-unknown-unknown 2>/dev/null || true
	@echo "Installing trunk..."
	cargo install trunk 2>/dev/null || true
	@echo "Rust setup complete."

help:
	@echo "Number Sequence Visualizer - Makefile Targets"
	@echo ""
	@echo "  Native builds:"
	@echo "    make build         - Build native debug version"
	@echo "    make release       - Build native optimized release"
	@echo "    make run           - Run native debug version"
	@echo "    make run-release   - Run native release version"
	@echo ""
	@echo "  Web/WASM builds:"
	@echo "    make web-build     - Build WASM for web (requires trunk)"
	@echo "    make web-serve     - Serve web app with hot reload"
	@echo ""
	@echo "  Other:"
	@echo "    make test          - Run unit tests"
	@echo "    make test-release  - Run unit tests (release)"
	@echo "    make lint          - Run clippy lints"
	@echo "    make fmt           - Format code"
	@echo "    make clean         - Clean build artifacts"
	@echo "    make rebuild       - Clean and rebuild"
	@echo "    make setup         - Install Rust/wasm32/trunk"
	@echo "    make help          - Show this help"
