#!/bin/bash
set -e

# Wait for previous cargo run to finish (optional, but good practice if running in parallel)
# But since I'm running this script, I assume the previous run is done or I can just run cargo run again.

# Cleanup
rm -rf ~/.config/configsync
rm -f ~/.dummy_config

# 1. Init
echo "Running init..."
cargo run -- init

# 2. Create dummy file
echo "Creating dummy file..."
echo "dummy content" > ~/.dummy_config

# 3. Add file
echo "Adding dummy file..."
cargo run -- add ~/.dummy_config

# 4. Verify
echo "Verifying..."
REPO_DIR=~/.config/configsync
if [ -L ~/.dummy_config ]; then
    echo "SUCCESS: ~/.dummy_config is a symlink"
else
    echo "FAILURE: ~/.dummy_config is not a symlink"
    exit 1
fi

if [ -f "$REPO_DIR/.dummy_config" ]; then
    echo "SUCCESS: File moved to repo"
else
    echo "FAILURE: File not found in repo"
    exit 1
fi

if grep -q ".dummy_config" "$REPO_DIR/team-config.toml"; then
    echo "SUCCESS: Config updated"
else
    echo "FAILURE: Config not updated"
    exit 1
fi

echo "All checks passed!"
