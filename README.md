# Pagurus

A minimal native SSH client written in Rust: a full terminal on the left, the server's file tree (SFTP) on the right.

Pagurus is meant to feel like a native utility. It starts instantly and stays light: the target is a binary of about 10–15 MB and tens of MB of RAM. It uses no webview, Electron or JVM.

> **Status:** early development.

## Planned features (v1)

- **Connect**
  - Hosts from `~/.ssh/config`, saved profiles, or manual entry.
  - Password, private key (with passphrase) or ssh-agent login.
  - Host keys are checked against `~/.ssh/known_hosts`.
  - Secrets are stored only in the system keychain, and only if you opt in.
- **Terminal**
  - `xterm-256color` with truecolor, the alternate screen, 10k lines of scrollback, mouse selection and UTF-8.
  - vim, htop, mc, less and tmux work.
- **File tree**
  - Lazy SFTP browsing.
  - Download files and folders.
  - Upload by drag and drop.
  - Create and delete folders.
  - Background transfers with progress and cancel.
- **Reconnect** button when the connection drops.

## Platforms

macOS (Apple Silicon) is primary. Windows 11 is second. Linux is best effort.

## Build

Requires stable Rust (edition 2024).

```sh
cargo run -p pagurus-app                            # run the app
cargo run -p pagurus-core --example shell -- <host> # GUI-less SSH shell for debugging
cargo build --release -p pagurus-app                # release build
```

## Layout

```
crates/core   SSH session, PTY channel, SFTP, ssh config, known_hosts, profiles — no GUI
crates/app    eframe/egui UI: terminal renderer, file tree, dialogs
```
