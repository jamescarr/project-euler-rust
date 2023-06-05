# Variables
RUSTC = rustc
CARGO = cargo
SRC_DIR = src
PROBLEMS_DIR = $(SRC_DIR)/problems
MAIN_FILE = $(SRC_DIR)/main.rs

# Targets
.PHONY: all init run_tests clean build

all: init run_tests

init:
	@echo "Initializing project Euler..."
	@$(CARGO) build

run_tests:
	@echo "Running tests..."
	@$(CARGO) test -- --nocapture

clean:
	@echo "Cleaning up..."
	@$(CARGO) clean
	@rm -rf Cargo.lock

build:
	@echo "Building project..."
	@$(CARGO) build
	@if [ $$? -eq 0 ]; then \
		echo "Build successful."; \
	else \
		echo "Build failed."; \
		exit 1; \
	fi

problem%:
	@$(CARGO) run problem$*
	@if [ $$? -eq 0 ]; then \
		echo "Problem $* completed successfully."; \
	else \
		echo "Error running problem $*."; \
	fi

.PHONY: help
help:
	@echo "Project Euler Makefile:"
	@echo "  make init            - Initialize the project"
	@echo "  make run_tests       - Run tests for all problems"
	@echo "  make clean           - Clean up the project"
	@echo "  make build           - build the project"
	@echo "  make problemX        - Run problem X (e.g., make problem001 for problem 1)"
	@echo "  make help            - Show this help message"
