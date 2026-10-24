// Dev-only helpers shared by the oracle scripts: a bundle of the parts of a pele checkout they
// use, and the model summaries the compat tests and the differential run compare.
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

// Bundles the named modules of a pele checkout (`npm ci` done, for esbuild) and imports them.
// `exports` maps a path in the checkout, or a package name, to the names it exports. Packages
// stay outside the bundle, which is written into the checkout's node_modules/.cache so that
// they resolve.
export async function loadPele(peleDir, exports) {
  const pele = resolve(peleDir);
  mkdirSync(join(pele, 'node_modules/.cache'), { recursive: true });
  const work = mkdtempSync(join(pele, 'node_modules/.cache/fermaid-oracle-'));
  const entry = join(work, 'entry.ts');
  writeFileSync(
    entry,
    Object.entries(exports)
      .map(([path, names]) => {
        const from = path.startsWith('@') ? path : join(pele, path);
        return `export { ${names.join(', ')} } from ${JSON.stringify(from)};\n`;
      })
      .join('')
  );
  const bundle = join(work, 'oracle.mjs');
  execFileSync(join(pele, 'node_modules/.bin/esbuild'), [
    entry,
    '--bundle',
    '--format=esm',
    '--platform=node',
    '--packages=external',
    `--outfile=${bundle}`,
    '--log-level=error',
  ]);
  return import(pathToFileURL(bundle).href);
}

export const q = (s) => (s === undefined || s === null ? '-' : JSON.stringify(String(s)));
export const list = (a) => (a && a.length ? JSON.stringify(a.join('\u0001')) : '-');
export function flowSummary(db) {
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

// JSON with the keys of every object sorted.
function canon(v) {
  if (v instanceof Map) v = Object.fromEntries(v);
  if (Array.isArray(v)) return `[${v.map(canon).join(',')}]`;
  if (v !== null && typeof v === 'object') {
    return `{${Object.keys(v).sort().map((k) => `${JSON.stringify(k)}:${canon(v[k])}`).join(',')}}`;
  }
  return JSON.stringify(v) ?? 'null';
}
const sorted = (map) => [...map.keys()].sort().map((k) => `${k}=${map.get(k)}`).join(',');
export function seqSummary(db) {
  const out = [];
  for (const [id, a] of db.actors) {
    const box = a.box ? db.boxes.indexOf(a.box) : '-';
    out.push(`actor:${q(id)}|${q(a.name)}|${q(a.description)}|${a.wrap}|${q(a.type)}|b=${box}|l=${canon(a.links)}|p=${canon(a.properties)}|d=${q(a.details)}`);
  }
  out.push(`created:${sorted(db.createdActors)}`);
  out.push(`destroyed:${sorted(db.destroyedActors)}`);
  for (const b of db.boxes) out.push(`box:${q(b.name)}|${b.wrap}|${q(b.fill)}|${list(b.actorKeys)}`);
  for (const m of db.messages) {
    const text = typeof m.message === 'string' ? q(m.message) : `n=${m.message.start ?? '-'},${m.message.step ?? '-'},${m.message.visible}`;
    out.push(`msg:${q(m.from)}|${q(m.to)}|${text}|${m.wrap}|${m.type ?? '-'}|${m.placement ?? '-'}|${m.centralConnection ?? 0}`);
  }
  out.push(`title:${q(db.title)}|acc:${q(db.accTitle)}|${q(db.accDescr)}`);
  return out.join('\n');
}


// The fields of a Langium pie tree that fermaid's parser keeps.
export function pieSummary(ast) {
  const out = [`show:${ast.showData === true}|t:${q(ast.title)}|at:${q(ast.accTitle)}|ad:${q(ast.accDescr)}`];
  for (const s of ast.sections ?? []) out.push(`section:${q(s.label)}|${s.value}`);
  return out.join('\n');
}
