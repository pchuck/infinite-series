# AGENTS.md

## Build/Lint/Test Commands

### Running Tests
- **All tests**: `python -m pytest test_generators.py -v`
- **Single test file**: `python -m pytest test_generators.py::TestGeneratePrimes::test_small_input -v`
- **Single test class**: `python -m pytest test_generators.py::TestGeneratePrimes -v`
- **With coverage**: `python -m pytest test_generators.py --cov=. --cov-report=html`
- **Verbose output**: Add `-vv` for more detailed test names

### Code Quality
- **Type checking**: `mypy prime_generator.py test_generators.py` (if mypy installed)
- **Linting**: `ruff check .` or `flake8 .` (if configured)

### Running Applications
- **Prime generator**: `python prime_generator.py 100` or `python prime_generator.py --help`
- **Performance comparison**: `python performance_comparison.py`

## Code Style Guidelines

### General
- Python 3.12+ compatible code
- Use type hints for all function signatures
- Follow PEP 8 naming conventions (snake_case for functions/variables, PascalCase for classes)
- Keep lines under 100 characters when possible

### Imports
- Standard library imports first (`import`, `from`), then third-party (`tqdm`)
- Group imports logically: typing imports together, external packages together
- Use `from typing import List, Optional, Callable` format
- Type ignore comments only when necessary (e.g., `# type: ignore`)

### Type Hints
- Annotate all public functions with parameter and return types
- Use `Optional[T]` for nullable returns
- Use `Callable[[ArgType], ReturnType]` for callbacks
- Example: `def sieve_of_eratosthenes(n: int, callback: Optional[Callable[[int], None]] = None) -> List[int]:`

### Error Handling
- Raise appropriate exceptions (`TypeError`, `ValueError`) with descriptive messages
- Use try-except blocks to handle user input errors gracefully
- Provide fallback behavior when optional dependencies missing (e.g., `tqdm`)

### Naming Conventions
- Functions: `snake_case` (e.g., `generate_primes`, `sieve_of_eratosthenes`)
- Classes: `PascalCase` (e.g., `TestGeneratePrimes`)
- Constants: `UPPER_SNAKE_CASE`
- Variables: `snake_case`

### File Organization
- Main modules in root: `prime_generator.py`, `test_generators.py`, `performance_comparison.py`, `parallel_comparison.py`
- Each file has shebang and module docstring
- Use `if __name__ == "__main__":` pattern

### Progress Indicators
- Support optional progress bars via `tqdm` when available
- Provide fallback when `tqdm` not installed
- Accept `show_progress` parameter to control display

## Tool Use Tips

### File Operations
- **Edit files**: Use `edit` tool for precise string replacements
  - Must read file first
  - Match exact whitespace and indentation
  - Use `oldString` and `newString` parameters
- **Write files**: Use `write` tool for complete file replacement
  - Parameter is `filePath` (camelCase, not `file_path`)
  - Overwrites existing content

### Code Changes
- Follow existing code style in the file
- Add type hints to all function signatures
- Keep progress callback signatures consistent (iteration count, not percentage)
- Test changes before committing: `python -m pytest test_generators.py -v`

### Common Patterns
- **tqdm integration**: Pass iteration count to progress callback, let tqdm handle updates
  ```python
  with tqdm(total=max_iterations) as pbar:
      def callback(current):
          pbar.update(1)
  ```
- **Error handling**: Catch `ValueError` for user input, provide clear messages
- **Type hints**: Use `Optional[T]` for nullable returns, `Callable[[Arg], Ret]` for callbacks

### Testing Best Practices
- Run tests after any code changes: `python -m pytest test_generators.py -v`
- Check linting: `ruff check .`
- Verify type correctness if mypy available: `mypy *.py`
