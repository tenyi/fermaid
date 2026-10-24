# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

fermaid is a pure-Rust library that renders [Mermaid](https://mermaid.js.org) diagram text to SVG — no JavaScript runtime, no headless browser, no Node.js. It is inspired by [obsidianmd/pele](https://github.com/obsidianmd/pele), a small, synchronous, dependency-free TypeScript Mermaid-to-SVG renderer. Pele is the main reference for behavior and scope:

- Targets Mermaid 12.1.0 syntax, but with its own layout and visual style (pixel-identical output to Mermaid.js is not a goal).
- Each diagram type is a self-contained unit (parser + layout + SVG emitter) that can be registered on its own; the full entry point includes all types.
- Colors and fonts come from CSS variables in the emitted SVG (pele uses `--pele-bg`, `--pele-fg`, `--pele-line`, `--pele-accent`, `--pele-series-1..8`, …), so output can be re-themed without re-rendering.
- Untrusted input is a first-class concern: escape all diagram text, filter links and inline styles, and cap source and output size.
- Parsers are validated against Mermaid's own parser specs and with differential fuzzing.

The project is at the very beginning: only the `cargo init --lib` skeleton exists. The development plan and all settled design decisions are in `docs/ROADMAP.md`. Read it before starting work. Key points:

- Algorithms, text metrics and limits are ported from pele v1.9.0 (`e7f092a`), written as idiomatic Rust (not a line-by-line translation).
- Zero runtime dependencies. `std`, MSRV 1.85. One Cargo feature per diagram type.
- Planned workspace: `crates/fermaid` (lib), `crates/fermaid-cli`, `xtask/`, `fuzz/`.
- CSS variables use the `--fermaid-*` prefix with pele's suffixes.
- Output must be deterministic (byte-identical for identical input and options).
- An explicit nesting depth limit is required, because a Rust stack overflow aborts and cannot be caught.

Update this file as real architecture lands.

## Commands

```sh
cargo build
cargo test                       # all tests
cargo test <name_substring>      # single test / filter
cargo test -- --nocapture        # show println! output
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

Rust edition is 2024.

## Language and documentation rules

This is an open-source project for a worldwide audience.

- Code, comments, `CLAUDE.md`, commit messages, issues/PR text, and `README.md` are written in **English**.
- Translated READMEs live alongside it: `README.zh-TW.md` (Traditional Chinese), `README.zh-CN.md` (Simplified Chinese), `README.ja.md` (Japanese). When `README.md` changes in substance, update all three translations in the same change and keep the language-switcher line at the top of each file in sync.
