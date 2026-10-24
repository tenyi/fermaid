# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

fermaid is a pure-Rust library that renders [Mermaid](https://mermaid.js.org) diagram text to SVG — no JavaScript runtime, no headless browser, no Node.js. It is inspired by [obsidianmd/pele](https://github.com/obsidianmd/pele), a small, synchronous, dependency-free TypeScript Mermaid-to-SVG renderer. Pele is the main reference for behavior and scope:

- Targets Mermaid 12.1.0 syntax, but with its own layout and visual style (pixel-identical output to Mermaid.js is not a goal).
- Each diagram type is a self-contained unit (parser + layout + SVG emitter) that can be registered on its own; the full entry point includes all types.
- Colors and fonts come from CSS variables in the emitted SVG (pele uses `--pele-bg`, `--pele-fg`, `--pele-line`, `--pele-accent`, `--pele-series-1..8`, …), so output can be re-themed without re-rendering.
- Untrusted input is a first-class concern: escape all diagram text, filter links and inline styles, and cap source and output size.
- Parsers are validated against Mermaid's own parser specs and with differential fuzzing.

The development plan and all settled design decisions are in `docs/ROADMAP.md`. Read it before starting work. Key points:

- Algorithms, text metrics and limits are ported from pele v1.9.0 (`e7f092a`), written as idiomatic Rust (not a line-by-line translation). When behavior is in doubt, run pele's TypeScript sources and compare.
- Zero runtime dependencies. `std`, MSRV 1.85 (no APIs stabilized later). One Cargo feature per diagram type; `default = ["all"]`.
- CSS variables use the `--fermaid-*` prefix with pele's suffixes.
- Output must be deterministic (byte-identical for identical input and options).
- An explicit nesting depth limit is required, because a Rust stack overflow aborts and cannot be caught.

## Architecture

M0 (foundation), M1 (pie), M2 (flowchart), M3 (sequence, differential fuzzing), M4 (state, class, ER, requirement) and M5 (gantt, timeline, journey, kanban, mindmap) are done; every other type returns `ErrorCode::Unsupported`.

- `crates/fermaid` — the library. `render` checks the source limit, runs `preprocess`, detects the type (`detect.rs`), looks it up (`diagram.rs`, an internal `Diagram` trait; one module per type, feature-gated), renders it and checks the output limit.
  - `diagrams/` — one module per type behind its feature (`pie/`: `parser.rs`, `model.rs`, `render.rs`, and `compat.rs`, which checks the generated Mermaid spec cases; `flowchart/`: a jison-style `lexer.rs` over UTF-16 code units, `parser.rs`, `db.rs` (the model), `graph.rs` (what is drawn), `shapes.rs` (shape names), `render.rs`, `compat.rs`; `sequence/`: `lexer.rs` (UTF-16, as flowchart), `parser.rs` (a flat statement list), `db.rs`, `colors.rs` (CSS color names for boxes), `actors.rs` (participant shapes), `render.rs`, `compat.rs`; `state/`, `class/`, `er/` and `requirement/` follow the same split of `lexer.rs`, `parser.rs`, `db.rs`, `graph.rs`, `render.rs` and `compat.rs`, with `markers.rs` for class and ER line ends; `timeline/` and `journey/` have `lexer.rs`, `parser.rs`, `db.rs`, `render.rs` and `compat.rs`; `kanban/` and `mindmap/` only `db.rs`, `render.rs` and `compat.rs`, over the shared outline parser; `gantt/` has `lexer.rs`, `parser.rs`, `db.rs` (scheduling), `dates.rs` (JavaScript's `Date` in UTC, dayjs formats and strict parsing), `axis.rs` (d3's time ticks and `axisFormat`), `render.rs` and `compat.rs`). `common/ordered.rs` is an insertion-ordered map, as a JavaScript `Map`; `common/generics.rs` reads `~T~` generic types. `common/tokens.rs` is the tokenizer and `Reader` for Langium-defined types (pele's regex token types as hand-written matchers); `common/fit.rs` fits a chart to `RenderOptions::max_width`; `common/literal.rs` breaks long words when a diagram is squeezed; `common/golden.rs` holds helpers of the golden compat tests. `common/scan.rs` is a small tokenizer for jison-style line grammars; `common/plain.rs` parses the line grammars of timeline and journey; `common/outline.rs` parses the indented outlines of kanban and mindmap.
  - `layout/` — `layered.rs` (pele's Sugiyama layout of one flat graph) and `compound.rs` (groups laid out on their own and placed in their parents, with ports where edges cross borders), and `components.rs` (disconnected parts wrapped into rows, for class diagrams). The arithmetic keeps pele's order, so coordinates match pele's exactly.
  - `url.rs` — link sanitizing and the scheme allowlist; `unicode.rs` — the `\p{L}` table jison lexers use.
  - `preprocess/` — `%%` comments, front matter (`yaml.rs`, a YAML subset), `%%{init}%%` (`json.rs`, strict JSON).
  - `text/` — `Style`, `TextMeasurer`, `MetricsMeasurer` (pele's Arial tables decoded at compile time), `entities.rs` (entity codes), `label.rs` (label parsing, measuring and wrapping; a port of pele's `text/label.ts` without math and icons).
  - `svg/` — escaping, number formatting, label `<text>` output, theme CSS variables and style resolution, the document root and titles, node `shapes.rs`, `edges.rs` (paths, loops), `markers.rs` (arrowheads) and `cluster.rs` (subgraph titles).
  - `error.rs` (`Error` with code, kind, line, column, snippet), `options.rs` (`RenderOptions`, `Limits`, `Rendered`, `Link`), `util.rs` (JavaScript-compatible whitespace, trimming and rounding).
  - `tests/` — one file per type (`pie.rs`, `flowchart.rs`, `sequence.rs`, `state.rs`, `class.rs`, `er.rs`, `requirement.rs`, `timeline.rs`, `journey.rs`, `kanban.rs`, `mindmap.rs`, `gantt.rs`) (render, security, limits, speed, insta snapshots in `snapshots/`), `options.rs` (the options every type reads: the nesting limit, the fonts, the fitting, the link allowlist), `support/mod.rs` (inert-output check, hostile payloads, mutator; shared by the type tests), `compat/<type>/` (generated `cases.rs`, for most types also the generated `expected.rs`, and a hand-written `SKIPPED.md`).
- `crates/fermaid-cli` — the `fermaid` binary, hand-rolled argument parsing.
- `xtask/` — `cargo xtask compat <type> --pele <DIR>` converts pele's `tests/compat/<type>` specs into `cases.rs`.
- `tools/oracle/` — dev-only Node scripts; each needs a pele checkout with `npm ci` done. `golden.mjs` writes what pele makes of each compat case, as `expected.rs`. `diff.mjs` is the differential run against Mermaid's own parsers (and `@braintree/sanitize-url` for links); fermaid's side is the ignored `oracle_dump` test of each type, and intended differences are in `KNOWN.md`. `lib.mjs` holds the pele bundle loader and the model summaries both share.
- `fuzz/` — cargo-fuzz targets (nightly, outside the workspace).
- `.github/workflows/ci.yml` — fmt, clippy, tests on stable and 1.85; fuzz smoke run. `oracle.yml` runs `diff.mjs` nightly and on demand.

Update this file as real architecture lands.

## Commands

```sh
cargo build
cargo test                       # all tests
cargo test <name_substring>      # single test / filter
cargo test -- --nocapture        # show println! output
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cd fuzz && cargo +nightly fuzz run render -- -max_total_time=60   # also: pie, flowchart, sequence, state, class, er, requirement, timeline, journey, kanban, mindmap, gantt
cargo xtask compat pie --pele ../pele   # regenerate spec cases from a pele checkout
node tools/oracle/golden.mjs ../pele sequence   # regenerate expected.rs (flowchart, sequence, state, class, er, requirement, timeline, journey, kanban, mindmap, gantt)
node tools/oracle/diff.mjs ../pele              # differential run against Mermaid's parsers
```

Rust edition is 2024.

## Language and documentation rules

This is an open-source project for a worldwide audience.

- Code, comments, `CLAUDE.md`, commit messages, issues/PR text, and `README.md` are written in **English**.
- Translated READMEs live alongside it: `README.zh-TW.md` (Traditional Chinese), `README.zh-CN.md` (Simplified Chinese), `README.ja.md` (Japanese). When `README.md` changes in substance, update all three translations in the same change and keep the language-switcher line at the top of each file in sync.
