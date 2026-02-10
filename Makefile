.PHONY: help benchmark build test clean

all: help
COUNT=50000000
DEST=/dev/null

benchmark:
	@echo "Performance Comparison: $(COUNT)"
	@echo "=========================================="
	@echo ""
	@echo "Rust:"
	@cd rust && $(MAKE) run-release-parallel-progress N=$(COUNT)
	@echo ""
	@echo "Go:"
	@cd golang && $(MAKE) run-progress-parallel N=$(COUNT)
	@echo ""
	@echo "Python:"
	@cd python && $(MAKE) run-progress-parallel N=$(COUNT)

build:
	@echo "Building all implementations..."
	@cd rust && $(MAKE) release
	@cd golang && $(MAKE) build
	@echo "All implementations built."

test:
	@echo "Running all tests..."
	@cd rust && $(MAKE) test
	@cd golang && $(MAKE) test
	@cd python && $(MAKE) test
	@echo "All tests completed."

clean:
	@echo "Cleaning all build artifacts..."
	@cd rust && $(MAKE) clean
	@cd golang && $(MAKE) clean
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
	@echo "  cd rust && make help"
	@echo "  cd golang && make help"
	@echo "  cd python && make help"
