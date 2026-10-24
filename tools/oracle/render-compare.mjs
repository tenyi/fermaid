// Dev-only: draws the same charts with pele and with fermaid, and compares the two pictures.
// The model of a chart is compared by tools/oracle/diff.mjs; this compares what is drawn, which
// is the layout port.
//
//   node tools/oracle/render-compare.mjs <pele checkout> <type>
//
// The names of the classes and of the CSS variables are fermaid's, not pele's, so they are
// normalized away before the two are compared. The checkout needs `npm ci`.
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadPele } from './lib.mjs';

const [peleDir, type] = process.argv.slice(2);
if (!peleDir || !type) {
  console.error('usage: node tools/oracle/render-compare.mjs <pele checkout> <type>');
  process.exit(2);
}
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

/// The charts of each type, and the configuration each of them is drawn with.
const TYPES = {
  xychart: {
    modules: {
      'src/diagrams/xychart/db.ts': ['XyChartDb'],
      'src/diagrams/xychart/parser.ts': ['parseXyChart'],
      'src/diagrams/xychart/render.ts': ['renderXyChart'],
    },
    names: [
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
    ],
    cases: {
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
    },
  },
  git: {
    modules: {
      'src/diagrams/git/db.ts': ['buildGit'],
      'src/diagrams/git/parser.ts': ['parseGit'],
      'src/diagrams/git/render.ts': ['renderGit'],
    },
    names: [
      ['pele-branches', 'fermaid-branches'],
      ['pele-branch', 'fermaid-branch'],
      ['pele-lane', 'fermaid-lane'],
      ['pele-edges', 'fermaid-edges'],
      ['pele-edge-merge', 'fermaid-edge-merge'],
      ['pele-edge-cherry-pick', 'fermaid-edge-cherry-pick'],
      ['pele-edge', 'fermaid-edge'],
      ['pele-commits', 'fermaid-commits'],
      ['pele-commit-cherry-pick', 'fermaid-commit-cherry-pick'],
      ['pele-commit-highlight', 'fermaid-commit-highlight'],
      ['pele-commit-reverse', 'fermaid-commit-reverse'],
      ['pele-commit-normal', 'fermaid-commit-normal'],
      ['pele-commit-merge', 'fermaid-commit-merge'],
      ['pele-commit-label', 'fermaid-commit-label'],
      ['pele-commit', 'fermaid-commit'],
      ['pele-tag', 'fermaid-tag'],
      ['pele-label', 'fermaid-label'],
      ['pele-title', 'fermaid-title'],
    ],
    cases: {
      history: [
        `gitGraph
    commit
    commit
    commit
`,
        {},
      ],
      branches: [
        `gitGraph
    commit
    branch develop
    commit
    checkout main
    merge develop
    commit
`,
        {},
      ],
      tags: [
        `gitGraph
    commit id:"one" tag:"v1.0" msg:"first"
    commit id:"two" type:HIGHLIGHT tag:"v1.1"
    branch release
    commit id:"three" type:REVERSE
`,
        {},
      ],
      down: [`gitGraph TB:\n    commit\n    branch other\n    commit\n`, {}],
      up: [`gitGraph BT:\n    commit\n    commit\n`, {}],
      titled: [
        `gitGraph
    title The history of a project
    commit
    commit
`,
        {},
      ],
      plain: [
        `gitGraph
    commit
    branch feature
    commit
    checkout main
    merge feature
`,
        { gitGraph: { showBranches: false } },
      ],
      parallel: [
        `gitGraph
    commit
    branch a
    commit
    branch b
    commit
`,
        { gitGraph: { parallelCommits: true } },
      ],
    },
  },
  quadrantchart: {
    modules: {
      'src/diagrams/quadrant/db.ts': ['QuadrantDb'],
      'src/diagrams/quadrant/parser.ts': ['parseQuadrant'],
      'src/diagrams/quadrant/render.ts': ['renderQuadrant'],
    },
    names: [
      ['pele-quadrants', 'fermaid-quadrants'],
      ['pele-quadrant', 'fermaid-quadrant'],
      ['pele-axis-labels', 'fermaid-axis-labels'],
      ['pele-point-labels', 'fermaid-point-labels'],
      ['pele-points', 'fermaid-points'],
      ['pele-point', 'fermaid-point'],
      ['pele-label', 'fermaid-label'],
      ['pele-title', 'fermaid-title'],
    ],
    cases: {
      campaigns: [
        `quadrantChart
    title Reach and engagement of campaigns
    x-axis Low Reach --> High Reach
    y-axis Low Engagement --> High Engagement
    quadrant-1 We should expand
    quadrant-2 Need to promote
    quadrant-3 Re-evaluate
    quadrant-4 May be improved
    Campaign A: [0.3, 0.6]
    Campaign B: [0.45, 0.23]
    Campaign C: [0.57, 0.69]
    Campaign D: [0.78, 0.34]
    Campaign E: [0.40, 0.34]
    Campaign F: [0.35, 0.78]
`,
        {},
      ],
      boxes: [
        `quadrantChart
  x-axis Urgent --> Not Urgent
  y-axis Not Important --> "Important ❤"
  quadrant-1 Plan
  quadrant-2 Do
  quadrant-3 Delegate
  quadrant-4 Delete
`,
        {},
      ],
      styled: [
        `quadrantChart
  title Styled
  x-axis Low Reach --> High Reach
  y-axis Low Engagement --> High Engagement
  quadrant-1 We should expand
  quadrant-2 Need to promote
  quadrant-3 Re-evaluate
  quadrant-4 May be improved
  Campaign A: [0.9, 0.0] radius: 12
  Campaign B:::class1: [0.8, 0.1] color: #ff3300, radius: 10
  Campaign C: [0.7, 0.2] radius: 25, color: #00ff33, stroke-color: #10f0f0
  Campaign D: [0.6, 0.3] radius: 15, stroke-color: #00ff0f, stroke-width: 5px ,color: #ff33f0
  Campaign E:::class2: [0.5, 0.4]
  Campaign F:::class3: [0.4, 0.5] color: #0000ff
  classDef class1 color: #109060
  classDef class2 color: #908342, radius : 10, stroke-color: #310085, stroke-width: 10px
  classDef class3 color: #f00fff, radius : 10
`,
        {},
      ],
      axes: [
        `quadrantChart
  x-axis Low --> High
  y-axis Low --> High
  quadrant-1 A
  quadrant-2 B
  quadrant-3 C
  quadrant-4 D
`,
        { quadrantChart: { xAxisPosition: 'bottom', yAxisPosition: 'right' } },
      ],
      small: [
        `quadrantChart
  x-axis Low --> High
  y-axis Low --> High
  quadrant-1 A
  quadrant-2 B
`,
        {},
      ],
    },
  },
};

