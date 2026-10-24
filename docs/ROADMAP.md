# fermaid Roadmap

fermaid renders Mermaid diagram text to SVG in pure Rust. Behavior, layout
algorithms, text metrics and limits are ported from
[obsidianmd/pele](https://github.com/obsidianmd/pele), pinned at
**v1.9.0 (`e7f092a4de147cc32210913c20e36eaa5fcd0ef1`)**. Mermaid syntax target: 12.1.0.

Milestones M0–M3 are planned to task level. M4 onward is scoped only and will be
refined when M3 ends.

## Decisions

| Topic | Decision |
|---|---|
| Deliverables | Library crate + thin CLI. Core does no I/O, so a WASM build stays possible later. |
| Relationship to pele | Port algorithms and data faithfully (layered layout, metrics, wrapping, limits), written as idiomatic Rust. Not a line-by-line translation. Output should be close to pele's, not byte-identical. Credit pele in `NOTICE` and README. |
| Runtime dependencies | None. Dev-dependencies (insta, proptest, cargo-fuzz) are allowed. |
| Errors | Strict: `Err(Error)`, no partial SVG. No lenient mode for now. |
| Platform | `std`, `rust-version = "1.85"`, edition 2024. |
| Diagram selection | One Cargo feature per diagram type, `default = ["all"]`. The `Diagram` trait stays internal; there is no runtime `register()`. |
| Theming | CSS variables `--fermaid-bg`, `--fermaid-fg`, `--fermaid-line`, `--fermaid-accent`, `--fermaid-series-1..8`, … (same suffixes as pele). |
| Limits | Same defaults as pele: source 50,000 chars, output 4,000,000 chars, 5,000 edges, plus an explicit nesting depth limit (default 64), because a Rust stack overflow aborts and cannot be caught. Diagram config can never raise limits. |
| Determinism | Same input + same options ⇒ byte-identical output. Coordinates use at most 2 decimals with trailing zeros trimmed. Ids are `{id_prefix}-{n}`. Output never depends on hash-map iteration order. |
| Text measurement | `TextMeasurer` trait; the default `MetricsMeasurer` ports pele's Arial ASCII tables (regular/bold) and its non-ASCII rules (CJK/fullwidth 1 em, other non-ASCII 580, mono 600, combining/zero-width 0). Wrapping rules are ported too. |
| Preprocessing | `%%` comments, a front-matter YAML subset (`title`, nested `config` maps and scalars), and `%%{init}%%` parsed as a JSON5 subset. Unknown keys are ignored. |
| Testing | insta snapshots; Mermaid spec cases ported from pele's `tests/compat/<type>`, each with a `SKIPPED.md`; per-type security tests; cargo-fuzz (no panics, limits hold); differential fuzzing against Mermaid's parsers from M3 (dev-only Node harness, outside regular CI). |
| CI | GitHub Actions: `fmt --check`, `clippy -D warnings`, `test` on stable and MSRV, 60-second fuzz smoke run per target. |
| Releases | `0.0.1` after M1 (reserves the name), `0.1.0` after M2. Breaking changes are allowed before 1.0 and recorded in `CHANGELOG.md`. |

## Public API (initial shape)

```rust
pub fn render(src: &str, opts: &RenderOptions) -> Result<Rendered, Error>;
pub fn detect_type(src: &str) -> Option<DiagramKind>;

pub struct Rendered {
    pub svg: String,
    pub width: f64,
    pub height: f64,
    pub links: Vec<String>,
    pub kind: DiagramKind,
}

pub struct Error {
    pub code: ErrorCode, // Unsupported | Syntax | Semantic | Limit
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub snippet: String, // source line with a caret under the error position
}

// RenderOptions: Default; fields font_family, font_size, id_prefix, padding,
// limits { source, output, max_edges, max_depth }, link_schemes,
// measurer: Option<Box<dyn TextMeasurer>>
```

## Repository layout

```
crates/fermaid       library (published as `fermaid`)
crates/fermaid-cli   binary `fermaid`, hand-rolled arg parsing
xtask/               metrics table generation, compat case conversion
fuzz/                cargo-fuzz targets
tools/oracle/        (M3) dev-only Node harness for differential fuzzing
```

## M0 — Foundation

1. Convert to a workspace with the layout above; set `rust-version = "1.85"`.
2. Core skeleton: `Error`, `ErrorCode`, `RenderOptions`, `Limits`, `Rendered`,
   `DiagramKind`, `detect_type`, an internal `Diagram` trait, and feature-gated dispatch.
3. `svg` module: zero-dependency builder with escaping, deterministic number
   formatting and sequential ids.
4. `text` module: `TextMeasurer` trait and `MetricsMeasurer`; xtask generates
   the metrics tables from pele; port the wrapping rules.
5. `preprocess` module: comments, front-matter YAML subset, `%%{init}%%`.
6. CI workflow.
7. `NOTICE` crediting pele; a project introduction in `README.md` and its three translations.

**Done when**
- CI is green.
- `fermaid` returns an `Unsupported` error with line and column for an unknown diagram type.
- Measurer unit tests match pele's `metricsMeasurer` on a fixed set of cases.

## M1 — Pie (end-to-end pipeline)

1. Pie lexer, parser and model (`showData`, `title`, `accTitle`, `accDescr`).
2. Layout and emitter using `--fermaid-series-*`.
3. Port the Mermaid spec cases from pele's `tests/compat/pie` and add `SKIPPED.md`.
4. insta snapshots, security tests (escaping, limits, depth), a fuzz target.
5. First version of the CLI: `fermaid [INPUT|-] [-o OUT] [--font-size N] [--id-prefix P]`.
6. Publish `0.0.1`.

**Done when**
- Compat tests pass; every skipped case is listed in `SKIPPED.md`.
- 10 minutes of fuzzing produces no panic.
- The SVG renders in a browser and its theme can be changed through CSS.

## M2 — Flowchart (core layout)

1. `layout::layered`: port pele's Sugiyama layout (ranking, crossing
   reduction, dummy/label nodes, edge tracks, straighten/settle/snap).
2. `layout::compound` for subgraphs; `components` for disconnected subgraphs.
3. Flowchart parser covering all node shapes, edge kinds, `classDef`, `style`,
   `click` and `linkStyle`.
4. Port svg shapes, edges and clusters.
5. Security: link scheme allowlist; `style` limited to allowed properties.
6. Compat, snapshot and security tests; a fuzz target.
7. Publish `0.1.0`.

**Done when**
- Compat tests pass.
- Every flowchart example from the Mermaid docs (pele's corpus) renders.
- A side-by-side manual review against pele on 20 fixed examples shows comparable layouts.
- A 5,000-edge stress test finishes in under 1 s in a release build
  (provisional target; adjust once a baseline exists).

## M3 — Sequence + differential fuzzing

1. Sequence diagrams: participants, actors, messages, activations,
   `loop`/`alt`/`opt`/`par`, notes, `autonumber`.
2. `tools/oracle/`: dev-only Node harness that loads Mermaid's jison and Langium
   parsers and runs differential fuzzing (about 3,000 mutated inputs, fixed seed)
   for pie, flowchart and sequence.
3. Refine the M4 plan.

**Done when**
- Differential fuzzing reports no unexplained differences for the three types;
  intentional differences are recorded in `SKIPPED.md`.
- Regular CI does not need Node; the oracle runs only in a nightly or
  manually triggered workflow.

## Later milestones (scope only)

- **M4** — state, class, er, requirement (reuse the layered layout).
- **M5** — gantt, timeline, journey, kanban, mindmap.
- **M6** — remaining types by usage; niche types (agentflow, wardley, …) last.
