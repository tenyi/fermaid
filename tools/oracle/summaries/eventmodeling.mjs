// The event modeling diagram: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the grammar Mermaid generates from its own, and the model is filled
// by pele's port of Mermaid's event modeling database. This module is the shape every summary
// module of this directory has: the summary of the model, the modules of a pele checkout the
// reference needs, what a corpus entry of the type starts with, what the mutator writes into an
// input, and the reference itself.
import { list, loadPele, q } from '../lib.mjs';

export const eventmodelingSummary = (model) => {
  const out = [];
  for (const item of model.boxes) {
    const lane = model.lanes.find((lane) => lane.index === item.lane.index);
    out.push(
      `box:${q(item.id)}|${item.kind}|${q(item.entity)}|${q(item.name)}|${q(item.namespace)}|` +
        `${item.reset}|${lane ? lane.index : 0}|${q(lane ? lane.label : undefined)}|` +
        `${q(item.data)}|${q(item.dataType)}`
    );
  }
  for (const lane of model.lanes) {
    out.push(`lane:${lane.index}|${q(lane.label)}|${lane.group}|${q(lane.namespace)}`);
  }
  for (const relation of model.relations) {
    out.push(`rel:${q(relation.source.id)}->${q(relation.target.id)}`);
  }
  for (const note of model.notes) {
    out.push(`note:${q(note.frame)}|${q(note.text)}|${q(note.dataType)}`);
  }
  for (const specification of model.specifications) {
    out.push(
      `spec:${q(specification.frame)}|given=${list(specification.given)}|` +
        `when=${list(specification.when)}|then=${list(specification.then)}`
    );
  }
  for (const entity of model.entities) {
    out.push(`entity:${q(entity)}`);
  }
  out.push(`title:${q(model.title)}`);
  return out.join('\n');
};

export const eventmodelingModules = {
  'src/diagrams/eventmodeling/model.ts': ['buildEventModel'],
  'src/preprocess.ts': ['encodeEntities'],
  '@mermaid-js/parser': ['parse as langiumParse'],
};

export const eventmodelingKeyword = /(^|\n)\s*eventmodeling\b/;

export const eventmodelingFragments = [
  'eventmodeling',
  'tf ',
  'timeframe ',
  'rf ',
  'resetframe ',
  'ui ',
  'cmd ',
  'command ',
  'evt ',
  'event ',
  'rmo ',
  'readmodel ',
  'pcr ',
  'processor ',
  '01 ',
  '02 ',
  '03 ',
  '999 ',
  'entity ',
  'data ',
  'note ',
  'gwt ',
  'given ',
  'when ',
  'then ',
  '->> ',
  '[[',
  ']]',
  '`json`',
  '`md`',
  '`text`',
  '{ a: 1 }',
  '{\n  a: 1\n}\n',
  'CartUI',
  'AddItem',
  'ItemAdded',
  'Inventory.AddItem',
  'A',
  '\n',
  ' ',
  '%%',
  '//',
  '/*',
  '*/',
  'é',
];

// A Langium tree without its bookkeeping, but with the text of its references.
function plain(node) {
  if (Array.isArray(node)) return node.map(plain);
  if (node === null || typeof node !== 'object') return node;
  if (typeof node.$refText === 'string') return { $refText: node.$refText };
  const out = {};
  for (const [key, value] of Object.entries(node)) {
    if (key === '$cstNode' || key === '$container' || key === '$containerProperty' || key === '$containerIndex') continue;
    if (key.startsWith('$') && key !== '$type') continue;
    if (value !== undefined) out[key] = plain(value);
  }
  return out;
}

// The bundle of the checkout and the parser Mermaid's grammar generates, built once.
let bundle;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, eventmodelingModules);
  return bundle;
}

export const eventmodelingReference = async (peleDir, src) => {
  const m = await reference(peleDir);
  // Mermaid's Diagram.fromText encodes entities and adds a line break.
  const text = m.encodeEntities(src) + '\n';
  let ast;
  try {
    ast = await m.langiumParse('eventmodeling', text);
  } catch (e) {
    // Mermaid refuses the source: its parser found something it cannot read, which is a
    // syntax error.
    const error = new Error(String(e?.message ?? e));
    error.code = 'syntax';
    throw error;
  }
  // pele's model reads the source and the place of each frame, to tell which line a number
  // that is used twice is on.
  const offsets = new Map(
    (ast.frames ?? []).map((frame) => [frame.name, frame.$cstNode?.startOffset ?? 0])
  );
  const model = plain(ast);
  model.$source = text;
  for (const frame of model.frames ?? []) frame.$offset = offsets.get(frame.name) ?? 0;
  return eventmodelingSummary(m.buildEventModel(model, undefined));
};
