# Release Notes

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
