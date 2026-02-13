#!/bin/bash
set -e

# Cleanup for watch test
rm -rf ~/.config/configsync
pkill -f "target/debug/configsync watch" || true

# Init
./target/debug/configsync init

# Add a file
echo "initial content" > /tmp/testfile_watch
./target/debug/configsync add /tmp/testfile_watch

# Start watch in background
echo "Starting watcher..."
./target/debug/configsync watch > watch.log 2>&1 &
WATCH_PID=$!

# Wait for watcher to start
sleep 2

# Modify file (through the symlink or original? symlink points to original. 
# But watch watches the repo. So we must modify the file IN the repo?
# NO. Users edit the original file (which is a symlink to the repo file).
# Wait, `add` moves file to repo and symlinks back.
# So editing `/tmp/testfile_watch` actually edits `~/.config/configsync/testfile_watch`.
# The watcher watches `~/.config/configsync`.
# So modifying `/tmp/testfile_watch` Should trigger the watcher.
echo "new content" >> /tmp/testfile_watch

# Wait for debounce (2s) + processing
echo "Waiting for sync..."
sleep 5

# Check git log
cd ~/.config/configsync
echo "Checking git log..."
if git log -1 --pretty=%B | grep -q "Auto-sync"; then
    echo "SUCCESS: Auto-sync commit found"
else
    echo "FAILURE: Auto-sync commit NOT found"
    echo "--- Watch Log ---"
    cat ../../../personal_work/configsync/watch.log
    kill $WATCH_PID
    exit 1
fi

# Cleanup
kill $WATCH_PID
rm /tmp/testfile_watch
echo "Verification complete"
