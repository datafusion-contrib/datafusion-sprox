[private]
default:
    @just --list

# Build the things
build:
    cargo build

# Test the things
test:
    echo Testing sprox...

# Lint the things
lint:
    echo Linting sprox...

# Run the dev server
serve:
    echo Serving development server...
