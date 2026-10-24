// Dev-only: writes what pele makes of each generated Mermaid spec case, as golden data for the
// compat tests. pele passes Mermaid's own specs (less what its SKIPPED.md lists), so a case on
// which fermaid's model matches pele's passes the spec too.
//
//   node tools/oracle/golden.mjs <pele checkout> class|er|flowchart|requirement|sequence|state
//
// The checkout needs `npm ci` for esbuild. Writes crates/fermaid/tests/compat/<type>/expected.rs.
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { classSummary, erSummary, flowSummary, memberSummary, loadPele, requirementSummary, seqSummary, stateSummary } from './lib.mjs';

const [peleDir, type] = process.argv.slice(2);
if (!peleDir || !['class', 'er', 'flowchart', 'requirement', 'sequence', 'state'].includes(type)) {
  console.error('usage: node tools/oracle/golden.mjs <pele checkout> class|er|flowchart|requirement|sequence|state');
  process.exit(2);
}
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const { ClassDb, ClassMember, parseClassDiagram, buildClassGraph, ErDb, parseEr, buildErGraph, FlowDb, parseFlowchart, RequirementDb, parseRequirement, buildRequirementGraph, SeqDb, parseSequence, StateDb, parseState, encodeEntities } = await loadPele(peleDir, {
  'src/diagrams/class/db.ts': ['ClassDb'],
  'src/diagrams/class/parser.ts': ['parseClassDiagram'],
  'src/diagrams/class/graph.ts': ['buildClassGraph'],
  'src/diagrams/class/members.ts': ['ClassMember'],
  'src/diagrams/er/db.ts': ['ErDb'],
  'src/diagrams/er/parser.ts': ['parseEr'],
  'src/diagrams/er/graph.ts': ['buildErGraph'],
  'src/diagrams/flowchart/db.ts': ['FlowDb'],
  'src/diagrams/flowchart/parser.ts': ['parseFlowchart'],
  'src/diagrams/requirement/db.ts': ['RequirementDb'],
  'src/diagrams/requirement/parser.ts': ['parseRequirement'],
  'src/diagrams/requirement/graph.ts': ['buildRequirementGraph'],
  'src/diagrams/sequence/db.ts': ['SeqDb'],
  'src/diagrams/sequence/parser.ts': ['parseSequence'],
  'src/diagrams/state/db.ts': ['StateDb'],
  'src/diagrams/state/parser.ts': ['parseState'],
  'src/preprocess.ts': ['encodeEntities'],
});

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

let body = '';
for (const { name, src } of cases) {
  let result;
  if (src.includes('${')) result = 'template';
  else {
    try {
      if (type === 'flowchart') {
        const db = new FlowDb({ maxEdges: 500 });
        parseFlowchart(src, db);
        result = flowSummary(db);
      } else if (type === 'class') {
        // Many class specs test members on their own, so each source is also read as one.
        const members = `${memberSummary(new ClassMember(src, 'method'))}\n${memberSummary(new ClassMember(src, 'attribute'))}`;
        try {
          const db = new ClassDb();
          parseClassDiagram(src, db);
          result = `${classSummary(db, buildClassGraph)}\n${members}`;
        } catch (e) {
          result = `error:${e.code ?? 'internal'}\n${members}`;
        }
      } else if (type === 'er') {
        const db = new ErDb();
        parseEr(src, db);
        result = erSummary(db, buildErGraph);
      } else if (type === 'requirement') {
        const db = new RequirementDb();
        parseRequirement(src, db);
        result = requirementSummary(db, buildRequirementGraph);
      } else if (type === 'state') {
        const db = new StateDb();
        parseState(src, db);
        result = stateSummary(db);
      } else {
        // As pele's spec adapter does: the text as given, with entities encoded.
        const db = new SeqDb();
        parseSequence(encodeEntities(src) + '\n', db);
        result = seqSummary(db);
      }
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
