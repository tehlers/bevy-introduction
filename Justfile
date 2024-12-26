# List available recipes
default:
    just --list

# Run an example
[group("Run")]
run example:
    @cargo run --quiet --example {{ example }}

# Format the source code
[group("Dev")]
format:
    cargo fmt

# Check whether the source code is formatted correctly
[group("Verify")]
check-format:
    cargo fmt --check

# Execute the tests
[group("Verify")]
test:
    # Uses cargo-nextest to run tests - see https://nexte.st/ for details
    cargo nextest run --no-tests=pass --examples

# Check the source code for common mistakes
[group("Verify")]
lint:
    cargo clippy

# Perform all checks of the source code
[group("Verify")]
check: check-format test lint
