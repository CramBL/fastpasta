import 'scripts/unique_error_codes.just'
import 'scripts/check_version_tag.just'
import 'scripts/test_coverage.just'
import 'scripts/sanitizers.just'
import 'scripts/util.just'

# Absolute path to the directory containing the utility recipes to invoke them from anywhere
## USAGE: `{{PRINT}} green "Hello world"`
PRINT := join(justfile_directory(), 'scripts/pretty_print.just')
## Usage: `{{PROMPT}} "Are you sure?"` (returns 0 if user answers "yes", 1 otherwise)
PROMPT := join(justfile_directory(), 'scripts/prompt.just') + " prompt"

[private]
@default:
    just --list

alias c := check
alias b := build
alias t := test
alias l := lint
alias fmt := format
alias fc := full-check

# Run Full checks and format
full-check: check format lint check-unique-error-codes test

# Check if it compiles without compiling
check *ARGS:
    cargo check {{ ARGS }}

# Build the application
build *ARGS:
    cargo build {{ ARGS }}

# Run the tests
test *ARGS:
    cargo test {{ ARGS }}
    ./tests/regression/regression_tests.sh

# Run tests and collect coverage
test-coverage: run-test-coverage
# Open the test report that comes out of the test-coverage recipe
coverage-report: open-coverage-report

# Lint the code
lint *ARGS="-- -D warnings --no-deps":
    cargo clippy {{ ARGS }}

# Format the code
format *ARGS:
    cargo fmt {{ ARGS }}

# Build the documentation (use `--open` to open in the browser)
doc *ARGS:
    cargo doc {{ ARGS }}

# Publish to crates.io
publish:
    cargo publish

# Run the application (use `--` to pass arguments to the application)
run *ARGS:
    cargo run {{ ARGS }}

# Needs the rust toolchain
env:
    rustc --version
    cargo --version

# List the dependencies
deps:
    cargo tree

# Update the dependencies
update:
    cargo update

# Audit Cargo.lock files for crates containing security vulnerabilities
audit *ARGS:
    #!/usr/bin/env bash
    if ! which cargo-audit >/dev/null; then
        {{PRINT}} yellow "cargo-audit not found"
        just prompt-install "cargo install cargo-audit"
    fi
    cargo audit {{ ARGS }}

# Clean the `target` directory
clean:
    cargo clean

### CI variants with higher verbosities and slightly different configurations

# Full lint suite
[private]
ci_lint: \
    (check "--verbose") \
    (lint "--verbose") \
    check-version \
    (format "-- --check --verbose") \
    (doc "--verbose") \
    check-unique-error-codes

# Run tests
[private]
ci_test: \
    (test "--verbose")
