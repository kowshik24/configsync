# ConfigSync

<div align="center">
  <img src="logo.png" alt="ConfigSync Logo" width="200">
</div>

[![Rust CI](https://github.com/kowshik24/configsync/actions/workflows/ci.yml/badge.svg)](https://github.com/kowshik24/configsync/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/kowshik24/configsync.svg)](LICENSE)

**ConfigSync** is a team-oriented dotfile synchronization tool designed to bridge the gap between personal preference and team standards. It treats configuration as code, with versioning, diffing, and team-wide enforcement, without requiring users to manually manage git submodules or symlinks.

## The Problem

Every developer has dotfiles (`.zshrc`, `settings.json`, `.gitconfig`). Every team has shared configs (linters, formatters, IDE settings). Currently, teams solve this with:
*   **Git submodules**: Too complex for non-devs or quick setups.
*   **Slack/Email**: "Hey copy this" leads to drift immediately.
*   **Manual symlinking**: Fragile and platform-specific.

## The Solution

**ConfigSync** offers a CLI that autonomously manages your configuration state.

### Core Features

*   **Zero Git Knowledge Required**: The tool manages the repository state for you.
*   **Team-First Identity**: Join a team via URL; subscribe to changes rather than forking.
*   **Cross-Platform Symlinks**: Abstracted away for Linux, macOS, and Windows.
*   **Conflict Resolution**: (Coming Soon) Intelligent merging of local vs remote changes.

## Installation

### From Crates.io (Recommended)

Once published, you can install directly via cargo:

```bash
cargo install configsync
```

### From Source

```bash
git clone https://github.com/kowshik24/configsync.git
cd configsync
```


2.  Install locally:
    ```bash
    cargo install --path .
    ```

Or build manually and copy the binary:
```bash
cargo build --release
cp target/release/configsync ~/.local/bin/
```

## Quick Start

### 1. Initialize
To start tracking your dotfiles or clone an existing configuration repostiory:

```bash
# Initialize a empty local repository
configsync init

# OR clone an existing team repository
configsync init --url https://github.com/my-team/configs.git
```

### 2. Add a File
To add a file to the sync (e.g., your `.vimrc`):

```bash
configsync add ~/.vimrc
```
This moves `~/.vimrc` to the local repository and creates a symlink in its place.

### 3. Sync Changes
To push your local changes upstream:

```bash
configsync push
```

To pull the latest team standards:

```bash
configsync pull
```

### 4. Secrets Management 🔒
Encrypt sensitive files (like `.env`, `id_rsa`) before syncing.

```bash
# Initialize keys (run once per machine)
configsync secrets init

# Add a secret file
configsync secrets add ~/.env
```
The file is encrypted in the repo and automatically decrypted when you `pull`.

### 5. Rollback & History ⏪
Made a mistake? View history and undo changes.

```bash
# View change log
configsync history

# Undo last change
configsync undo
```

### 6. Doctor 🩺
Diagnose issues like broken symlinks or missing keys.

```bash
configsync doctor
```

## Architecture

ConfigSync is built in Rust for performance and reliability.
- **Core**: Handles git operations via `libgit2`, file system abstraction, and configuration management.
- **CLI**: Powered by `clap` for a robust command-line experience.
- **Storage**: Uses TOML for configuration (`team-config.toml`) and standard Git for version control.

## Roadmap

- [x] Basic Sync (Init, Add, Push, Pull)
- [x] Daemon Mode (Auto-sync)
- [x] Role System (Work/Personal)
- [x] Secret Management (via `age`)
- [x] Rollback System (History/Undo)
- [x] Doctor (Diagnostics)
- [x] Windows Junction Support
- [ ] Conflict Resolution TUI

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
