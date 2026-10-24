# Changelog

All notable changes are recorded here. fermaid follows [Semantic Versioning](https://semver.org);
before 1.0, a minor version may break the API.

## Unreleased

## 0.2.0 - 2026-10-24

The first published release. It covers milestones M0 to M5: the foundation, pie charts,
flowcharts, sequence diagrams, the Mermaid grammar types, and the M5 chart types. Every
diagram type is behind its own Cargo feature, and the default feature `all` includes them
all. The library has no runtime dependencies.

### Diagram types

- Pie charts, behind the `pie` feature: `showData`, `title`, `accTitle`, `accDescr`, and the
  `pie` config keys `textPosition`, `donutHole` and `legendPosition`.
- Flowcharts, behind the `flowchart` feature: every node shape, including `@{ shape: … }`
  metadata, all edge kinds and lengths, edge ids, subgraphs with their own direction,
  `classDef`, `class`, `style`, `linkStyle`, `click` links and tooltips, markdown labels, and
  the `flowchart` config keys `curve`, `nodeSpacing`, `rankSpacing` and `wrappingWidth`. The
  layered and compound layouts are ported from pele.
- Sequence diagrams, behind the `sequence` feature: participants and actors of every kind,
  every arrow kind, activations, notes, `loop`, `alt`, `opt`, `par`, `critical`, `break` and
  `rect` blocks, boxes, `autonumber`, created and destroyed participants, and the `sequence`
  config keys. Participant menus are parsed but never drawn as links.
- State diagrams, behind the `state` feature: states with descriptions, composite states with
  their own direction, concurrency regions, start and end, choice, fork and join, notes,
  `classDef`, `class`, `style` and `click`.
- Class diagrams, behind the `class` feature: classes with members, generic types and
  annotations, relations with markers and cardinalities, namespaces, notes, lollipop
  interfaces, `classDef`, `cssClass`, `style`, `link`, `click` and `callback`.
- ER diagrams, behind the `er` feature: entities with attributes, keys and comments,
  relationships with crow's foot cardinalities, aliases, subgraphs, `classDef`, `class` and
  `style`.
- Requirement diagrams, behind the `requirement` feature: requirements of every kind and
  elements with their fields, relationships, `classDef`, `class` and `style`.
- Gantt charts, behind the `gantt` feature: tasks with dates, durations, `after` and `until`,
  sections, `done`, `active`, `crit`, `milestone` and `vert` tasks, excluded and included
  days, `dateFormat`, `axisFormat`, `tickInterval`, `weekday`, `todayMarker`, `topAxis`,
  compact mode and `click` links. Dates are UTC.
- Timelines (`timeline`), user journeys (`journey`), kanban boards (`kanban`, with ticket links
  through `ticketBaseUrl`) and mind maps (`mindmap`), each behind its own feature.

### Library

- `render` and `detect_type`, with `Error` codes, line, column and snippet, and `Limits` on the
  source, the output, edges and nesting depth.
- Preprocessing as in Mermaid: `%%` comments, front matter (`title`, `config`) and `%%{init}%%`.
- `RenderOptions`, including `max_width`: a pie chart moves its legend below the chart and
  shrinks to fit the width the host has.
- `RenderOptions::now` and the CLI's `--now`: the time a gantt chart's today marker shows.
  The default is 1970-01-01, so output does not depend on the clock.
- `RenderOptions::link_schemes`, `image_schemes` and `link_rel`: links from diagram text are
  sanitized and kept only when relative or of an allowed scheme.
- `RenderOptions::auto_direction` and `direction_breakpoint`: a diagram that runs across turns
  to run down when it does not fit `max_width`.
- Themeable SVG through `--fermaid-*` CSS variables, including `--fermaid-series-1` to `-8`.
- Text measurement (`TextMeasurer`, `MetricsMeasurer`) and label wrapping ported from pele.
- The `fermaid` command line tool: `fermaid [INPUT|-] [-o OUTPUT] [--font-size N]
  [--max-width N] [--id-prefix PREFIX]`.

### Links and safety

- Links: `http` and `https` addresses are parsed as `new URL()` parses them, as Mermaid's
  sanitizer does: an address with a host the URL standard refuses becomes `about:blank` and is
  left out, and paths, queries and fragments are normalized and percent-encoded.
- Style declarations from diagram text are limited to presentation properties; values that
  could load or run anything are dropped.
