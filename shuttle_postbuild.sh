#!/bin/bash

# Download and extract the trunk binary
curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.3/trunk-x86_64-unknown-linux-gnu.tar.gz -o trunk.tar.gz
tar -xzf trunk.tar.gz

# Clean up the tar file
rm trunk.tar.gz

# Run trunk build
cd /app/site && /app/trunk clean && /app/trunk build --release