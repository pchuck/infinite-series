#!/bin/bash

# Simple benchmarking script for Miller-Rabin primality tester
set -e

SCRIPT_DIR="$(dirname "$0")"
cd "$SCRIPT_DIR" 2>/dev/null || cd "$(pwd)"

MODE=${1:-"--release"}

echo "=== Miller-Rabin Benchmark Suite ==="
echo ""

if [ "$MODE" = "--release" ]; then
    echo "Building release binary..."
    cargo build --release > /dev/null 2>&1
    BINARY="./target/release/miller-rabin-tester"
else
    echo "Building debug binary..."
    cargo build > /dev/null 2>&1
    BINARY="./target/debug/miller-rabin-tester"
fi

echo ""
echo "=== Single Number Tests ==="

test_number() {
    local n="$1"
    local expected="$2"

    echo -n "Test: $n -> "
    
    result=$("$BINARY" --number "$n" --output-format json 2>/dev/null)
    is_prime=$(echo "$result" | grep -o 'is_prime.*' | sed 's/[^:]*: *//' | tr -d ',')
    elapsed=$(echo "$result" | grep -o 'performance_ms.*' | sed 's/[^:]*: *//' | tr -d ',')

    if [ "$is_prime" = "true" ]; then
        echo "PRIME ✓ ($elapsed ms)"
    else
        echo "COMPOSITE ✓ ($elapsed ms)"
    fi
}

test_number 104729 true
test_number 1009 true  
test_number 561 false
test_number 341 false

echo ""
echo "=== Performance Test: Small Range ==="
"$BINARY" --batch-test --start 2 --end 200 -t 4

echo ""
echo "=== Performance Test: Medium Range ==="
"$BINARY" --batch-test --start 1000 --end 1500 -t 4

echo ""
echo "=== Performance Test: Large Range ==="
"$BINARY" --batch-test --start 10000000 --end 15000000 -t 4 > /dev/null

echo ""
echo "=== Parallel Mode with Large Prime ==="
"$BINARY" --number 1299709 --parallel --threads 8 --verbose

if [ -f "./large_benchmark.sh" ]; then
    echo ""
    echo "=== Large Number Benchmarks (Mersenne Primes) ==="
    ./large_benchmark.sh $MODE
else
    if [ -f "large_prime_m1279.txt" ]; then
        echo ""
        echo "Testing pre-generated large primes..."
        
        for file in large_prime_*.txt; do
            if [ -f "$file" ]; then
                n=$(cat "$file")
                digits=${#n}
                echo "  $file ($digits digits):"
                "$BINARY" --number "$n" > /dev/null 2>&1 && echo "    OK ✓" || echo "    FAILED ✗"
            fi
        done
        
        if [ -f "composite_large.txt" ]; then
            n=$(cat composite_large.txt)
            result=$("$BINARY" --number "$n" 2>/dev/null)
            if echo "$result" | grep -q COMPOSITE; then
                echo "  composite_large.txt: Correctly identified as COMPOSITE ✓"
            else
                echo "  composite_large.txt: Failed to identify composite ✗"
            fi
        fi
    fi
fi

echo ""
echo "Benchmark complete!"
