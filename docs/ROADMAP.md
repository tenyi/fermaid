# fermaid Roadmap

fermaid renders Mermaid diagram text to SVG in pure Rust. Behavior, layout
algorithms, text metrics and limits are ported from
[obsidianmd/pele](https://github.com/obsidianmd/pele), pinned at
**v1.9.0 (`e7f092a4de147cc32210913c20e36eaa5fcd0ef1`)**. Mermaid syntax target: 12.1.0.

Milestones M0–M5 are planned to task level and done. M6 is scoped only and will be refined
next.

## Decisions

| Topic | Decision |
|---|---|
| Deliverables | Library crate + thin CLI. Core does no I/O, so a WASM build stays possible later. |
| Relationship to pele | Port algorithms and data faithfully (layered layout, metrics, wrapping, limits), written as idiomatic Rust. Not a line-by-line translation. Output should be close to pele's, not byte-identical. Credit pele in `NOTICE` and README. |
| Runtime dependencies | None. Dev-dependencies (insta, proptest, cargo-fuzz) are allowed. |
| Errors | Strict: `Err(Error)`, no partial SVG. No lenient mode for now. |
| Platform | `std`, `rust-version = "1.85"`, edition 2024. |
| Diagram selection | One Cargo feature per diagram type, `default = ["all"]`. The `Diagram` trait stays internal; there is no runtime `register()`. |
| Links | Click targets are sanitized as `@braintree/sanitize-url` does (as Mermaid does), then kept only when relative or of a scheme in `RenderOptions::link_schemes` (default `http`, `https`, `mailto`, `tel`). A refused link is left out, not written as `about:blank`. |
| Theming | CSS variables `--fermaid-bg`, `--fermaid-fg`, `--fermaid-line`, `--fermaid-accent`, `--fermaid-series-1..8`, … (same suffixes as pele). |
| Limits | Same defaults as pele: source 50,000 chars, output 4,000,000 chars, 5,000 edges, plus an explicit nesting depth limit (default 64), because a Rust stack overflow aborts and cannot be caught. Diagram config can never raise limits. |
| Determinism | Same input + same options ⇒ byte-identical output. Coordinates use at most 2 decimals with trailing zeros trimmed. Ids are `{id_prefix}-{n}`. Output never depends on hash-map iteration order. |
| Text measurement | `TextMeasurer` trait; the default `MetricsMeasurer` ports pele's Arial ASCII tables (regular/bold) and its non-ASCII rules (CJK/fullwidth 1 em, other non-ASCII 580, mono 600, combining/zero-width 0). Wrapping rules are ported too. The tables are embedded in pele's packed form and decoded at compile time (`const fn`), so no generator is needed. |
| Dates | Gantt dates are read and drawn in UTC: there is no time zone database, and output must be deterministic. "Today" is `RenderOptions::now` (milliseconds since the epoch, CLI `--now`); `None` is 1970-01-01, not the clock. A date no format matches falls back to `new Date(text)`, which V8 reads with a parser of its own, ported in `gantt/legacy_date.rs`; the dates it reads are in `gantt/date_cases.rs`, and `tools/oracle/dates.mjs` writes them. |
| Preprocessing | `%%` comments, a front-matter YAML subset (`title`, nested `config` maps and scalars), and `%%{init}%%` parsed as strict JSON, like `JSON.parse` in pele (not JSON5). Unknown keys are ignored. |
| Testing | insta snapshots; Mermaid spec cases ported from pele's `tests/compat/<type>`, each with a `SKIPPED.md`; per-type security tests; cargo-fuzz (no panics, limits hold); differential fuzzing against Mermaid's parsers from M3 (dev-only Node harness, outside regular CI). |
| CI | GitHub Actions: `fmt --check`, `clippy -D warnings`, `test` on stable and MSRV, 60-second fuzz smoke run per target. |
| Releases | `0.2.0` is the first published release, after M5; it reserves the name on crates.io. Breaking changes are allowed before 1.0 and recorded in `CHANGELOG.md`. |

## Public API (initial shape)

```rust
pub fn render(src: &str, opts: &RenderOptions) -> Result<Rendered, Error>;
pub fn detect_type(src: &str) -> Option<DiagramKind>;

pub struct Rendered {
    pub svg: String,
    pub width: f64,
    pub height: f64,
    pub links: Vec<Link>,
    pub kind: DiagramKind,
}

pub struct Link {
    pub id: String,     // element id as written in the diagram
    pub href: String,
    pub internal: bool, // points into the page
}

pub struct Error {
    pub code: ErrorCode, // Unsupported | Syntax | Semantic | Limit
    pub kind: Option<DiagramKind>, // None when the type was not detected
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub snippet: String, // source line with a caret under the error position
}

// RenderOptions: Default; fields font_family, font_family_mono,
// font_size: Option<f64>, id_prefix, max_width: Option<f64> (M1), padding: Option<f64>, responsive,
// limits { source, output, max_edges, max_depth },
// measurer: Option<Box<dyn TextMeasurer>>.
// link_schemes is added in M2 together with URL sanitizing.
```

## Repository layout

```
crates/fermaid       library (published as `fermaid`)
crates/fermaid-cli   binary `fermaid`, hand-rolled arg parsing
xtask/               (M1) compat case conversion
fuzz/                cargo-fuzz targets
tools/oracle/        (M3) dev-only Node harness for differential fuzzing
```

## M0 — Foundation (done)

1. [x] Convert to a workspace with the layout above; set `rust-version = "1.85"`.
2. [x] Core skeleton: `Error`, `ErrorCode`, `RenderOptions`, `Limits`, `Rendered`,
   `DiagramKind`, `detect_type`, an internal `Diagram` trait, and feature-gated dispatch.
3. [x] `svg` module: zero-dependency builder with escaping, deterministic number
   formatting and sequential ids.
4. [x] `text` module: `TextMeasurer` trait and `MetricsMeasurer` with pele's tables
   embedded and decoded at compile time; port the wrapping rules (label layout).
5. [x] `preprocess` module: comments, front-matter YAML subset, `%%{init}%%`, entity codes.
6. [x] CI workflow; a `render` fuzz target.
7. [x] `NOTICE` crediting pele; a project introduction in `README.md` and its three translations.

**Done when**
- CI is green.
- `fermaid` returns an `Unsupported` error with line and column for an unknown diagram type.
- Measurer unit tests match pele's `metricsMeasurer` on a fixed set of cases.

## M1 — Pie (end-to-end pipeline) (done)

0. [x] `xtask`: convert pele's `tests/compat/<type>` cases into Rust test data
   (`cargo xtask compat <type> --pele <DIR>` writes `crates/fermaid/tests/compat/<type>/cases.rs`).
1. [x] Pie lexer, parser and model (`showData`, `title`, `accTitle`, `accDescr`), on a shared
   tokenizer for Langium-defined types (`diagrams/common/tokens.rs`).
2. [x] Layout and emitter using `--fermaid-series-*`, with `RenderOptions::max_width`
   (pele's `fitWidth`).
3. [x] Port the Mermaid spec cases from pele's `tests/compat/pie` and add `SKIPPED.md`.
4. [x] insta snapshots, security tests (escaping, limits; pie has no nesting), a fuzz target.
5. [x] First version of the CLI: `fermaid [INPUT|-] [-o OUT] [--font-size N] [--max-width N] [--id-prefix P]`.
6. Publishing is deferred to one release after M5: `0.2.0` (see "Release" below).

Notes:
- Output was compared byte for byte with pele 1.9.0 (after renaming `pele` to `fermaid`) on
  pele's pie corpus, its hostile payloads and 3,000 mutated inputs: identical, errors and their
  positions included.
- 10 minutes of fuzzing the `pie` target (1.79 million runs, seeded with the pie corpus) found
  no panic.
- As in pele, pie errors are positioned in the text after preprocessing (comment lines and
  front matter removed), so a line number can be off when comments precede the error.

**Done when**
- Compat tests pass; every skipped case is listed in `SKIPPED.md`.
- 10 minutes of fuzzing produces no panic.
- The SVG renders in a browser and its theme can be changed through CSS.

## M2 — Flowchart (core layout) (done)

1. [x] `layout::layered`: port pele's Sugiyama layout (ranking, crossing
   reduction, dummy/label nodes, edge tracks, straighten/settle/snap).
2. [x] `layout::compound` for subgraphs. (pele's `components` is used only by other diagram
   types; it moves to the milestone of the first type that needs it.)
3. [x] Flowchart parser covering all node shapes, edge kinds, `classDef`, `style`,
   `click` and `linkStyle`. The lexer works on UTF-16 code units, as Mermaid's jison lexer does.
4. [x] Port svg shapes, edges and clusters.
5. [x] Security: URL sanitizing and a link scheme allowlist (`RenderOptions::link_schemes`,
   `image_schemes`, `link_rel`); `style` limited to allowed properties.
6. [x] Compat, snapshot and security tests; a fuzz target.
7. Publishing is deferred to one release after M5: `0.2.0` (see "Release" below).

Notes:
- Compat: 301 cases from pele's port of the Mermaid flowchart specs. `tools/oracle/golden.mjs`
  records what pele makes of each, and fermaid's model matches it on all 274 that are not
  skipped (27 are built from test-code values; see `SKIPPED.md`).
- All 114 flowchart examples from the Mermaid docs render. On pele's flowchart corpus (379
  inputs), the inputs in pele's own flowchart tests (102) and 2,000 mutated inputs, the output
  is byte for byte pele's, errors included, except where a node has an icon: with no icon
  resolver, fermaid draws a dashed placeholder where pele leaves the space empty. This also
  settles the side-by-side review.
