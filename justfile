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