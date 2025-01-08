# Basic Makefile for Rust projects
SHELL := /bin/bash

# Variables
CARGO := cargo
FMT := rustfmt
RUSTUP := rustup

# Help message (default target)
.PHONY: help
help:
	@echo "Rust Project Makefile"
	@echo "Commands:"
	@awk 'BEGIN {FS = ":.*?##"}; /^[a-zA-Z_-]+:.*?##/ {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Check and Install Tools
.PHONY: check-tools
check-tools: ## Check if necessary tools are installed, prompt to install if missing
	@command -v $(CARGO) >/dev/null 2>&1 || { \
		echo "Cargo is not installed. Please install Rust."; \
		exit 1; \
	}
	@$(CARGO) clippy --version >/dev/null 2>&1 || { \
		echo "Clippy is not installed. Do you want to install it? [y/N]"; \
		read ans; [ "$$ans" = "y" ] && $(RUSTUP) component add clippy || echo "Skipping Clippy installation."; \
	}
	@$(FMT) --version >/dev/null 2>&1 || { \
		echo "Rustfmt is not installed. Do you want to install it? [y/N]"; \
		read ans; [ "$$ans" = "y" ] && $(RUSTUP) component add rustfmt || echo "Skipping Rustfmt installation."; \
	}

# Formatting
.PHONY: format
format: check-tools ## Format all source files
	@$(CARGO) fmt

.PHONY: check-format
check-format: check-tools ## Check if formatting is correct
	@$(FMT) --check $(shell find . -name "*.rs")

# Building
.PHONY: build
build: ## Build the project in debug mode
	@$(CARGO) build

.PHONY: release
release: ## Build the project in release mode
	@$(CARGO) build --release

# Running
.PHONY: run
run: ## Run the project
	@$(CARGO) run

.PHONY: run-release
run-release: ## Run the project in release mode
	@$(CARGO) run --release

# Testing
.PHONY: test
test: ## Run tests
	@$(CARGO) test

.PHONY: test-release
test-release: ## Run tests in release mode
	@$(CARGO) test --release

# Linting
.PHONY: lint
lint: check-tools ## Run linter (clippy)
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

# Cleaning
.PHONY: clean
clean: ## Clean up build artifacts
	@$(CARGO) clean

# Listing
.PHONY: list
list: ## List all targets in this Makefile
	@awk 'BEGIN {FS = ":.*?##"}; /^[a-zA-Z_-]+:.*?##/ {print $$1}' $(MAKEFILE_LIST)

# Debug
.PHONY: debug
debug: ## Run the project with debugging environment variables
	@RUST_LOG=debug $(CARGO) run
