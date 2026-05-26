#!/bin/sh
set -e

echo "Building drip..."
cargo build --release

echo "Installing to /usr/local/bin/drip..."
sudo cp target/release/drip /usr/local/bin/drip

echo "Done. Run 'drip enter' to get started."