const spec = TYPES[type];
if (!spec) {
  console.error(`unknown type: ${type}`);
  process.exit(2);
}

const { XyChartDb, parseXyChart, renderXyChart, QuadrantDb, parseQuadrant, renderQuadrant, buildGit, parseGit, renderGit } =
  await loadPele(resolve(peleDir), spec.modules);

function draw(src, config) {
  if (type === 'xychart') {
    const db = new XyChartDb();
    parseXyChart(src, db);
    return renderXyChart(db, config, {}).svg;
  }
  if (type === 'git') {
    const model = buildGit(parseGit(src), config, undefined);
    return renderGit(model, config, {}).svg;
  }
  const db = new QuadrantDb();
  parseQuadrant(src, db);
  return renderQuadrant(db, config, {}).svg;
}

/// The configuration of a chart, as the front matter fermaid reads it.
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

function normalize(svg) {
  let out = svg;
  for (const [a, b] of spec.names) {
    out = out
      .split(`"${a}"`)
      .join(`"${b}"`)
      .split(` ${a}"`)
      .join(` ${b}"`)
      .split(`${a} `)
      .join(`${b} `);
  }
  out = out.split('--pele-').join('--fermaid-');
  out = out.replace(/var\(--fermaid-series-\d+/g, 'var(--fermaid-series-N');
  out = out.replace(/class="pele pele-([^"]+)"/g, 'class="fermaid fermaid-$1"');
  out = out.replace(/id="pele-/g, 'id="fermaid-');
  out = out.replace(/aria-(labelledby|describedby)="pele-/g, 'aria-$1="fermaid-');
  return out;
}

const work = mkdtempSync(join(tmpdir(), 'fermaid-render-'));
let differ = 0;
for (const [name, [src, config]] of Object.entries(spec.cases)) {
  const pele = normalize(draw(src, config));

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