- A random graph of 5,000 edges renders in about 0.1 s in a release build (pele: 0.4 s).
- 10 minutes of fuzzing the `flowchart` target (625,000 runs, seeded with pele's flowchart
  corpus) found no panic.

**Done when**
- Compat tests pass.
- Every flowchart example from the Mermaid docs (pele's corpus) renders.
- A side-by-side manual review against pele on 20 fixed examples shows comparable layouts.
- A 5,000-edge stress test finishes in under 1 s in a release build
  (provisional target; adjust once a baseline exists).

## M3 — Sequence + differential fuzzing (done)

1. [x] Sequence diagrams: participants, actors, messages, activations,
   `loop`/`alt`/`opt`/`par`, notes, `autonumber`.
2. [x] `tools/oracle/`: dev-only Node harness that loads Mermaid's jison and Langium
   parsers and runs differential fuzzing (about 3,000 mutated inputs, fixed seed)
   for pie, flowchart and sequence.
3. [x] Refine the M4 plan.

Notes:
- Sequence diagrams cover everything pele draws: every participant kind (including
  `@{ "type": … }`), every arrow kind with half heads and central connections, activations,
  notes, all block kinds, boxes, `autonumber`, created and destroyed participants, and the
  `sequence` config keys `mirrorActors`, `rightAngles`, `actorMargin`, `width`, `height`,
  `messageMargin`, `noteMargin`, `boxMargin`, `messageAlign`, `showSequenceNumbers`,
  `hideUnusedParticipants` and `wrap`. As in pele, participant menus (`link`, `links`) are
  parsed but never drawn as links.
- Compat: 165 cases from pele's port of the Mermaid sequence specs; fermaid's model matches
  pele's on all of them.
- On pele's sequence corpus (200 inputs), with and without `max_width` and a non-default
  `sequence` config, and on 3,000 mutated inputs, the output is byte for byte pele's, errors
  included.
- 7 minutes of fuzzing the `sequence` target (680,000 runs, seeded with pele's sequence corpus)
  found no panic.
- `tools/oracle/diff.mjs` parses each type's corpus and 3,000 mutated inputs with Mermaid's own
  parsers (`@mermaid-js/parser` for pie; the parsers jison generates from Mermaid's flowchart
  and sequence grammars, driving pele's port of Mermaid's model) and with fermaid's, and
  compares the results. With seeds 1, 7, 42 and 1234 (5,000 inputs each), the three types
  agree on every input. A fourth check, `url`, compares the link sanitizer with
  `@braintree/sanitize-url`; it found that fermaid's imitation of `new URL()` let through
  hosts that `new URL()` refuses, and missed percent-encoding and dot segments, which are now
  fixed. The differences that remain are in `tools/oracle/KNOWN.md`: one comes from Node's
  URL parser departing from the WHATWG standard, the other from IDNA tables fermaid does not
  carry. The workflow `.github/workflows/oracle.yml` runs it nightly and on demand.

**Done when**
- Differential fuzzing reports no unexplained differences for the three types;
  intentional differences are recorded in `SKIPPED.md`.
- Regular CI does not need Node; the oracle runs only in a nightly or
  manually triggered workflow.

## M4 — State, class, ER, requirement ✅

Each type is ported from pele as flowchart and sequence were: parser, model, render, and a
check against pele's output on its corpus and on mutated inputs.

1. State diagrams: states, composite states with their own direction, start and end, choice,
   fork and join, notes, concurrency regions, `classDef` and `class`, `direction`. Reuses
   the compound layout.
2. Class diagrams: classes with members and generics, relations with cardinality, namespaces,
   annotations, notes, `classDef`, `style`, `cssClass`, links and tooltips (sanitized as in
   flowcharts). Reuses the layered layout.
3. ER diagrams: entities with attributes and keys, relationships with cardinality and
   identification, aliases, `classDef` and `style`.
4. Requirement diagrams: requirements, elements and relationships, `classDef` and `style`.
5. For each: compat cases from pele's `tests/compat/<type>` with `SKIPPED.md` (golden where the
   specs test a model), snapshot, security and speed tests, a fuzz target, and a place in
   `tools/oracle/diff.mjs` (state, class and ER have jison grammars in pele; requirement too).
6. Port pele's `components` layout only if one of these needs it.

**Done when**
- Compat tests pass, and each type's output matches pele's on its corpus and on mutated
  inputs, apart from recorded differences.
- The differential run reports no unexplained differences for the new types.
- 10 minutes of fuzzing each new target produces no panic.

Notes:
- All four types are ported. Class diagrams needed pele's `components` layout, which wraps
  disconnected parts into rows to fit a narrow host; it is in `layout/components.rs`.
- On each type's corpus from pele, on 3,000 mutated inputs, and with `max_width`, the output
  is byte for byte pele's, errors included.
- Compat cases from pele's ports of the Mermaid specs pass for all four types. The ER and
  requirement specs partly call the model directly or build diagrams from arrays of lines, so
  many of their extracted sources are not diagrams and both sides reject them; `SKIPPED.md`
  says so, and the differential run covers the parsers instead.
- The differential run against Mermaid's jison grammars (state, class, ER, requirement) agrees
  on every input.
- Fuzzing found no panic (class 831,000 runs, ER 895,000 runs in 10 minutes).

## M5 — Gantt, timeline, journey, kanban, mindmap ✅

Each type is ported from pele as in M4, with compat cases, snapshot, security and speed tests,
and a fuzz target.

Notes:
- Timelines and journeys share a plain line parser (`common/plain.rs`); kanban boards and mind
  maps share an indented outline parser (`common/outline.rs`); all four use the tokenizer in
  `common/scan.rs`.
- On each type's corpus from pele, on hand-written cases, on 3,000 mutated inputs, and with
  `max_width`, the output is byte for byte pele's, errors included. For gantt charts pele runs
  with `TZ=UTC` and `now: 0` (and other fixed times). The dates V8's own date parser reads are
  the ones Mermaid reads, so nothing is left out.
- Compat cases pass. The gantt and mind map specs partly call the model directly; those cases
  are listed in `SKIPPED.md`.
- The differential run against Mermaid's grammars covers all five types, and every one of
  them agrees on every input.
- Fuzzing found no panic.
- Re-verified against pele 1.9.0 (`e7f092a`) and Node 22: the golden files of the five types
  regenerate byte for byte, the differential run over 3,000 mutated inputs per type reports no
  difference for any of the thirteen types, and ten minutes of fuzzing per new target found no
  panic (gantt 757,835 runs, timeline 233,850, journey 293,981, kanban 279,941, mind map
  311,144).

## Release — 0.2.0

The first published version. The crates.io names `fermaid` and `fermaid-cli` were free, and
`0.0.1` and `0.1.0` were never published, so `0.2.0` is the only version crates.io shows.

1. [ ] Publish `fermaid` 0.2.0 to crates.io, then `fermaid-cli` 0.2.0, which depends on it
   (needs the owner's crates.io token).
2. [ ] Tag `v0.2.0`, push the tag and publish the GitHub release.

## Later milestones (scope only)

- **M6** — remaining types by usage; niche types (agentflow, wardley, …) last.
