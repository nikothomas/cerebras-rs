# Makefile for Cerebras Rust SDK

.PHONY: generate clean build test

# Generate code from OpenAPI spec
generate:
	openapi-generator generate -g rust -i openapi.yaml -c config.yaml

# Clean build artifacts
clean:
	cargo clean

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Build and run tests
all: generate build test
