# ConfigSync

<div align="center">
  <img src="logo.png" alt="ConfigSync Logo" width="200">
  <p><strong>A safe, team-oriented dotfile synchronization tool for developers.</strong></p>

  [![Rust CI](https://github.com/kowshik24/configsync/actions/workflows/ci.yml/badge.svg)](https://github.com/kowshik24/configsync/actions/workflows/ci.yml)
  [![License](https://img.shields.io/github/license/kowshik24/configsync.svg)](LICENSE)
</div>

---

**ConfigSync** bridges the gap between personal preference and team standards. It treats your configuration as code—versioned, diffed, and synced across machines—without the headache of manual symlinks or complex git submodules.

## 🚀 Key Features

-   **Zero-Config Git**: ConfigSync manages the git repository for you. No need to memorize git commands for your dotfiles.
-   **Machine Roles**: Assign roles (e.g., `work`, `personal`) to files. Sync your `.vimrc` everywhere, but keep your `.ssh/config` work-specific.
-   **Secrets Management**: Encrypt sensitive files (like `.env`, `id_rsa`) using `age` encryption. Keys stay local; data syncs safely.
-   **Safety Nets**: Built-in `history` and `undo` commands let you revert mistakes instantly.
-   **Doctor**: diagnose broken symlinks, missing keys, or configuration drift with a single command.
-   **Daemon Mode**: Watch for changes and sync automatically in the background.
-   **Cross-Platform**: Works on Linux, macOS, and Windows (using native Junctions).

## 📦 Installation

### Option 1: Binary Release (Recommended)
Download the latest binary for your platform from the [Releases Page](https://github.com/kowshik24/configsync/releases).

### Option 2: Cargo
If you have Rust installed:
```bash
cargo install configsync
```

### Option 3: From Source
```bash
git clone https://github.com/kowshik24/configsync.git
cd configsync
cargo install --path .
```

## ⚡ Quick Start

### 1. Initialize
Start tracking your dotfiles. You can optionally tag this machine with a role (e.g., `work`).

```bash
configsync init --role work
```

*Already have a repo?* Clone it directly:
```bash
configsync init --url https://github.com/my-username/dotfiles.git --role personal
```

### 2. Add a File
Move a file to the repo and replace it with a symlink.

```bash
configsync add ~/.zshrc
```

### 3. Sync
Push your changes to the remote repository.

```bash
configsync push
```

Pull changes from other machines.

```bash
configsync pull
```

## 📚 User Guide

### Managing Files
Add files to your shared configuration. By default, files are synced to all machines.

```bash
# Add a global file
configsync add ~/.vimrc

# Add a file ONLY for 'work' machines
configsync add ~/.npmrc --role work
```

### Secrets Management 🔒
Never commit plain-text secrets. ConfigSync uses `age` to encrypt files before they hit the disk in the repo.

**1. Initialize Keys (Run once per machine)**
```bash
configsync secrets init
# Creates ~/.local/share/configsync/key.txt (Keep this safe! Backup manually!)
```

**2. Add a Secret**
```bash
configsync secrets add ~/.env
```
The file is encrypted as `secrets/.env.age` in the repo. It is automatically decrypted and restored when you run `configsync pull` or `configsync apply`.

### History & Undo ⏪
Made a mistake? No problem.

```bash
# View change log
configsync history

# Undo the last change (reverts commit and restores files)
configsync undo
```

### Daemon Mode 🔄
Don't want to run `push` manually? Start the watcher.

```bash
configsync watch
```
It monitors your tracked files and auto-commits changes.

### Diagnostics 🩺
Something feels wrong? Broken symlinks?

```bash
configsync doctor
```
The Doctor will check your config, repo status, symlinks, and keys, and suggest fixes.

## 🏗️ Architecture

ConfigSync is built in Rust for speed and reliability.
-   **Core**: Uses `libgit2` for git operations and `age` for encryption.
-   **Storage**: Configuration is stored in `team-config.toml`.
-   **Symlinks**: Uses native symlinks on Unix and Junctions on Windows.

## 🤝 Contributing

Contributions are welcome! Please check out the issues or submit a PR.

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
