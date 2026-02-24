.PHONY: all build release run run-release test clean help bench bench-quick bench-report

# Default target
all: build

N ?= 10000000

build:
	cargo build

release:
	cargo build --release

run_direct: 
	cargo run --bin primes_cli

run: build
	./target/debug/primes_cli

run-release: release
	./target/release/primes_cli -n 1000000 --quiet

run-release-parallel: release
	@./target/release/primes_cli -n $(N) -p -P > /dev/null

test:
	cargo test

test-verbose:
	cargo test -- --nocapture

test-integration:
	cargo test --test cli_integration

clean:
	cargo clean

rebuild: clean build

setup:
	@echo "Checking Rust installation..."
	@rustc --version 2>/dev/null || (echo "Rust not found. Installing via rustup..." && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
	@echo "Updating Rust..."
	rustup update stable 2>/dev/null || true
	@echo "Rust setup complete."

bench:
	cargo bench

bench-quick:
	@echo "Running quick benchmarks (n <= 1M)..."
	cargo bench --bench prime_benchmarks -- --test n_100000 n_1000000

bench-report:
	@echo "Opening benchmark report..."
	@if command -v xdg-open >/dev/null 2>&1; then \
		xdg-open target/criterion/report/index.html; \
	elif command -v open >/dev/null 2>&1; then \
		open target/criterion/report/index.html; \
	else \
		echo "Report available at: target/criterion/report/index.html"; \
	fi

bench-save-baseline:
	cargo bench --bench prime_benchmarks -- --save-baseline reference

bench-check:
	cargo bench --bench prime_benchmarks -- --baseline reference --noise-threshold 0.09

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
	@echo "  make test-integration         - Run integration tests only"
	@echo "  make bench                    - Run performance benchmarks"
	@echo "  make bench-quick              - Run quick benchmarks (n <= 1M)"
	@echo "  make bench-report             - Open benchmark HTML report"
	@echo "  make bench-save-baseline      - Save benchmark baseline"
	@echo "  make bench-check              - Run benchmark against baseline"
	@echo "  make clean                    - Clean build artifacts"
	@echo "  make rebuild                  - Clean and rebuild"
	@echo "  make setup                    - Check/install Rust"
	@echo "  make help                     - Show this help"
