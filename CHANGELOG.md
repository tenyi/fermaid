# Changelog

All notable changes are recorded here. fermaid follows [Semantic Versioning](https://semver.org);
before 1.0, a minor version may break the API.

## Unreleased

- Sequence diagrams, behind the `sequence` feature: participants and actors of every kind,
  every arrow kind, activations, notes, `loop`, `alt`, `opt`, `par`, `critical`, `break` and
  `rect` blocks, boxes, `autonumber`, created and destroyed participants, and the `sequence`
  config keys. Participant menus are parsed but never drawn as links.

## 0.1.0 (unreleased)

- Flowcharts, behind the `flowchart` feature: every node shape, including `@{ shape: … }`
  metadata, all edge kinds and lengths, edge ids, subgraphs with their own direction,
  `classDef`, `class`, `style`, `linkStyle`, `click` links and tooltips, markdown labels, and
  the `flowchart` config keys `curve`, `nodeSpacing`, `rankSpacing` and `wrappingWidth`. The
  layered and compound layouts are ported from pele.
- `RenderOptions::link_schemes`, `image_schemes` and `link_rel`: links from diagram text are
  sanitized and kept only when relative or of an allowed scheme.
- `RenderOptions::auto_direction` and `direction_breakpoint`: a diagram that runs across turns
  to run down when it does not fit `max_width`.
- Style declarations from diagram text are limited to presentation properties; values that
  could load or run anything are dropped.

## 0.0.1 (unreleased)

The first release, which reserves the name on crates.io.

- Pie charts: `showData`, `title`, `accTitle`, `accDescr`, and the `pie` config keys
  `textPosition`, `donutHole` and `legendPosition`, behind the `pie` feature.
- `render` and `detect_type`, with `Error` codes, line, column and snippet, and `Limits` on the
  source, the output, edges and nesting depth.
- Preprocessing as in Mermaid: `%%` comments, front matter (`title`, `config`) and `%%{init}%%`.
- `RenderOptions`, including `max_width`: a pie chart moves its legend below the chart and
  shrinks to fit the width the host has.
- Themeable SVG through `--fermaid-*` CSS variables, including `--fermaid-series-1` to `-8`.
- Text measurement (`TextMeasurer`, `MetricsMeasurer`) and label wrapping ported from pele.
- The `fermaid` command line tool: `fermaid [INPUT|-] [-o OUTPUT] [--font-size N]
  [--max-width N] [--id-prefix PREFIX]`.
