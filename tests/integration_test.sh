#!/bin/bash
set -e

# ==========================================
# ConfigSync Master Verification Script
# ==========================================

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[TEST] $1${NC}"
}

error() {
    echo -e "${RED}[FAIL] $1${NC}"
    exit 1
}

# Cleanup
rm -rf ~/.config/configsync
rm -f /tmp/normal_file
rm -f /tmp/work_file
rm -f /tmp/secret_file
rm -f ~/.local/share/configsync/key.txt

# Build
log "Building..."
cargo build

# 1. Initialize
log "Step 1: Initialization"
./target/debug/configsync init --role work
if [ ! -d ~/.config/configsync ]; then error "Repo not created"; fi

# 2. Add Normal File
log "Step 2: Add Normal File"
touch /tmp/normal_file
echo "Normal Content" > /tmp/normal_file
./target/debug/configsync add /tmp/normal_file
if [ ! -L /tmp/normal_file ]; then error "Normal file not symlinked"; fi

# 3. Add Role File
log "Step 3: Add Role File (Work)"
touch /tmp/work_file
echo "Work Content" > /tmp/work_file
./target/debug/configsync add /tmp/work_file --role work
if [ ! -L /tmp/work_file ]; then error "Work file not symlinked"; fi

# 4. Secrets
log "Step 4: Secrets Management"
./target/debug/configsync secrets init
if [ ! -f ~/.local/share/configsync/key.txt ]; then error "Key not generated"; fi

touch /tmp/secret_file
echo "Secret Content" > /tmp/secret_file
./target/debug/configsync secrets add /tmp/secret_file
if [ ! -f ~/.config/configsync/secrets/secret_file.age ]; then error "Secret not encrypted in repo"; fi


# 5. Doctor Check (Should be clean)
log "Step 5: Doctor Check (Clean)"
./target/debug/configsync doctor

# 6. Rollback / Undo
log "Step 6: Rollback (Undo)"
# Commit current state first if not already
cd ~/.config/configsync
git add .
git commit -m "State before bad change" || true
cd -

# Make a bad change
echo "Bad Content" > ~/.config/configsync/normal_file
cd ~/.config/configsync
git add .
git commit -m "Bad Commit"
cd -

# Verify bad content via symlink
if [ "$(cat /tmp/normal_file)" != "Bad Content" ]; then error "Symlink didn't reflect bad content"; fi

# Undo
./target/debug/configsync undo

# Verify restored content
if [ "$(cat /tmp/normal_file)" != "Normal Content" ]; then error "Undo failed to restore content"; fi


# 7. Simulation: New Machine / Restore
log "Step 7: Restore Simulation"
# Delete all local files to simulate fresh machine
rm /tmp/normal_file
rm /tmp/work_file
rm /tmp/secret_file

# Run Pull/Apply
./target/debug/configsync pull

# Verify Normal
if [ ! -f /tmp/normal_file ]; then error "Normal file not restored"; fi
# Verify Work (Match Role)
if [ ! -f /tmp/work_file ]; then error "Work file not restored"; fi
# Verify Secret (Decrypt)
if [ ! -f /tmp/secret_file ]; then error "Secret file not restored"; fi
if [ "$(cat /tmp/secret_file)" != "Secret Content" ]; then error "Secret content mismatch"; fi

log "🎉 ALL SYSTEMS GO! Release v0.3.1 is stable."
