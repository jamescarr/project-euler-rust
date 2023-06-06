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


next_problem:
	@$(eval PROBLEM_NUM=$(shell sh -c 'ls src/euler/problem_*.rs | wc -l | xargs expr 1 +'))
	@$(eval PROBLEM_NAME=problem_$(shell printf "%03d" $(PROBLEM_NUM)))
	@touch src/euler/$(PROBLEM_NAME).rs > /dev/null
	@awk '/\/\/ new problem mod/{print;print "pub mod $(PROBLEM_NAME);";print "";next}1' src/euler/mod.rs > tmp && mv tmp src/euler/mod.rs > /dev/null
	@echo "pub fn solve() {\n\tprintln!(\"$(PROBLEM_NAME) not yet implemented.\");\n}" >> src/euler/$(PROBLEM_NAME).rs
	@awk '/\/\/ new problem mapping/{print;print "\t\"$(PROBLEM_NAME)\" => $(PROBLEM_NAME)::solve(),";print "";next}1' src/euler/mod.rs > tmp && mv tmp src/euler/mod.rs > /dev/null
	@echo "Created new problem file: src/euler/$(PROBLEM_NAME).rs"

run_tests:
	@echo "Running tests..."
	@$(CARGO) test -- --nocapture

run_tests%:
	@echo "Running tests for problem$* ..."
	@$(CARGO) test problem$* -- --nocapture


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
		echo "Problem$* solved."; \
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
