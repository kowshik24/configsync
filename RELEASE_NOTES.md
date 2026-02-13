# v0.1.0: The "Personal with Training Wheels" MVP

This is the first stable release of **ConfigSync**, designed for personal dotfile management with an eye towards team synchronization.

## Features

- **Initialization**: Easily start tracking dotfiles or clone an existing remote repo (`configsync init`).
- **Add Files**: securely move a config file to the repo and replace it with a symlink (`configsync add`).
- **Syncing**: 
  - `configsync push`: Commit and push changes to the remote.
  - `configsync pull`: Pull updates and automatically apply symlinks.
- **Cross-Platform**: Basic support for Linux (and macOS via Unix-like behavior).

## Installation

Since we are not yet on crates.io, you can download the binary attached to this release and place it in your PATH, or build from source:

```bash
cargo install --git https://github.com/kowshik24/configsync.git
```

## SHA256 Checksum
`3c422fbeef7545052aa3382c113cf9163d911ded8b33786b2d5d44275cfd150a` configsync
