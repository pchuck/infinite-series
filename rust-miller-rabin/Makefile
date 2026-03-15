.PHONY: help build test run release clean batch benchmark parallel all-tests file-test custom-bases large-benchmark composite-large batch-large m607 m1279 m2203 m11213 m44497 m86243 m216091 m13982683

help:
	@echo "Miller-Rabin Primality Tester"
	@echo ""
	@echo "Usage:"
	@echo "  Single number:     --number <N> [-p|--parallel]"
	@echo "  File input:        -f <file>"
	@echo "  Batch range:       --batch-test --start X --end Y"
	@echo "  Parallel mode:     Add -p to enable (auto-detects threads)"
	@echo "  Thread control:    Use -t N for specific thread count, 0=auto"
	@echo ""
	@echo "Targets:"
	@echo "  build                  - Build debug binary"
	@echo "  release                - Build optimized release binary"
	@echo "  test                   - Run unit tests"
	@echo "  clean                  - Clean build artifacts"
	@echo ""
	@echo "Basic Benchmarks (use THREADS=N for thread count):"
	@echo "  batch                  - Batch test range 100000-105000 with auto threads"
	@echo "  benchmark              - Benchmark large prime 2147483647 (sequential)"
	@echo "  parallel               - Parallel test 2147483647 (auto threads)"
	@echo ""
	@echo "Mersenne Prime Benchmarks (use EXTRA='-p' for parallel):"
	@echo "  m607     (~184 digits) ~20ms      M2^607-1"
	@echo "  m1279    (~386 digits) ~100ms    M2^1279-1"
	@echo "  m2203    (~664 digits) ~500ms    M2^2203-1"
	@echo ""
	@echo "Large-Scale Benchmarks (use EXTRA='-p' for parallel):"
	@echo "  m44497  (~13390 digits)         M2^44497-1"
	@echo "  m86243  (~25960 digits)        M2^86243-1"
	@echo ""
	@echo "Environment Variables:"
	@echo "  THREADS=N make benchmark     # Use N threads"
	@echo "  EXTRA='-p' make m2203       # Enable parallel mode"

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
	cargo run --release -- --batch-test --start 100000 --end 105000

benchmark:
	@echo "Benchmarking 2147483647 (sequential)..."
	cargo run --release -- --number 2147483647 $(EXTRA)

parallel:
	@echo "Parallel benchmark: 2147483647 with auto-detected threads..."
	cargo run --release -- --number 2147483647 -p

file-test:
	@echo "Create test file..."
	@echo -e "7\n11\n15\n104729" > /tmp/mr_test_numbers.txt
	@echo "Testing numbers from file:"
	cargo run --release -- -f /tmp/mr_test_numbers.txt $(EXTRA)

all-tests: test
	@echo "Running all tests including doc-tests..."
	cargo test --doc

custom-bases:
	@if [ -z "$(NUMBER)" ] || [ -z "$(BASES)" ]; then \
		echo "Usage: make custom-bases NUMBER=<n> BASES='a,b,c'"; exit 1; fi
	@echo "Testing $(NUMBER) with bases: $(BASES)"
	./target/release/miller-rabin-tester --number $(NUMBER) --bases $(BASES)

custom-bases-parallel:
	@if [ -z "$(NUMBER)" ] || [ -z "$(BASES)" ]; then \
		echo "Usage: make custom-bases-parallel NUMBER=<n> BASES='a,b,c'"; exit 1; fi
	@echo "Testing $(NUMBER) with bases: $(BASES) in parallel mode"
	./target/release/miller-rabin-tester --number $(NUMBER) --bases $(BASES) -p

m607:
	@if [ ! -f large_prime_m607.txt ]; then \
		echo "Generating M607 (~184 digits)..."; python3 -c "open('large_prime_m607.txt','w').write(str((1<<607)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m607.txt)

m1279:
	@if [ ! -f large_prime_m1279.txt ]; then \
		echo "Generating M1279 (~386 digits)..."; python3 -c "open('large_prime_m1279.txt','w').write(str((1<<1279)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m1279.txt)

m2203:
	@if [ ! -f large_prime_m2203.txt ]; then \
		echo "Generating M2203 (~664 digits)..."; python3 -c "open('large_prime_m2203.txt','w').write(str((1<<2203)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m2203.txt)  --verbose --parallel --show-progress

m11213:
	@if [ ! -f large_prime_m11213.txt ]; then \
		echo "Generating M11213 (~3375 digits)..."; python3 -c "open('large_prime_m11213.txt','w').write(str((1<<11213)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m11213.txt) --verbose --parallel --show-progress

m44497:
	@if [ ! -f large_prime_m44497.txt ]; then \
		echo "Generating M44497 (~13390 digits)..."; python3 -c "import sys; sys.set_int_max_str_digits(15000); open('large_prime_m44497.txt','w').write(str((1<<44497)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m44497.txt) --verbose --parallel --show-progress

m86243:
	@if [ ! -f large_prime_m86243.txt ]; then \
		echo "Generating M86243 (~25960 digits)..."; python3 -c "import sys; sys.set_int_max_str_digits(30000); open('large_prime_m86243.txt','w').write(str((1<<86243)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m86243.txt) --verbose --parallel --show-progress

m216091:
	@if [ ! -f large_prime_m216091.txt ]; then \
		echo "Generating M216091 (~65K digits)..."; python3 -c "import sys; sys.set_int_max_str_digits(70000); open('large_prime_m216091.txt','w').write(str((1<<216091)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m216091.txt) --verbose --parallel --show-progress

m13982683:
	@echo ""
	@echo "============================================"
	@echo "WARNING: M13982683 has ~4.2 million digits!"
	@echo "Expected runtime: 10+ hours with Miller-Rabin"
	@echo "This requires significant memory and CPU time"
	@echo "============================================"
	@sleep 3
	@if [ ! -f large_prime_m13982683.txt ]; then \
		echo "Generating M13982683 (~4.2M digits)..."; python3 -c "import sys; sys.set_int_max_str_digits(5000000); open('large_prime_m13982683.txt','w').write(str((1<<13982683)-1))"; fi
	cargo run --release -- --number $$(cat large_prime_m13982683.txt)

composite-large:
	@if [ ! -f composite_large.txt ]; then \
		echo "Generating large composite (M2203-2)..."; python3 -c "open('composite_large.txt','w').write(str((1<<2203)-2))"; fi
	cargo run --release -- --number $$(cat composite_large.txt)

batch-large:
	@if [ ! -f large_batch_input.txt ]; then \
		echo "Creating batch input file..."; python3 -c \
			"import sys; f=open('large_batch_input.txt','w'); f.write(str((1<<607)-1)+'\n'); f.write(str((1<<1279)-2)+'\n'); f.write(str((1<<1279)-1)+'\n'); f.write(str((1<<2203)-1)); f.close()"; fi
	cargo run --release -- -f large_batch_input.txt

large-benchmark:
	@echo "============================================"
	@echo "Miller-Rabin: Large Number Benchmark Suite"
	@echo "============================================"
	@make release
	@echo ""
	@echo "=== Quick Benchmarks ===" && echo ""
	@make m607 $(EXTRA) && @echo ""
	@make m1279 $(EXTRA) && @echo ""
	@make m2203 $(EXTRA) && @echo ""
	@make composite-large
	@echo ""
	@echo "============================================"
	@echo "Quick benchmark complete!"
	@echo "(Run 'make medium-benchmark' for M11213, M44497)"
	@echo "============================================"
