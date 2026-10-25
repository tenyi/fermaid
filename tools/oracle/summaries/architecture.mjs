// The architecture diagram: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the grammar Mermaid generates from its own, and the model is filled
// by pele's port of Mermaid's architecture database. This module is the shape every summary
// module of this directory has: the summary of the model, the modules of a pele checkout the
// reference needs, what a corpus entry of the type starts with, what the mutator writes into an
// input, and the reference itself.
import { list, loadPele, q } from '../lib.mjs';

export const architectureSummary = (model) => {
  const out = [];
  for (const group of model.groups.values()) {
    out.push(
      `group:${q(group.id)}|icon=${q(group.icon)}|title=${q(group.title)}|in=${q(group.in)}`
    );
  }
  for (const node of model.nodes.values()) {
    if (node.type === 'junction') {
      out.push(`junction:${q(node.id)}|in=${q(node.in)}`);
    } else {
      out.push(
        `service:${q(node.id)}|icon=${q(node.icon)}|iconText=${q(node.iconText)}|title=${q(node.title)}|in=${q(node.in)}`
      );
    }
  }
  for (const edge of model.edges) {
    out.push(
      `edge:${q(edge.lhsId)}|${edge.lhsDir}|${edge.lhsInto}|${edge.lhsGroup}|${q(edge.rhsId)}|${edge.rhsDir}|${edge.rhsInto}|${edge.rhsGroup}|${q(edge.title)}`
    );
  }
  for (const hint of model.layoutHints) {
    out.push(`align:${hint.direction}|members=${list(hint.members)}`);
  }
  out.push(`title:${q(model.title)}|acc:${q(model.accTitle)}|${q(model.accDescr)}`);
  return out.join('\n');
};

export const architectureModules = {
  'src/diagrams/architecture/db.ts': ['ArchitectureDb', 'populate'],
  'src/preprocess.ts': ['encodeEntities'],
  '@mermaid-js/parser': ['parse as langiumParse'],
};

export const architectureKeyword = /(^|\n)\s*architecture-beta\b/;

export const architectureFragments = [
  'architecture-beta',
  'service ',
  'group ',
  'junction ',
  'align row ',
  'align column ',
  ' in ',
  '(database)',
  '(cloud)',
  '[Name]',
  '["Name"]',
  '{group}',
  ':L',
  ':R',
  ':T',
  ':B',
  ' -- ',
  ' --> ',
  ' - [label] - ',
  ':',
  '-',
  '--',
  '{',
  '}',
  '(',
  ')',
  '[',
  ']',
  '"',
  "'",
  '\n',
  ' ',
  '%%',
  'title ',
  'accTitle: ',
  'accDescr: ',
  'A',
  'é',
];

// A Langium tree without its bookkeeping.
function plain(node) {
  if (Array.isArray(node)) return node.map(plain);
  if (node === null || typeof node !== 'object') return node;
  const out = {};
  for (const [key, value] of Object.entries(node)) {
    if (key.startsWith('$')) continue;
    if (value !== undefined) out[key] = plain(value);
  }
  return out;
}

// The bundle of the checkout and the parser Mermaid's grammar generates, built once.
let bundle;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, architectureModules);
  return bundle;
}

export const architectureReference = async (peleDir, src) => {
  const m = await reference(peleDir);
  // Mermaid's Diagram.fromText encodes entities and adds a line break.
  const ast = plain(await m.langiumParse('architecture', m.encodeEntities(src) + '\n'));
  const db = new m.ArchitectureDb();
  m.populate(ast, db);
  return architectureSummary(db);
};
