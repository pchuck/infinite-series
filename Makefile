.PHONY: help benchmark build test clean

all: help
COUNT=50000000
DEST=/dev/null

benchmark:
	@echo "Performance Comparison: $(COUNT)"
	@echo "=========================================="
	@echo ""
	@echo "Rust:"
	@cd rust-primes && $(MAKE) run-release-parallel_cli N=$(COUNT)
	@echo ""
	@echo "Go:"
	@cd golang-primes && $(MAKE) run-progress-parallel N=$(COUNT)
	@echo ""
	@echo "Python:"
	@cd python-primes && $(MAKE) run-progress-parallel N=$(COUNT)

build:
	@echo "Building all implementations..."
	@cd rust-primes && $(MAKE) release
	@cd golang-primes && $(MAKE) build
	@echo "All implementations built."

test:
	@echo "Running all tests..."
	@cd rust-primes && $(MAKE) test
	@cd golang-primes && $(MAKE) test
	@cd python-primes && $(MAKE) test
	@echo "All tests completed."

clean:
	@echo "Cleaning all build artifacts..."
	@cd rust-primes && $(MAKE) clean
	@cd golang-primes && $(MAKE) clean
	@echo "Cleaned."

help:
	@echo "Prime Number Generator - Multi-Language Project"
	@echo ""
	@echo "Performance Comparisons (default: $(COUNT)M):"
	@echo "  make benchmark        - Compare all 3 implementations"
	@echo ""
	@echo "Project Commands:"
	@echo "  make build            - Build all implementations"
	@echo "  make test             - Run all tests"
	@echo "  make clean            - Remove build artifacts"
	@echo ""
	@echo "Language-specific targets:"
	@echo "  cd rust-primes && make help"
	@echo "  cd golang-primes && make help"
	@echo "  cd python-primes && make help"
