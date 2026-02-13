# Stellar TUI

<div align="center">

[![CodeQL](https://github.com/padparadscho/stellar-tui/actions/workflows/codeql.yml/badge.svg)](https://github.com/padparadscho/stellar-tui/actions/workflows/codeql.yml)
[![Crates.io](https://img.shields.io/crates/v/stellar-tui.svg)](https://crates.io/crates/stellar-tui)
[![Docs.rs](https://docs.rs/stellar-tui/badge.svg)](https://docs.rs/stellar-tui)

</div>

A **terminal user interface (TUI)** for exploring and executing [Stellar RPC](https://developers.stellar.org/docs/data/apis/rpc) methods.

Select a method, fill in parameters, send the request, and inspect the JSON response, all from the terminal.

Built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm).

## Features

- Three-pane layout: methods list, request form, and response viewer.
- Structured request forms with type badges, inline validation, and contextual hints.
- Network management for switching between RPC endpoints.
- Method documentation with links to official Stellar docs.
- Fullscreen toggle for request and response panes.
- Response pagination for large payloads.
- Search within responses with regex support.
- Copy to clipboard support for responses by selection or page.
- Responsive layout that adapts to narrow terminals (< 110 columns).
- Mouse support for pane focus, navigation, and scrolling.

## Prerequisites

- [Rust](https://rust-lang.org/) **1.93.0** or later.

## Installation

### From [crates.io](https://crates.io/crates/stellar-tui)

```sh
# Install
cargo install stellar-tui

# Run
stellar-tui
```

### From source

```sh
# Clone and install from the repository
git clone https://github.com/padparadscho/stellar-tui.git
cd stellar-tui

cargo install --path .

# Run
stellar-tui
```

### Local Development

```sh
# Clone the repository and run locally
git clone https://github.com/padparadscho/stellar-tui.git
cd stellar-tui

cargo build # Debug build
cargo run # Run locally
cargo test # Test suite
cargo clippy # Lint

cargo build --release # Optimized binary at target/release/stellar-tui
```

## Usage

The default configuration connects to the Stellar **Testnet** endpoint at `https://soroban-testnet.stellar.org`. Additional endpoints can be added through the Settings modal (`s`).

### Keybindings

| Key              | Action                                                              |
| ---------------- | ------------------------------------------------------------------- |
| `Tab`            | Cycle focus between panes. Previous / next search result            |
| `Up` / `Down`    | Navigate items or scroll content                                    |
| `Left` / `Right` | Move caret in editable fields/search. Previous / next response page |
| `Backspace`      | Delete character before cursor                                      |
| `Delete`         | Delete character after caret                                        |
| `r` / `Ctrl+R`   | Execute the selected method                                         |
| `f` / `Ctrl+F`   | Toggle fullscreen (request or response)                             |
| `n` / `Ctrl+N`   | Cycle active network                                                |
| `i` / `Ctrl+I`   | Method documentation                                                |
| `s` / `Ctrl+S`   | Settings                                                            |
| `a` / `Ctrl+A`   | About                                                               |
| `p` / `Ctrl+P`   | Purge request + response                                            |
| `c` / `Ctrl+C`   | Copy selection or current response page                             |
| `Home` / `End`   | Jump to start / end of response page                                |
| `Esc`            | Close modal or exit fullscreen                                      |
| `q` / `Ctrl+Q`   | Quit                                                                |

- Mouse click focuses a pane.
- Scroll navigates methods, fields, or response content.
- In the response pane, mouse drag selects text:
  - When selection exists, `c` / `Ctrl+C` copies selected text.
  - When no selection exists, `c` / `Ctrl+C` copies current response page.
- When search is not active in response pane, `Left` / `Right` switch pages.

## Supported Methods

All methods use JSON-RPC 2.0 over HTTP POST. See the [Stellar RPC API Reference](https://developers.stellar.org/docs/data/apis/rpc/api-reference/methods) for parameter details.

| Method                | Parameters                                                                  |
| --------------------- | --------------------------------------------------------------------------- |
| `getHealth`           | —                                                                           |
| `getFeeStats`         | —                                                                           |
| `getLatestLedger`     | —                                                                           |
| `getNetwork`          | —                                                                           |
| `getVersionInfo`      | —                                                                           |
| `getEvents`           | startLedger, endLedger, cursor, limit, type, contractIds, topics, xdrFormat |
| `getLedgerEntries`    | keys                                                                        |
| `getLedgers`          | startLedger, cursor, limit, xdrFormat                                       |
| `getTransaction`      | hash, xdrFormat                                                             |
| `getTransactions`     | startLedger, cursor, limit, xdrFormat                                       |
| `sendTransaction`     | transaction (base64 envelope)                                               |
| `simulateTransaction` | transaction, instructionLeeway, authMode, xdrFormat                         |

## Settings

The config file is resolved in this order:

| Platform | Path                                                                |
| -------- | ------------------------------------------------------------------- |
| Linux    | `~/.config/stellar-tui/config.json`                                 |
| macOS    | `~/Library/Application Support/org.stellar.stellar-tui/config.json` |

Falls back to `./config.json` in the working directory if the platform config directory is unavailable.

## Contributing

If you're interested in helping improve the `stellar-tui` project, please see the [CONTRIBUTING](/CONTRIBUTING.md) file for guidelines on how to get started.

## License

This project is licensed under the [MIT License](/LICENSE).
