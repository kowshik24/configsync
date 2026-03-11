# ConfigSync

<div align="center">
  <img src="logo.png" alt="ConfigSync Logo" width="160">
  <p><strong>Safe, team-friendly dotfile synchronization in Rust.</strong></p>

  <p>
    <a href="https://github.com/kowshik24/configsync/actions/workflows/ci.yml"><img src="https://github.com/kowshik24/configsync/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="LICENSE"><img src="https://img.shields.io/github/license/kowshik24/configsync.svg" alt="License"></a>
  </p>
</div>

ConfigSync manages your local configuration files as code, with git-backed history, role-aware syncing, encrypted secrets, and safety-focused recovery commands.

## Why ConfigSync

- Track config changes with commit history and rollback support.
- Sync shared dotfiles across machines without manually managing symlinks.
- Keep sensitive files encrypted in-repo and decrypted locally only.
- Apply machine roles (for example `work`, `personal`) to include/exclude files safely.

## Core Features

- Git-based sync: `init`, `push`, `pull`, `history`, `undo`
- Local apply engine: `apply` to restore links/files from tracked state
- Role-aware file targeting via `--role`
- Encrypted secrets with `age` via `secrets init` and `secrets add`
- Health diagnostics with `doctor`
- Optional watch mode with `watch`

## Installation

### 1. Binary release (recommended)

Download the latest binary from the [Releases page](https://github.com/kowshik24/configsync/releases).

### 2. Cargo

```bash
cargo install configsync
```

### 3. Build from source

```bash
git clone https://github.com/kowshik24/configsync.git
cd configsync
cargo install --path .
```

## Quick Start

### 1. Initialize repository state

```bash
configsync init --role work
```

Clone existing shared config repo instead:

```bash
configsync init --url https://github.com/<you>/<dotfiles>.git --role personal
```

### 2. Add files

```bash
configsync add ~/.zshrc
configsync add ~/.npmrc --role work
```

### 3. Sync and apply

```bash
configsync push
configsync pull
configsync apply
```

## Command Reference

| Command | Purpose |
|---|---|
| `configsync init [--url <repo>] [--role <role> ...]` | Initialize local ConfigSync repository metadata |
| `configsync add <path> [--role <role> ...]` | Track a file or directory and replace destination with symlink |
| `configsync push` | Commit local repo changes and push to remote (if configured) |
| `configsync pull` | Pull remote changes (if configured), then apply locally |
| `configsync apply` | Re-apply tracked state to local filesystem |
| `configsync history` | Show recent commit history |
| `configsync undo [<commit>]` | Revert a commit (safeguards prevent undoing root commit) |
| `configsync doctor` | Validate repository, file links, and secret key state |
| `configsync watch` | Start watch mode for automatic sync workflows |
| `configsync secrets init` | Generate local secret key |
| `configsync secrets add <path>` | Encrypt and track a secret file |

## Secrets and Security

- Secret files are stored encrypted in the repo (`.age`).
- Private key is stored locally (default path under `~/.local/share/configsync/key.txt`).
- Back up your key securely. Without it, encrypted files cannot be decrypted.
- On Unix, restored secret file permissions are tightened (`600`).

## Operational Notes

- If no `origin` remote exists, `push`/`pull` keep local behavior and print guidance.
- `apply` skips paths that are already correctly linked.
- `undo` intentionally blocks reverting the initial repository commit to avoid teardown of baseline setup files.

## Troubleshooting

### `ConfigSync not initialized`

Run:

```bash
configsync init
```

### Missing remote warning during `push` or `pull`

Configure remote:

```bash
git -C ~/.config/configsync remote add origin <your-repo-url>
```

### Secret decryption issues

- Ensure key file exists on current machine.
- Re-run `configsync secrets init` only for new local key generation.
- Restore the original key backup if you need to decrypt older encrypted files.

### Run diagnostics

```bash
configsync doctor
```

## Recent Highlights

### v0.3.3

- First-install reliability improvements (`init` initial commit, safer `history`/`undo` behavior)
- Added explicit `apply` command
- Better commit signature handling when git identity is missing

### v0.3.4

- Added Rust CLI integration tests for critical flows
- Improved apply UX (`Already linked. Skipping.` for correct existing symlinks)
- Improved sync warnings and branch fallback handling in pull flows

## Contributing

Contributions are welcome. Please open an issue or submit a pull request.

## License

MIT. See [LICENSE](LICENSE).
