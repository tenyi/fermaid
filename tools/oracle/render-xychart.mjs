// Dev-only: draws the same XY charts with pele and with fermaid, and compares the two
// pictures. The model of a chart is compared by tools/oracle/diff.mjs; this compares what is
// drawn, which is the layout port.
//
//   node tools/oracle/render-xychart.mjs <pele checkout>
//
// The names of the classes and of the CSS variables are fermaid's, not pele's, so they are
// normalized away before the two are compared. The checkout needs `npm ci`.
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadPele } from './lib.mjs';

const [peleDir] = process.argv.slice(2);
if (!peleDir) {
  console.error('usage: node tools/oracle/render-xychart.mjs <pele checkout>');
  process.exit(2);
}
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const { XyChartDb, parseXyChart, renderXyChart } = await loadPele(resolve(peleDir), {
  'src/diagrams/xychart/db.ts': ['XyChartDb'],
  'src/diagrams/xychart/parser.ts': ['parseXyChart'],
  'src/diagrams/xychart/render.ts': ['renderXyChart'],
});

// The charts, and the configuration each of them is drawn with.
const CASES = {
  full: [
    `xychart-beta
  title "An Example Chart"
  x-axis ["90d", "60d", "30d", "7d", "1d", "Current"]
  y-axis "Seconds" 0 --> 198.2
  line "avg" [48.1, 41.5, 45.7, 72.8, 67.7, 59.9]
  line "p50" [38.2, 36.8, 39.7, 54.5, 49.0, 38.4]
  line "p95" [112.2, 75.3, 103.0, 177.0, 180.2, 109.4]
`,
    {},
  ],
  bars: [
    `xychart
  x-axis [Q1, Q2, Q3, Q4]
  y-axis "Revenue" 0 --> 100
  bar [12, 2, 20, 25]
`,
    {},
  ],
  horizontal: [`xychart horizontal\nx-axis [a, b]\ny-axis "v" 0 --> 10\nline [1, 2]\n`, {}],
  values: [
    `xychart
  x-axis [Q1, Q2, Q3, Q4]
  y-axis "Revenue" 0 --> 100
  bar [12, 2, 20, 25]
`,
    { xyChart: { showDataLabel: true } },
  ],
  points: [
    `xychart
  x-axis [Q1, Q2, Q3]
  y-axis "v" 0 --> 10
  line [1 "first", 5 "second", 9]
`,
    {},
  ],
  mixed: [
    `xychart
  title "Mixed"
  x-axis [a, b, c, d]
  y-axis "v" 0 --> 40
  bar [10, 20, 30, 40]
  line [5, 15, 25, 35]
`,
    {},
  ],
  range: [`xychart\n  x-axis 0 --> 10\n  y-axis 0 --> 100\n  line [10, 50, 100]\n`, {}],
  narrow: [
    `xychart
  x-axis [a very long category name, another long one, and a third]
  y-axis "a long value axis title" 0 --> 1000
  line [10, 500, 1000]
`,
    {},
  ],
};

// The names pele gives its classes and its CSS variables, and the ones fermaid gives them.
const NAMES = [
  ['pele-series pele-bars', 'fermaid-xychart-series fermaid-xychart-bars'],
  ['pele-series pele-line', 'fermaid-xychart-series fermaid-xychart-line'],
  ['pele-legend-item', 'fermaid-xychart-legend-item'],
  ['pele-legend', 'fermaid-xychart-legend'],
  ['pele-axis-labels', 'fermaid-xychart-axis-labels'],
  ['pele-axis-titles', 'fermaid-xychart-axis-titles'],
  ['pele-data-labels', 'fermaid-xychart-data-labels'],
  ['pele-point-labels', 'fermaid-xychart-point-labels'],
  ['pele-axis', 'fermaid-xychart-axis'],
  ['pele-grid', 'fermaid-xychart-grid'],
  ['pele-plot', 'fermaid-xychart-plot'],
  ['pele-title', 'fermaid-title'],
];

function normalize(svg) {
  let out = svg;
  for (const [a, b] of NAMES) {
    out = out.split(`"${a}"`).join(`"${b}"`).split(` ${a}`).join(` ${b}`);
  }
  out = out.split('--pele-').join('--fermaid-');
  out = out.replace(/var\(--fermaid-series-\d+/g, 'var(--fermaid-series-N');
  out = out.replace('class="pele pele-xychart"', 'class="fermaid fermaid-xychart"');
  out = out.replace(/id="pele-/g, 'id="fermaid-');
  out = out.replace(/aria-(labelledby|describedby)="pele-/g, 'aria-$1="fermaid-');
  return out;
}

// The configuration of a chart, as the front matter fermaid reads it.
function frontMatter(config) {
  const sections = Object.entries(config);
  if (!sections.length) return '';
  const lines = [];
  for (const [section, values] of sections) {
    lines.push(`  ${section}:`);
    for (const [key, value] of Object.entries(values)) {
      lines.push(`    ${key}: ${value}`);
    }
  }
  return `---\nconfig:\n${lines.join('\n')}\n---\n`;
}

const work = mkdtempSync(join(tmpdir(), 'fermaid-render-'));
let differ = 0;
for (const [name, [src, config]] of Object.entries(CASES)) {
  const db = new XyChartDb();
  parseXyChart(src, db);
  const pele = normalize(renderXyChart(db, config, {}).svg);

  const input = join(work, `${name}.mmd`);
  const output = join(work, `${name}.svg`);
  writeFileSync(input, frontMatter(config) + src);
  execFileSync('cargo', ['run', '-q', '-p', 'fermaid-cli', '--', input, '-o', output], {
    cwd: root,
    stdio: ['ignore', 'ignore', 'inherit'],
  });
  const fermaid = normalize(readFileSync(output, 'utf8'));

  if (pele === fermaid) {
    console.log(`${name}: the same picture`);
    continue;
  }
  differ++;
  let at = 0;
  while (at < pele.length && at < fermaid.length && pele[at] === fermaid[at]) at++;
  console.log(`${name}: different at ${at}`);
  console.log(`  pele:    ${pele.slice(Math.max(0, at - 60), at + 120)}`);
  console.log(`  fermaid: ${fermaid.slice(Math.max(0, at - 60), at + 120)}`);
}
process.exit(differ ? 1 : 0);
