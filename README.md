# fermaid

**English** | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

fermaid renders [Mermaid](https://mermaid.js.org) diagrams to SVG in pure Rust — no JavaScript runtime, no headless browser, no Node.js.

> **Status:** early development. There is no usable API yet.

## Goals

- Pure Rust, synchronous, minimal dependencies.
- Mermaid syntax compatibility, with its own clean layout and visual style.
- Themeable output: colors and fonts exposed as CSS variables in the SVG.
- Safe for untrusted input: escaped text, filtered links and styles, size limits.

## Inspiration

fermaid is inspired by [Pele](https://github.com/obsidianmd/pele) from the Obsidian team, a small, dependency-free TypeScript library that renders Mermaid diagrams to SVG. fermaid aims to bring the same approach to the Rust ecosystem.

## Development

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

## License

[MIT](LICENSE)
