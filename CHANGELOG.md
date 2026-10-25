# Changelog

All notable changes are recorded here. fermaid follows [Semantic Versioning](https://semver.org);
before 1.0, a minor version may break the API.

## Unreleased

- Architecture diagrams, behind the `architecture` feature: services in groups, the edges
  between them with their labels, junctions, `service`, `group` and `junction` with their icons,
  and the `architecture` config keys `iconSize`, `padding` and `fontSize`. A service has no icon
  font, so its box holds its name.
- Packet diagrams, behind the `packet` feature: `title`, `accTitle` and `accDescr`, rows of
  fields with their bit ranges, a field of `+bits`, and the `packet` config keys `rowHeight`,
  `bitWidth`, `bitsPerRow`, `showBits`, `bitOrder`, `paddingX` and `paddingY`.
- Radar charts, behind the `radar` feature: `title`, `accTitle` and `accDescr`, `axis` and
  `curve` with their entries, and the options `showLegend`, `ticks`, `min`, `max` and
  `graticule`; and the `radar` config keys `width`, `height`, `marginTop`, `marginRight`,
  `marginBottom`, `marginLeft`, `axisScaleFactor`, `axisLabelFactor` and `curveTension`.
- Treemaps, behind the `treemap` feature: a tree of sections and leaves with their values, and
  `classDef` and `class`.
- Block diagrams, behind the `block` feature: blocks in a grid with `columns`, composites with
  `block:` and `end`, `space`, every shape of Mermaid, block arrows with their directions, links
  with their labels and their arrow ends, `classDef`, `class` and `style`, and the `block` config
  key `padding`.
- Sankey charts, behind the `sankey` feature: the CSV form of Mermaid, a `source,target,value`
  record per line, with the fields quoted as RFC 4180 says; and the `sankey` config keys
  `width`, `height`, `nodeWidth`, `nodePadding`, `nodeAlignment`, `nodeColors`, `linkColor`,
  `showValues`, `prefix`, `suffix` and `labelStyle`.
- XY charts, behind the `xychart` feature: `title`, `x-axis` with categories or a range,
  `y-axis` with a range, `line` and `bar` series with point labels, `accTitle` and `accDescr`,
  the `horizontal` and `vertical` orientation, and the `xyChart` config keys `width`, `height`,
  `chartOrientation`, `showTitle`, `showLegend` and `showDataLabel`, with the `showTitle`,
  `showTick`, `showLabel` and `showAxisLine` keys of `xAxis` and `yAxis`.
- Quadrant charts, behind the `quadrantchart` feature: `title`, `x-axis` and `y-axis` with a
  `-->`, `quadrant-1` to `quadrant-4`, points with `radius`, `color`, `stroke-color` and
  `stroke-width`, `classDef` and the classes of a point, `accTitle` and `accDescr`, and the
  `quadrantChart` config keys `chartWidth`, `chartHeight`, `titlePadding`,
  `xAxisLabelPadding`, `yAxisLabelPadding`, `xAxisPosition`, `yAxisPosition`,
  `quadrantPadding`, `quadrantTextTopPadding`, `pointRadius` and `pointTextPadding`.
- Git graphs, behind the `git` feature: `title`, `accTitle` and `accDescr`, `commit` with an
  `id`, a `msg`, `tag`s and a `type` of `NORMAL`, `REVERSE` or `HIGHLIGHT`, `branch` with an
  `order`, `checkout` and `switch`, `merge` and `cherry-pick` with a `parent`, the `LR`, `TB`
  and `BT` directions, and the `gitGraph` config keys `mainBranchName`, `mainBranchOrder`,
  `showCommitLabel`, `showBranches`, `rotateCommitLabel` and `parallelCommits`.
- C4 diagrams, behind the `c4` feature: `title`, `accTitle` and `accDescr`, the `C4Context`,
  `C4Container`, `C4Component`, `C4Dynamic` and `C4Deployment` statements, persons, systems,
  containers and components with their boundaries and their relationships, and the
  `UpdateLayoutConfig` statement.
- Use case diagrams, behind the `usecase` feature: `title`, `accTitle` and `accDescr`, actors and
  use cases with the relations between them, boundaries, notes, the JSON tables, `classDef` and
  `class`; and the `usecase` config keys `nodeSpacing` and `rankSpacing`.
- Information diagrams, behind the `info` feature: `title`, `accTitle` and `accDescr`, the
  version of the library and the lines of text.
- Venn diagrams, behind the `venn` feature: the circles with their labels; and the `venn` config
  keys `width`, `height` and `padding`.
- Tree view diagrams, behind the `treeview` feature: the tree drawn with box-drawing
  characters, the icons in front of a label, `classDef` and `class`; and the `treeView` config
  keys `rowIndent`, `paddingX`, `paddingY`, `lineThickness`, `showIcons`, `defaultIconPack`,
  `filenameIcons` and `extensionIcons`.
- Cynefin diagrams, behind the `cynefin` feature: `title`, `accTitle` and `accDescr`, the items
  in the four regions and the transitions between them; and the `cynefin` config keys `width`,
  `height`, `showDomainDescriptions`, `boundaryAmplitude` and `seed`.
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
