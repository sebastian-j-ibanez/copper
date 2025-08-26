#! /usr/bin/bash

# Copyright (c) 2025 Sebastian Ibanez
# Author: Sebastian Ibanez
# Created: 2025-08-25

# This script builds and copies
# the copper release binary to ~/.local/bin.

echo "[INFO] Building copper binary..."
build_time=$( { time cargo build --release; } 2>&1 )
real_time=$( echo "$build_time" | grep real | awk '{print $2}' )

echo "[INFO] Copying copper binary to ~/.local/bin..."
if ! [ -d ~/.local/bin ]; then
    mkdir -p ~/.local/bin
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cp "$SCRIPT_DIR/../target/release/copper" "$HOME/.local/bin/"

echo "[INFO] Setup complete!"
echo "[INFO] Build time: $real_time"
