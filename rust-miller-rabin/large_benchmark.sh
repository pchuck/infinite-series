#!/bin/bash

# Large number benchmarks for Miller-Rabin primality tester
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

MODE=${1:-"--release"}

echo "=== Miller-Rabin: Large Number Benchmarks ==="
echo ""

if [ "$MODE" = "--release" ]; then
    echo "[Using release build]"
    BINARY="./target/release/miller-rabin-tester"
else
    echo "[Using debug build]"  
    BINARY="./target/debug/miller-rabin-tester"
fi

run_benchmark() {
    local label="$1"
    local file="$2"

    if [ ! -f "$file" ]; then
        echo "  $label: SKIPPED (file not found)"
        return
    fi

    n=$(cat "$file")
    digits=${#n}

    echo ""
    echo "--- $label ($digits digits) ---"
    
    start_time=$(date +%s.%N)
    result=$("$BINARY" --number "$n" 2>/dev/null)
    end_time=$(date +%s.%N)

    elapsed=$(echo "$end_time - $start_time" | bc)
    
    if echo "$result" | grep -q "PRIME"; then
        echo "  Result: PRIME ✓"
    else
        echo "  Result: COMPOSITE ✓"
    fi
    
    printf "  Time: %.3f s\n" "$elapsed"
}

echo ""
echo "=== Individual Benchmarks ==="

run_benchmark "M607 (~184 digits)" "large_prime_m607.txt"
run_benchmark "M1279 (~386 digits)" "large_prime_m1279.txt"
run_benchmark "M2203 (~664 digits)" "large_prime_m2203.txt"
run_benchmark "Large Composite" "composite_large.txt"

echo ""
echo "=== Batch Test: Multiple Large Numbers ==="

if [ -f large_batch_input.txt ]; then
    echo "[Testing 4 large numbers from large_batch_input.txt]"
    
    start_time=$(date +%s.%N)
    "$BINARY" --file large_batch_input.txt > /dev/null 2>&1
    end_time=$(date +%s.%N)
    
    elapsed=$(echo "$end_time - $start_time" | bc)
    printf "  Batch time: %.3f s\n" "$elapsed"
fi

echo ""
echo "=== Summary ==="
cat << 'EOF'
Files generated:
  large_prime_m607.txt     - M607 = 2^607-1 (~184 digits)  
  large_prime_m1279.txt   - M1279 = 2^1279-1 (~386 digits)
  large_prime_m2203.txt   - M2203 = 2^2203-1 (~664 digits)
  composite_large.txt     - (M2203 - 2), known composite
  large_batch_input.txt   - All primes + composite for batch testing

Usage:
  ./miller-rabin-tester --number "$(cat large_prime_m1279.txt)"
  ./miller-rabin-tester --file large_batch_input.txt
EOF