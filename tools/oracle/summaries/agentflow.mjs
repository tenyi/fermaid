// The agent flow diagram: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the parser Mermaid generates from its own grammar, and the model is
// filled by pele's port of Mermaid's agentflow database. This module is the shape every summary
// module of this directory has: the summary of the model, the modules of a pele checkout the
// reference needs, what a corpus entry of the type starts with, what the mutator writes into an
// input, and the reference itself.
import { join, resolve } from 'node:path';
import { asYy, list, loadPele, q } from '../lib.mjs';

export const agentflowSummary = (model) => {
  const out = [];
  for (const v of model.vertices) {
    out.push(`vertex:${q(v.id)}|${q(v.label)}|${q(v.shape)}|${q(v.vertexKind)}`);
  }
  for (const c of model.connectors) {
    out.push(`connector:${q(c.id)}|${q(c.title)}`);
  }
  for (const e of model.edges) {
    out.push(
      `edge:${q(e.start)}|${q(e.end)}|${q(e.id)}|${q(e.label ?? undefined)}|${q(e.type ?? undefined)}|${q(e.stroke ?? undefined)}|${q(e.edgeSemantic ?? undefined)}|${e.length ?? '-'}`
    );
  }
  for (const s of model.subGraphs) {
    out.push(`subgraph:${q(s.id)}|${list(s.nodes)}|${q(s.type ?? 'flow')}|${q(s.title ?? '')}|${q(s.direction ?? undefined)}`);
  }
  if (model.direction !== undefined) out.push(`direction:${q(model.direction)}`);
  return out.join('\n');
};

export const agentflowModules = {
  'src/diagrams/agentflow/db.ts': ['AgentflowDb'],
  'tests/support/jison.ts': ['buildParser'],
  'src/preprocess.ts': ['encodeEntities'],
};

export const agentflowKeyword = /(^|\n)\s*agentflow-beta\b/;

export const agentflowFragments = [
  'agentflow-beta',
  'agentflow-beta TB',
  'agentflow-beta LR',
  'flow',
  'connector',
  'global',
  'end',
  'direction',
  'TB',
  'BT',
  'RL',
  'LR',
  'TD',
  '-->',
  '--x',
  '---',
  '-.->',
  '-.-',
  '--',
  '|',
  '"',
  '`',
  '[',
  ']',
  '(',
  ')',
  '{',
  '}',
  '((',
  '))',
  '{{',
  '}}',
  '[/',
  '/]',
  '[\\',
  '\\]',
  '>',
  'classDef c fill:red',
  'class A c',
  'style A fill:red',
  'linkStyle 0 stroke:#f00',
  'interpolate basis',
  'click A callback',
  'click A "https://example.com"',
  'accTitle: The title',
  'accDescr: The description',
  '@{ shape: task }',
  '@{ shape: tool }',
  '@{ shape: input }',
  '@{ shape: decision }',
  '@{ shape: refdoc }',
  '@{ shape: action }',
  '@{ shape: circle }',
  '@{ shape: stadium }',
  '@{ view: collapsed }',
  '@{ direction: LR }',
  '\n',
  ' ',
  '%%',
  'A',
  'é',
];

// The bundle of the checkout and the parser Mermaid's grammar generates, built once.
let bundle;
let parser;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, agentflowModules);
  parser ??= bundle.buildParser(
    join(resolve(peleDir), 'tests/compat/agentflow/upstream/agentflow.jison')
  );
  return { bundle, parser };
}

export const agentflowReference = async (peleDir, src) => {
  const { bundle, parser } = await reference(peleDir);
  const db = new bundle.AgentflowDb();
  // The grammar asks the database whether it has seen a graph before, so that only the first
  // `agentflow-beta` is followed by a direction.
  let first = true;
  parser.yy = asYy(db, {
    lex: {
      firstGraph: () => {
        const was = first;
        first = false;
        return was;
      },
    },
    getLogger: () => ({ debug() {}, log() {}, warn() {}, error() {} }),
  });
  // Mermaid's Diagram.fromText encodes entities and adds a line break. Its agentflow parser
  // also strips the horizontal whitespace between a closing `}` and the end of the line, so
  // that the grammar's `node shapeData separator` rule still reduces.
  const text = bundle
    .encodeEntities(src)
    .replace(/}[^\S\n]*\n/g, '}\n');
  parser.parse(text + '\n');
  return agentflowSummary(db.semanticModel());
};
