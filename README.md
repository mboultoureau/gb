# gb — Gameboy Emulator in Rust

A modular Gameboy emulator written in Rust. This project is both a functional emulator and a vehicle for learning Rust.

## Goals

### Primary

- Learn Rust by building a real, complex project
- Produce a correct, performant Gameboy emulator

### Optional

- Support web (WASM) and native runtimes
- Expose a modular core API usable for machine learning (e.g. training agents on Pokemon, Mario)
- Frame-by-frame save states and variable cycle rates for ML training acceleration
- Headless mode for CI/testing/training
- Full audio support
- All major gamepad/keyboard controller support

## Architecture

The project is split into two main parts:

- **`gb-core`** — The emulator core (CPU, memory, PPU, APU, timers, interrupts, cartridge MBCs). No platform dependencies. Headless-compatible.
- **`gb-interface`** — Frontends that consume `gb-core`:
  - `gb-web` — WebAssembly frontend for browser play
  - `gb-native` — Native desktop frontend (may share a package with `gb-web`)

## Project Values

- Idiomatic Rust: follow the standard patterns, use `clippy`, run `cargo fmt`
- Tested: unit tests for all components, integration tests with known-good ROMs
- CI/CD: GitHub Actions for test, lint, build, and deploy
- Lawful: only original/unlicensed content or clean-room reimplementations

## AI Usage

- All code is authored by the project maintainer. AI does not commit code without human review.
- Documentation (including this file) may be drafted with AI assistance, but all content is verified before inclusion.
- The AI's primary role is mentorship: identifying non-idiomatic patterns, suggesting improvements, and explaining Rust concepts.
- AI participates in code review on every pull request, challenging design decisions where a more idiomatic or performant alternative exists.
