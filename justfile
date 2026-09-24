# Run `just` with no args to see this list.
default:
    @just --list

# Where the throwaway fake Unity project lives for manual testing.
test_env_dir := ".test-env"

# Build the CLI in debug mode.
build:
    cargo build

# Run the automated test suite (unit + integration).
test:
    cargo test

# Create a throwaway fake Unity project (Assets/, ProjectSettings/, Packages/) for manual testing.
test-env: build
    ./scripts/build_test_env.sh "{{test_env_dir}}"

# Remove the fake Unity project so `just test-env` can build a fresh one.
clean-test-env:
    rm -rf "{{test_env_dir}}"

# Run a quick doctor pass inside the fake project, rebuilding first.
doctor-check: test-env
    cd {{test_env_dir}} && ../target/debug/uinit doctor

# Locally install the current build of the CLI
local-install: build
    cargo install --path . --force

# Tag the version currently in Cargo.toml and push it, triggering the release pipeline.
publish:
    ./scripts/publish.sh

# Init a package project into the test environment
init-test-package:
    cd .test-env && uinit init --template package MyTestPackage

# Init a game project into the test environment
init-test-game:
    cd .test-env && uinit init --template game MyTestGame

# Format, lint, etc
check:
    cargo fmt --all
    cargo check --all -q --locked
    cargo clippy