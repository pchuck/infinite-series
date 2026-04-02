CC = gcc
CFLAGS = -Wall -Wextra -Werror -std=c11 -O2
CFLAGS_DEBUG = -Wall -Wextra -Werror -std=c11 -g -O0 -DDEBUG
LDFLAGS = -lpthread -lm

SRC_DIR = .
BUILD_DIR = build
BUILD_DIR_DEBUG = build_debug

LIB_SRC = $(SRC_DIR)/primes.c
LIB_OBJ = $(BUILD_DIR)/primes.o
LIB_OBJ_DEBUG = $(BUILD_DIR_DEBUG)/primes.o

CLI_SRC = $(SRC_DIR)/main.c
CLI_BIN = $(BUILD_DIR)/primes_cli
CLI_BIN_DEBUG = $(BUILD_DIR_DEBUG)/primes_cli

TEST_SRC = $(SRC_DIR)/test_primes.c
TEST_BIN = $(BUILD_DIR)/test_primes
TEST_BIN_DEBUG = $(BUILD_DIR_DEBUG)/test_primes

.PHONY: all clean test debug run-release run-debug help

all: $(CLI_BIN)

debug: $(CLI_BIN_DEBUG)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(BUILD_DIR_DEBUG):
	mkdir -p $(BUILD_DIR_DEBUG)

$(LIB_OBJ): $(LIB_SRC) $(SRC_DIR)/primes.h | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(LIB_OBJ_DEBUG): $(LIB_SRC) $(SRC_DIR)/primes.h | $(BUILD_DIR_DEBUG)
	$(CC) $(CFLAGS_DEBUG) -c $< -o $@

$(CLI_BIN): $(LIB_OBJ) $(CLI_SRC) | $(BUILD_DIR)
	$(CC) $(CFLAGS) $(LIB_OBJ) $(CLI_SRC) -o $@ $(LDFLAGS)

$(CLI_BIN_DEBUG): $(LIB_OBJ_DEBUG) $(CLI_SRC) | $(BUILD_DIR_DEBUG)
	$(CC) $(CFLAGS_DEBUG) $(LIB_OBJ_DEBUG) $(CLI_SRC) -o $@ $(LDFLAGS)

$(TEST_BIN): $(LIB_OBJ) $(TEST_SRC) | $(BUILD_DIR)
	$(CC) $(CFLAGS) $(LIB_OBJ) $(TEST_SRC) -o $@ $(LDFLAGS)

$(TEST_BIN_DEBUG): $(LIB_OBJ_DEBUG) $(TEST_SRC) | $(BUILD_DIR_DEBUG)
	$(CC) $(CFLAGS_DEBUG) $(LIB_OBJ_DEBUG) $(TEST_SRC) -o $@ $(LDFLAGS)

test: $(TEST_BIN)
	$(TEST_BIN)

test-debug: $(TEST_BIN_DEBUG)
	$(TEST_BIN_DEBUG)

run-release: $(CLI_BIN)
	$(CLI_BIN)

run-debug: $(CLI_BIN_DEBUG)
	$(CLI_BIN_DEBUG)

clean:
	rm -rf $(BUILD_DIR) $(BUILD_DIR_DEBUG)

help:
	@echo "C Primes Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build release binary (default)"
	@echo "  debug        - Build debug binary"
	@echo "  test         - Run tests with release build"
	@echo "  test-debug   - Run tests with debug build"
	@echo "  run-release  - Run release CLI"
	@echo "  run-debug    - Run debug CLI"
	@echo "  clean        - Remove build directories"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make all              # Build release binary"
	@echo "  make test             # Run all tests"
	@echo "  make run-release -n 100  # Generate primes up to 100"
