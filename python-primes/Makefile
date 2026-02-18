.PHONY: all run test clean help lint typecheck

# Default target
all: run

N ?= 10000000

run:
	python prime_generator.py 1000

run-quiet:
	python prime_generator.py $(N) 2>/dev/null

run-progress:
	python prime_generator.py $(N) --progress > /dev/null

run-progress-parallel:
	python prime_generator.py $(N) --progress --parallel > /dev/null

test:
	python -m pytest test_generators.py -v

test-verbose:
	python -m pytest test_generators.py -vv

lint:
	ruff check .

typecheck:
	mypy prime_generator.py test_generators.py

clean:
	rm -f __pycache__/*.pyc
	rm -f *.pyc

rebuild: clean run

fmt:
	ruff format .

setup:
	python3 -m venv venv
	. venv/bin/activate && pip install -r requirements.txt

help:
	@echo "Python Prime Generator - Makefile Targets"
	@echo ""
	@echo "  make run                      - Run (n=1000)"
	@echo "  make run-quiet                - Count primes (uses N, default 10M)"
	@echo "  make run-progress             - With progress bar"
	@echo "  make run-progress-parallel    - Parallel + progress"
	@echo "  make test                     - Run all tests"
	@echo "  make test-verbose             - Run tests with more detail"
	@echo "  make lint                     - Run ruff linter"
	@echo "  make typecheck                - Run mypy type checker"
	@echo "  make clean                    - Remove pycache"
	@echo "  make setup                    - Create venv and install deps"
	@echo "  make help                     - Show this help"
	@echo ""
	@echo "Variables:"
	@echo "  N=50000000 make run-progress  - Run with custom N"
