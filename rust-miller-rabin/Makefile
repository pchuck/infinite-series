.PHONY: help build test run release clean batch benchmark parallel all-tests file-test custom-bases custom-bases-parallel

help:
	@echo "Miller-Rabin Primality Tester"
	@echo ""
	@echo "Targets:"
	@echo "  build            - Build debug binary"
	@echo "  release          - Build optimized release binary"
	@echo "  test             - Run unit tests"
	@echo "  run              - Test number 7 (sequential)"
	@echo "  clean            - Clean build artifacts"
	@echo "  batch            - Batch test range 100000-105000 with 4 threads"
	@echo "  benchmark        - Benchmark large prime 2147483647"
	@echo "  parallel          - Parallel test 2147483647 with 8 threads"
	@echo "  file-test         - Test numbers from a file"
	@echo "  custom-bases            - Test number with custom bases (set NUMBER and BASES)"
	@echo "  custom-bases-parallel   - Parallel test with custom bases"
	@echo ""
	@echo "Custom Base Examples:"
	@echo "  make custom-bases NUMBER=104729 BASES='2,3,5'"
	@echo "  make custom-bases NUMBER=2147483647 BASES='2,3,5,7,11,13'"

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

custom-bases:
	@if [ -z "$(NUMBER)" ] || [ -z "$(BASES)" ]; then \
		echo "Usage: make custom-bases NUMBER=<n> BASES='a,b,c'"; \
		echo "Example: make custom-bases NUMBER=104729 BASES='2,3,5'"; \
		echo "Parallel:  make custom-bases-parallel NUMBER=2147483647 BASES='2,3,5,7,11,13' -p"; \
		exit 1; fi
	@echo "Testing $(NUMBER) with bases: $(BASES)"
	./target/release/miller-rabin-tester --number $(NUMBER) --bases $(BASES)

custom-bases-parallel:
	@if [ -z "$(NUMBER)" ] || [ -z "$(BASES)" ]; then \
		echo "Usage: make custom-bases-parallel NUMBER=<n> BASES='a,b,c'"; \
		exit 1; fi
	@echo "Testing $(NUMBER) with bases: $(BASES) in parallel mode"
	./target/release/miller-rabin-tester --number $(NUMBER) --bases $(BASES) -p
