# List available recipes
default:
    just --list

# Run an example
[group("Run")]
run example:
    @RUST_LOG=error cargo run --quiet --example {{ example }}

# Start presentation
[group("Run")]
present:
    presenterm --enable-snippet-execution --present bevy-introduction.md

# Start presentation in preparation mode
[group("Dev")]
prepare:
    presenterm --enable-snippet-execution --validate-overflows bevy-introduction.md

# Format the source code
[group("Dev")]
format:
    cargo fmt

# Update example code in presentation
[group("Dev")]
update-example-code:
    #! /usr/bin/env sh
    set -euxo pipefail
    UPDATED_PRESENTATION=$(mktemp)
    ./tools/update-example-code.py bevy-introduction.md > $UPDATED_PRESENTATION
    mv $UPDATED_PRESENTATION bevy-introduction.md

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
