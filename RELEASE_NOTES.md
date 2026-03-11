# Release Notes

## v0.3.5 - CI Reliability Gate + Real-World Test Hardening

### CI/CD Improvements
- Expanded CI to run strict formatting, clippy (`-D warnings`), Rust tests, and shell integration checks.
- Added cross-platform build verification on Linux, macOS, and Windows.
- Updated release workflow to require verification before publishing crate/binaries.
- Fixed isolated HOME integration checks to respect XDG paths (`XDG_CONFIG_HOME`, `XDG_DATA_HOME`), preventing false negatives in CI.

### Runtime / Robustness
- Removed `unwrap()` usage in CLI project directory resolution paths.
- Improved Windows link behavior with directory symlink + junction fallback.
- Improved pull conflict guidance for non-fast-forward scenarios.

### Test Coverage
- Added and expanded CLI integration tests for real-use edge cases:
  - uninitialized flows
  - missing secret key behavior
  - repository basename collision on add
  - apply conflict handling
  - pull behavior without `origin`

## v0.3.4 - Test Coverage + Sync UX Hardening 🔧

### New Tests
- Added Rust CLI integration tests for:
  - fresh `init` + `history` behavior
  - `undo` root-commit safeguard
  - `push` local commit behavior without global git identity
  - `apply` command and already-linked symlink UX

### Sync UX Improvements
- `apply` now reports `Already linked. Skipping.` when destination symlink is already correct.
- `apply` now reports clearer destination inspection errors instead of generic failures.
- Improved `push`/`pull` warning messages for missing `origin` remote configuration.

### Pull/Branch Handling
- `pull` now attempts branch fetch with dynamic branch detection and fallbacks (`main`, `master`) for better interoperability.

## v0.3.3 - First-Install Reliability & CLI UX ✅

### Reliability Fixes
- Added a robust git signature fallback for commit operations when `user.name` / `user.email` are not configured globally.
- `init` now creates an initial repository commit so `history` and `undo` do not fail on unborn branches.
- `history` now handles empty/unborn branch state gracefully.
- `undo` now provides a clear message when there are no commits to revert.
- `undo` now refuses to revert the initial/root commit to prevent accidental teardown of initial setup files.

### CLI Improvements
- Added `configsync apply` as a first-class command.
- Removed duplicate `apply` invocation in the `undo` flow.
- Improved `pull` behavior by resolving branch names dynamically instead of hardcoding `main`.

### Validation / Tooling
- Updated integration script to set repo-local git identity for deterministic rollback tests.
- Revalidated with `cargo fmt`, `cargo clippy -D warnings`, `cargo test --all-targets`, and full integration script execution.

## v0.3.2 - Critical Runtime Fix 🩹

### Bug Fixes
- Fixed a critical issue where `configsync` failed to run outside a source directory due to a runtime dependency on `Cargo.toml` introduced by `clap` configuration.
- Removed `clap`'s `cargo` feature to ensure the binary is completely standalone.

## v0.3.1 - Enterprise Upgrade & Fixes 🚀

This release introduces powerful features for safety and security, plus important fixes.

### New Features
- **Secrets Management** 🔒: 
    - Encrypt sensitive files (like `.env`, `id_rsa`) using `age` encryption.
    - `configsync secrets init`: Generate local key pair.
    - `configsync secrets add <file>`: Encrypt and track.
    - Automatic decryption on `pull` / `apply`.
- **Rollback System** ⏪:
    - `configsync history`: View recent changes.
    - `configsync undo`: Revert the last change safely.
- **Doctor** 🩺:
    - `configsync doctor`: Diagnose broken symlinks, missing keys, and config issues.
- **CI/CD**: Automated integration tests and publishing.

### Improvements & Fixes
- Fixed Clippy lints for cleaner codebase.
- Improved cleanup of verification scripts.
- Enhanced documentation and `README.md`.
- Better error reporting in CLI.


## v0.2.0 - Role System & Watcher
- Added `watch` command (daemon mode).
- Added Role support (`--role work/personal`).
- Added Windows Junction support.

## v0.1.0 - MVP
- Initial release.
- Basic `init`, `add`, `push`, `pull` commands.
