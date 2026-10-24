# Changelog

All notable changes are recorded here. fermaid follows [Semantic Versioning](https://semver.org);
before 1.0, a minor version may break the API.

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
