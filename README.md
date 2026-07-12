# gb

[![CI](https://github.com/mboultoureau/gb/actions/workflows/ci.yml/badge.svg)](https://github.com/mboultoureau/gb/actions/workflows/ci.yml)

A modular Gameboy emulator in Rust — a learning project and a functional emulator.

## Quick Start

```console
$ cargo run -p gb_desktop -- path/to/rom.gb
```

Run tests:

```console
$ cargo test
```

## Project Layout

| Crate | Type | Description |
|-------|------|-------------|
| `gb_core` | library | Emulator core (CPU, memory, PPU, APU, timers, interrupts, cartridge MBCs). Platform-agnostic, headless-compatible. |
| `gb_desktop` | binary | Native desktop frontend. |

## Goals

- Learn Rust by building a real, complex project
- Produce a correct, performant Gameboy emulator

### Stretch Goals

- WebAssembly frontend (`gb-web`)
- Modular core API usable for ML training (agent training on Pokemon/Mario)
- Frame-by-frame save states and variable cycle rates for ML acceleration
- Headless mode for CI/testing/training
- Full audio and controller support

## Project Values

- **Idiomatic Rust**: follow standard patterns, `clippy`-clean, `cargo fmt`-compliant
- **Tested**: unit tests for all components, integration tests with known-good ROMs
- **CI/CD**: GitHub Actions for test, lint, build
- **Lawful**: only original/unlicensed content or clean-room reimplementations

## Required Secrets (for AI Code Review)

The AI code review workflow expects these secrets:

- `GEMINI_API_KEY` — Google Gemini API key
- `OPENROUTER_API_KEY` — OpenRouter API key

## Resources

- [Pandocs](https://gbdev.io/pandocs/) — Gameboy hardware reference
- [gbdev.io](https://gbdev.io/) — Home of the Gameboy development community
- [gekkio's docs](https://gekkio.fi/blog/) — Low-level Gameboy analysis
