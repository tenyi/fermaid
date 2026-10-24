// Dev-only: writes what pele makes of each generated Mermaid spec case, as golden data for the
// compat tests. pele passes Mermaid's own specs (less what its SKIPPED.md lists), so a case on
// which fermaid's model matches pele's passes the spec too.
//
//   node tools/oracle/golden.mjs <pele checkout> flowchart
//
// The checkout needs `npm ci` for esbuild. Writes crates/fermaid/tests/compat/<type>/expected.rs.
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const [peleDir, type] = process.argv.slice(2);
if (!peleDir || type !== 'flowchart') {
  console.error('usage: node tools/oracle/golden.mjs <pele checkout> flowchart');
  process.exit(2);
}
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const pele = resolve(peleDir);
const work = mkdtempSync(join(tmpdir(), 'fermaid-oracle-'));
const entry = join(work, 'entry.ts');
writeFileSync(
  entry,
  `export { FlowDb } from ${JSON.stringify(join(pele, 'src/diagrams/flowchart/db.ts'))};\n` +
    `export { parseFlowchart } from ${JSON.stringify(join(pele, 'src/diagrams/flowchart/parser.ts'))};\n`
);
const bundle = join(work, 'oracle.mjs');
execFileSync(join(pele, 'node_modules/.bin/esbuild'), [entry, '--bundle', '--format=esm', `--outfile=${bundle}`, '--log-level=error']);
const { FlowDb, parseFlowchart } = await import(pathToFileURL(bundle).href);

// Reads the Rust string literals of cases.rs, as `{:?}` writes them.
function rustString(lit) {
  let out = '';
  for (let i = 1; i < lit.length - 1; i++) {
    const c = lit[i];
    if (c !== '\\') {
      out += c;
      continue;
    }
    const n = lit[++i];
    if (n === 'n') out += '\n';
    else if (n === 'r') out += '\r';
    else if (n === 't') out += '\t';
    else if (n === '0') out += '\0';
    else if (n === 'u') {
      const end = lit.indexOf('}', i);
      out += String.fromCodePoint(parseInt(lit.slice(i + 2, end), 16));
      i = end;
    } else out += n;
  }
  return out;
}
const casesRs = readFileSync(join(root, `crates/fermaid/tests/compat/${type}/cases.rs`), 'utf8');
const cases = [...casesRs.matchAll(/name: ("(?:[^"\\]|\\.)*"),\n\s*src: ("(?:[^"\\]|\\.)*"),/g)].map((m) => ({
  name: rustString(m[1]),
  src: rustString(m[2]),
}));

const q = (s) => (s === undefined || s === null ? '-' : JSON.stringify(String(s)));
const list = (a) => (a && a.length ? JSON.stringify(a.join('\u0001')) : '-');
function summary(db) {
  const out = [`dir:${q(db.direction)}`];
  for (const n of db.nodes.values()) {
    out.push(`node:${q(n.id)}|${q(n.text)}|${n.labelType}|${q(n.type)}|c=${list(n.classes)}|s=${list(n.styles)}|l=${q(n.link)}|t=${q(n.linkTarget)}`);
  }
  for (const e of db.edges) {
    out.push(
      `edge:${q(e.start)}|${q(e.end)}|${q(e.type)}|${q(e.stroke)}|${e.length ?? '-'}|${q(e.text)}|${e.labelType}|${q(e.id)}|i=${q(e.interpolate)}|s=${list(e.style)}|c=${list(e.classes)}`
    );
  }
  for (const s of db.subgraphs) out.push(`sub:${q(s.id)}|${q(s.title)}|${s.labelType}|n=${list(s.nodes)}|d=${q(s.dir)}|c=${list(s.classes)}`);
  for (const id of [...db.classes.keys()].sort()) {
    const c = db.classes.get(id);
    out.push(`class:${q(id)}|s=${list(c.styles)}|t=${list(c.textStyles)}`);
  }
  for (const id of [...db.tooltips.keys()].sort()) out.push(`tip:${q(id)}|${q(db.tooltips.get(id))}`);
  out.push(`default:s=${list(db.defaultEdgeStyle)}|i=${q(db.defaultInterpolate)}`);
  out.push(`acc:${q(db.accTitle)}|${q(db.accDescr)}`);
  return out.join('\n');
}

let body = '';
for (const { name, src } of cases) {
  let result;
  if (src.includes('${')) result = 'template';
  else {
    const db = new FlowDb({ maxEdges: 500 });
    try {
      parseFlowchart(src, db);
      result = summary(db);
    } catch (e) {
      result = `error:${e.code ?? 'internal'}`;
    }
  }
  body += `    (${JSON.stringify(name)}, ${JSON.stringify(result)}),\n`;
}
const target = join(root, `crates/fermaid/tests/compat/${type}/expected.rs`);
writeFileSync(
  target,
  `// What pele 1.9.0 makes of each case in cases.rs, as summarized by tools/oracle/golden.mjs.\n` +
    `// Generated; do not edit.\n\nconst EXPECTED: &[(&str, &str)] = &[\n${body}];\n`
);
console.log(`wrote ${cases.length} results to ${target}`);
