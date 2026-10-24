# fermaid

**English** | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

fermaid renders [Mermaid](https://mermaid.js.org) diagrams to SVG in pure Rust — no JavaScript runtime, no headless browser, no Node.js. It is a port of [pele](https://github.com/obsidianmd/pele), a small, dependency-free TypeScript renderer, and targets Mermaid 12.1.0 syntax with pele's own layout and visual style.

> **Status:** pre-alpha. Flowcharts, sequence, state, class, ER and requirement diagrams and pie charts are supported; every other diagram type returns an `Unsupported` error for now. Gantt charts, timelines, journeys, kanban boards and mind maps come next. See the [roadmap](docs/ROADMAP.md).

| Diagram type | Status |
|---|---|
| Pie (`pie`) | Supported: `showData`, `title`, `accTitle`, `accDescr`, and the `pie` config keys `textPosition`, `donutHole` and `legendPosition` |
| Flowchart (`flowchart`, `graph`) | Supported: every node shape, including `@{ shape: … }`, all edge kinds and lengths, edge ids, subgraphs with their own direction, `classDef`, `class`, `style`, `linkStyle`, `click` links and tooltips, markdown labels, and the `flowchart` config keys `curve`, `nodeSpacing`, `rankSpacing` and `wrappingWidth` |
| Sequence (`sequenceDiagram`) | Supported: participants and actors of every kind, every arrow kind, activations, notes, `loop`, `alt`, `opt`, `par`, `critical`, `break` and `rect` blocks, boxes, `autonumber`, created and destroyed participants, and the `sequence` config keys. Participant menus (`link`, `links`) are not drawn as links |
| State (`stateDiagram`, `stateDiagram-v2`) | Supported: states with descriptions, composite states with their own direction, concurrency regions, start and end, choice, fork and join, notes, `classDef`, `class`, `style` and `click` |
| Class (`classDiagram`) | Supported: classes with members, generic types and annotations, relations with markers and cardinalities, namespaces, notes, lollipop interfaces, `classDef`, `cssClass`, `style`, `link`, `click` and `callback` |
| ER (`erDiagram`) | Supported: entities with attributes, keys and comments, relationships with crow's foot cardinalities, aliases, subgraphs, `classDef`, `class` and `style` |
| Requirement (`requirementDiagram`) | Supported: requirements of every kind and elements with their fields, relationships, `classDef`, `class` and `style` |
| The rest | Planned |

## Goals

- Pure Rust, synchronous, no runtime dependencies, no I/O in the library.
- Mermaid syntax compatibility, with its own clean layout and visual style.
- Deterministic output: the same input and options give byte-identical SVG.
- Themeable output: colors and fonts are CSS variables in the SVG.
- Safe for untrusted input: escaped text, filtered links and styles, size and nesting limits.

## Library

```rust
let options = fermaid::RenderOptions {
    font_size: Some(14.0),
    ..Default::default()
};
match fermaid::render("pie\n  \"Rust\": 70\n  \"Other\": 30", &options) {
    Ok(rendered) => println!("{}", rendered.svg),
    Err(e) => eprintln!("{}:{}: {}\n{}", e.line, e.column, e.message, e.snippet),
}
```

`render` returns the SVG with its size and links, or an `Error` with a code (`Unsupported`, `Syntax`, `Semantic`, `Limit`), a line and column, and a snippet that points at the problem. `detect_type` tells the diagram type without rendering. Set `max_width` to the width the host has, and a diagram that can draw itself narrower does (a pie chart moves its legend below the chart). Links from `click` statements are sanitized and kept only when their scheme is allowed (`link_schemes`, by default `http`, `https`, `mailto` and `tel`); a flowchart that runs across turns to run down when it does not fit `max_width`. Each diagram type is a Cargo feature (`flowchart`, `pie`, …); the default feature `all` includes every type.

## Command line

```sh
cargo install --path crates/fermaid-cli
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --max-width 360 --id-prefix fig1 > diagram.svg
```

## Theming

The SVG takes its colors and fonts from CSS custom properties, with built-in fallbacks, so a page can re-theme a diagram without rendering it again:

```css
svg {
  --fermaid-bg: #1e1e1e;
  --fermaid-fg: #dcddde;
  --fermaid-line: #999;
  --fermaid-accent: #7f6df2;
  --fermaid-font: "Inter", sans-serif;
}
```

Available variables: `--fermaid-bg`, `--fermaid-fg`, `--fermaid-muted`, `--fermaid-line`, `--fermaid-surface`, `--fermaid-surface-alt`, `--fermaid-border`, `--fermaid-accent`, `--fermaid-radius`, `--fermaid-font`, `--fermaid-font-mono`, and `--fermaid-series-1` to `--fermaid-series-8` for the colors of chart series such as pie slices.

## Development

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo +nightly fuzz run render    # needs cargo-fuzz; `pie`, `flowchart`, `sequence`, `state`, `class`, `er` and `requirement` are other targets
cargo xtask compat pie --pele ../pele   # regenerates the Mermaid spec cases from a pele checkout
node tools/oracle/golden.mjs ../pele flowchart   # regenerates what pele makes of them
node tools/oracle/diff.mjs ../pele   # differential fuzzing against Mermaid's own parsers
```

The minimum supported Rust version is 1.85.

## Credits

fermaid's algorithms, text metrics and limits are ported from [pele](https://github.com/obsidianmd/pele) by kepano (MIT), from the Obsidian team. The diagram syntax and the parser specifications used as tests come from [Mermaid](https://github.com/mermaid-js/mermaid) (MIT). See [NOTICE](NOTICE).

## License

Multi-licensed under your choice of:
- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)
- [BSD-3-Clause](LICENSE-BSD3)
- [0BSD](LICENSE-0BSD)

See [LICENSE](LICENSE) and [NOTICE](NOTICE) for details.
