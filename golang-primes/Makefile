.PHONY: all build run test clean help

# Default target
all: build

N ?= 10000000

build:
	go build -o primes ./cmd/primes

run: build
	./primes 1000

run-quiet: build
	./primes --quiet $(N)

run-progress: build
	./primes --progress $(N) > /dev/null

run-progress-parallel: build
	./primes --progress --parallel $(N) > /dev/null

test:
	go test ./...

test-verbose:
	go test -v ./...

clean:
	rm -f primes

rebuild: clean build

fmt:
	go fmt ./...

vet:
	go vet ./...

setup:
	@echo "Checking Go installation..."
	@go version 2>/dev/null || echo "Go not found. Please install from https://go.dev/dl/"
	@echo "Go setup complete."

help:
	@echo "Go Prime Generator - Makefile Targets"
	@echo ""
	@echo "  make build                    - Build binary"
	@echo "  make run                      - Run (n=1000)"
	@echo "  make run-quiet                - Count primes < 1M"
	@echo "  make run-progress             - With progress bar (1M)"
	@echo "  make run-progress-parallel    - Parallel + progress (uses N env var)"
	@echo "  make test                     - Run all tests"
	@echo "  make test-verbose             - Run tests with verbose output"
	@echo "  make clean                    - Remove binary"
	@echo "  make rebuild                  - Clean and rebuild"
	@echo "  make fmt                      - Format code"
	@echo "  make vet                      - Run vet"
	@echo "  make help                     - Show this help"
