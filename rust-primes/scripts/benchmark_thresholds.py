#!/usr/bin/env python3
"""
 Wrapper script for benchmarking threshold values.
 
 This script runs the Rust threshold benchmark and provides a summary
 of recommended cutoff points for algorithm selection.
"""

import subprocess
import sys
from pathlib import Path


def run_benchmark():
    """Run the Rust threshold benchmark."""
    scripts_dir = Path(__file__).parent.resolve()
    project_root = scripts_dir.parent
    
    # Build release version if needed
    print("Building release version...")
    result = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        print(f"Build failed:\n{result.stderr}")
        return None
    
    # Run benchmark
    print("\nRunning threshold benchmark...")
    result = subprocess.run(
        ["cargo", "run", "--release", "--bin", "bench_thresholds"],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    
    return result.stdout


def main():
    if len(sys.argv) > 1 and sys.argv[1] in ["--help", "-h"]:
        print("Threshold Benchmark Wrapper")
        print("\nUsage: python benchmark_thresholds.py [OPTIONS]")
        print("\nOptions:")
        print("  -h, --help     Show this help message")
        print("\nDescription:")
        print("  This script benchmarks the three prime generation algorithms")
        print("  across a range of input sizes to find optimal threshold values.")
        print("\nThe benchmark runs on your hardware to determine optimal cutoff points")
        print("for algorithm selection based on input size n:")
        print("  - Classic Sieve: for small inputs (n < threshold1)")
        print("  - Segmented Sieve: for medium inputs (threshold1 <= n < threshold2)")  
        print("  - Parallel Segmented Sieve: for large inputs (n >= threshold2)")
        return
    
    output = run_benchmark()
    
    if output is None:
        sys.exit(1)
    
    # Parse and display just the key recommendations
    lines = output.split('\n')
    
    # Find and print the analysis section
    in_analysis = False
    for line in lines:
        if '=== Analysis ===' in line:
            in_analysis = True
            continue
        
        if in_analysis:
            if 'Recommendations' in line or 'Classic' in line or 'Segmented' in line or 'Parallel' in line:
                print(line)
    
    print("\n" + "="*60)
    print("To run the full benchmark manually:")
    print("  cd rust-primes && cargo run --release --bin bench_thresholds")
    print("="*60)


if __name__ == "__main__":
    main()
