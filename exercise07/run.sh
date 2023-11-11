#!/bin/bash

if [ -f "Cargo.toml" ]; then
    # Build the Rust project using cargo
    cargo build

    # Check if the build was successful
    if [ $? -eq 0 ]; then
        echo "Build successful. Running the program..."
        
        # Run the Rust program
        cargo run
    else
        echo "Build failed."
    fi
else
    echo "Error: Cargo.toml not found in the current directory."
fi