.PHONY: all build test run clean help

N ?= 10000000

all: build

build:
	mkdir -p out
	javac -d out src/main/java/net/ultrametrics/primes/*.java

test: build
	@echo "Running basic tests..."
	@java -cp out net.ultrametrics.primes.App 10 | grep -q "2, 3, 5, 7" && echo "PASS: Basic test"
	@java -cp out net.ultrametrics.primes.App 30 -q | grep -q "10" && echo "PASS: Count test"
	@java -cp out net.ultrametrics.primes.App 0 | grep -q "No primes" && echo "PASS: Edge case n=0"
	@java -cp out net.ultrametrics.primes.App 1 | grep -q "No primes" && echo "PASS: Edge case n=1"
	@java -cp out net.ultrametrics.primes.App 2 | grep -q "No primes" && echo "PASS: Edge case n=2"
	@echo "All tests passed"

run: build
	java -cp out net.ultrametrics.primes.App $(N)

run-quiet: build
	java -cp out net.ultrametrics.primes.App $(N) -q

run-progress: build
	java -cp out net.ultrametrics.primes.App $(N) -p

clean:
	rm -rf out

help:
	@echo "Java Prime Generator - Makefile Targets"
	@echo ""
	@echo "  make build              - Build the project"
	@echo "  make test               - Run basic tests"
	@echo "  make run                - Run (n=10000000)"
	@echo "  make run-quiet          - Only print count"
	@echo "  make run-progress       - With progress bar"
	@echo "  make clean              - Clean build artifacts"
	@echo "  make help               - Show this help"
	@echo ""
	@echo "Variables:"
	@echo "  N=50000000 make run     - Run with custom N"
