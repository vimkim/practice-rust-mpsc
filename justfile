# Justfile

# Default to "build" if no recipe is specified
_default: build

# Recipe to run Clippy and then build
build:
    cargo clippy -- -D warnings  # Run Clippy and treat warnings as errors
    cargo build                 # Proceed with the build if Clippy passes

# Recipe to run Clippy and then run the project
run:
    cargo clippy -- -D warnings  # Run Clippy and treat warnings as errors
    cargo run                   # Proceed with running if Clippy passes

# Recipe to run Clippy only
clippy:
    cargo clippy

# Recipe for clean builds
clean:
    cargo clean
