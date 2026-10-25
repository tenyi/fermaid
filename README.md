# fermaid

**English** | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

[![crates.io](https://img.shields.io/crates/v/fermaid.svg)](https://crates.io/crates/fermaid)
[![docs.rs](https://img.shields.io/docsrs/fermaid.svg)](https://docs.rs/fermaid)
[![license](https://img.shields.io/crates/l/fermaid.svg)](https://crates.io/crates/fermaid)

fermaid renders [Mermaid](https://mermaid.js.org) diagrams to SVG in pure Rust — no JavaScript runtime, no headless browser, no Node.js. It is a port of [pele](https://github.com/obsidianmd/pele), a small, dependency-free TypeScript renderer, and targets Mermaid 12.1.0 syntax with pele's own layout and visual style.

> **Status:** alpha. Flowcharts, sequence, state, class, ER and requirement diagrams, pie, gantt, sankey, XY and quadrant charts, timelines, user journeys, kanban boards and mind maps are supported; every other diagram type returns an `Unsupported` error for now. See the [roadmap](docs/ROADMAP.md).

| Diagram type | Status |
|---|---|
| Pie (`pie`) | Supported: `showData`, `title`, `accTitle`, `accDescr`, and the `pie` config keys `textPosition`, `donutHole` and `legendPosition` |
| Flowchart (`flowchart`, `graph`) | Supported: every node shape, including `@{ shape: … }`, all edge kinds and lengths, edge ids, subgraphs with their own direction, `classDef`, `class`, `style`, `linkStyle`, `click` links and tooltips, markdown labels, and the `flowchart` config keys `curve`, `nodeSpacing`, `rankSpacing` and `wrappingWidth` |
| Sequence (`sequenceDiagram`) | Supported: participants and actors of every kind, every arrow kind, activations, notes, `loop`, `alt`, `opt`, `par`, `critical`, `break` and `rect` blocks, boxes, `autonumber`, created and destroyed participants, and the `sequence` config keys. Participant menus (`link`, `links`) are not drawn as links |
| State (`stateDiagram`, `stateDiagram-v2`) | Supported: states with descriptions, composite states with their own direction, concurrency regions, start and end, choice, fork and join, notes, `classDef`, `class`, `style` and `click` |
| Class (`classDiagram`) | Supported: classes with members, generic types and annotations, relations with markers and cardinalities, namespaces, notes, lollipop interfaces, `classDef`, `cssClass`, `style`, `link`, `click` and `callback` |
| ER (`erDiagram`) | Supported: entities with attributes, keys and comments, relationships with crow's foot cardinalities, aliases, subgraphs, `classDef`, `class` and `style` |
| Requirement (`requirementDiagram`) | Supported: requirements of every kind and elements with their fields, relationships, `classDef`, `class` and `style` |
| Gantt (`gantt`) | Supported: tasks with dates, durations, `after` and `until`, sections, `done`, `active`, `crit`, `milestone` and `vert`, `excludes`, `includes` and `weekend`, `dateFormat`, `axisFormat`, `tickInterval`, `weekday`, `todayMarker`, `topAxis`, compact mode and `click` links. Dates are UTC, and today is `RenderOptions::now` |
| Timeline (`timeline`) | Supported: title, sections, periods with their events |
| User journey (`journey`) | Supported: title, sections, tasks with scores and actors |
| Kanban (`kanban`) | Supported: columns and cards with shapes, `@{ … }` metadata (`assigned`, `ticket`, `priority`, `label`), classes, and ticket links through `ticketBaseUrl` |
| Mind map (`mindmap`) | Supported: every node shape, icons, classes, and branch colors; laid out across, or down for a narrow host |
| Sankey (`sankey`, `sankey-beta`) | Supported: the CSV form of Mermaid, `source,target,value` per line, with quoted fields; and the `sankey` config keys `width`, `height`, `nodeWidth`, `nodePadding`, `nodeAlignment`, `nodeColors`, `linkColor`, `showValues`, `prefix`, `suffix` and `labelStyle` |
| XY chart (`xychart`, `xychart-beta`) | Supported: `title`, `x-axis` with categories or a range, `y-axis` with a range, `line` and `bar` series with point labels, `accTitle` and `accDescr`, the `horizontal` and `vertical` orientation, and the `xyChart` config keys `width`, `height`, `chartOrientation`, `showTitle`, `showLegend`, `showDataLabel`, `xAxis.showTitle`, `xAxis.showTick`, `xAxis.showLabel`, `xAxis.showAxisLine` and the same four under `yAxis` |
| Quadrant chart (`quadrantChart`) | Supported: `title`, `x-axis` and `y-axis` with a `-->`, `quadrant-1` to `quadrant-4`, points with `radius`, `color`, `stroke-color` and `stroke-width`, `classDef` and the classes of a point, `accTitle` and `accDescr`; and the `quadrantChart` config keys `chartWidth`, `chartHeight`, `titlePadding`, `xAxisLabelPadding`, `yAxisLabelPadding`, `xAxisPosition`, `yAxisPosition`, `quadrantPadding`, `quadrantTextTopPadding`, `pointRadius` and `pointTextPadding` |
| Git graph (`gitGraph`) | Supported: `title`, `accTitle` and `accDescr`, `commit` with an `id`, a `msg`, `tag`s and a `type` of `NORMAL`, `REVERSE` or `HIGHLIGHT`, `branch` with an `order`, `checkout` and `switch`, `merge` and `cherry-pick` with a `parent`; and the `gitGraph` config keys `mainBranchName`, `mainBranchOrder`, `showCommitLabel`, `showBranches`, `rotateCommitLabel` and `parallelCommits` |
| Block (`block`, `block-beta`) | Supported: blocks in a grid with `columns`, composites with `block:` and `end`, `space`, every shape of Mermaid, block arrows with their directions, links with their labels and their arrow ends, `classDef`, `class` and `style`; and the `block` config key `padding` |
| Architecture (`architecture`, `architecture-beta`) | Supported: services in groups, with the edges between them and their labels, junctions, `service`, `group` and `junction` with their icons; and the `architecture` config keys `iconSize`, `padding` and `fontSize`. A service has no icon font, so its box holds its name |
| Packet (`packet`, `packet-beta`) | Supported: `title`, `accTitle` and `accDescr`, rows of fields with their bit ranges, a field of `+bits`; and the `packet` config keys `rowHeight`, `bitWidth`, `bitsPerRow`, `showBits`, `bitOrder`, `paddingX` and `paddingY` |
| Radar chart (`radar-beta`) | Supported: `title`, `accTitle` and `accDescr`, `axis` and `curve` with their entries, and the options `showLegend`, `ticks`, `min`, `max` and `graticule`; and the `radar` config keys `width`, `height`, `marginTop`, `marginRight`, `marginBottom`, `marginLeft`, `axisScaleFactor`, `axisLabelFactor` and `curveTension` |
| Treemap (`treemap`) | Supported: a tree of sections and leaves with their values, and `classDef` and `class` |
| C4 diagram (`C4Context`, `C4Container`, `C4Component`, `C4Dynamic`, `C4Deployment`) | Supported: `title`, `accTitle` and `accDescr`, persons, systems, containers and components with their boundaries and their relationships, and the `UpdateLayoutConfig` statement |
| Use case diagram (`usecase-beta`) | Supported: `title`, `accTitle` and `accDescr`, actors and use cases with the relations between them; and the `usecase` config keys `nodeSpacing` and `rankSpacing` |
| Information diagram (`info`) | Supported: `title`, `accTitle` and `accDescr`, the version and the lines of text |
| Venn diagram (`venn-beta`) | Supported: the circles with their labels; and the `venn` config keys `width`, `height` and `padding` |
| Tree view diagram (`treeView-beta`) | Supported: the tree drawn with box-drawing characters, the icons in front of a label, `classDef` and `class`; and the `treeView` config keys `rowIndent`, `paddingX`, `paddingY`, `lineThickness`, `showIcons`, `defaultIconPack`, `filenameIcons` and `extensionIcons` |
| Cynefin diagram (`cynefin-beta:`) | Supported: `title`, `accTitle` and `accDescr`, the items in the four regions and the transitions between them; and the `cynefin` config keys `width`, `height`, `showDomainDescriptions`, `boundaryAmplitude` and `seed` |
| Ishikawa diagram (`ishikawa`) | Supported: the causes on the levels and the spine |
| Event modeling diagram (`eventmodeling`) | Supported: the lanes, the frames, the relations and the notes |
| Wardley map (`wardley-beta`) | Supported: `title`, `accTitle` and `accDescr`, the items with their axes and their evolution; and the `wardley` config keys `width`, `height` and `showGrid` |
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

`render` returns the SVG with its size and links, or an `Error` with a code (`Unsupported`, `Syntax`, `Semantic`, `Limit`), a line and column, and a snippet that points at the problem. `detect_type` tells the diagram type without rendering. Set `max_width` to the width the host has, and a diagram that can draw itself narrower does (a pie chart moves its legend below the chart). Links from `click` statements are sanitized and kept only when their scheme is allowed (`link_schemes`, by default `http`, `https`, `mailto` and `tel`); a flowchart that runs across turns to run down when it does not fit `max_width`. Each diagram type is a Cargo feature (`flowchart`, `pie`, …); the default feature `all` includes every type. A gantt chart dates "today" from `now`, in milliseconds since 1970-01-01 UTC; it defaults to 1970-01-01 so that output never depends on the clock.

## Command line

```sh
cargo install fermaid-cli                # from crates.io
cargo install --path crates/fermaid-cli  # from a checkout
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --max-width 360 --id-prefix fig1 > diagram.svg
fermaid plan.mmd --now 1700000000000 -o plan.svg   # a gantt chart with today's marker
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
cargo +nightly fuzz run render    # needs cargo-fuzz; `pie`, `flowchart`, `sequence`, `state`, `class`, `er`, `requirement`, `timeline`, `journey`, `kanban`, `mindmap`, `gantt`, `sankey`, `xychart`, `quadrantchart` and `git` are other targets
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
