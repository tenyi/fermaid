// The packet diagram: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the grammar Mermaid gives with Langium, and the model is filled by
// pele's port of Mermaid's packet database. This module is the shape every summary module of
// this directory has: the summary of the model, the modules of a pele checkout the reference
// needs, what a corpus entry of the type starts with, what the mutator writes into an input,
// and the reference itself.
import { loadPele, q } from '../lib.mjs';

export const packetSummary = (model) => {
  const out = [`title:${q(model.title)}|acc:${q(model.accTitle)}|${q(model.accDescr)}`];
  out.push(`bits:${model.bitsPerRow}`);
  model.rows.forEach((row, i) => {
    row.forEach((block, j) => {
      out.push(`block:${i}|${j}|${block.start}|${block.end}|${block.bits}|${q(block.label)}`);
    });
  });
  return out.join('\n');
};

export const packetModules = {
  'src/diagrams/packet/model.ts': ['buildPacket'],
  'src/preprocess.ts': ['preprocess', 'encodeEntities'],
  '@mermaid-js/parser': ['parse as langiumParse'],
};

export const packetKeyword = /(^|\n)\s*packet(-beta)?\b/;

export const packetFragments = [
  'packet',
  'packet-beta',
  'title ',
  'accTitle: ',
  'accDescr: ',
  'accDescr {',
  '}',
  '0-15',
  '16-31',
  '0-10',
  '+8',
  '+16',
  ': ',
  ':',
  '"',
  "'",
  '"field"',
  '0',
  '1',
  '16',
  '32',
  '-',
  '+',
  '\n',
  ' ',
  '%%',
  '---\n',
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

// The bundle of the checkout and Mermaid's grammar, built once.
let bundle;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, packetModules);
  return bundle;
}

export const packetReference = async (peleDir, src) => {
  const bundle = await reference(peleDir);
  // Mermaid's Diagram.fromText reads the front matter, the directives and the comments out of
  // the text before any parser sees it, and encodes the entities of the whole diagram.
  const pre = bundle.preprocess(src);
  let ast;
  try {
    ast = plain(await bundle.langiumParse('packet', bundle.encodeEntities(pre.text) + '\n'));
  } catch (e) {
    // A source Mermaid's grammar refuses is a syntax error.
    const error = new Error(e.message ?? String(e));
    error.code = e.code ?? 'syntax';
    throw error;
  }
  return packetSummary(bundle.buildPacket(ast, pre.config, pre.title));
};
