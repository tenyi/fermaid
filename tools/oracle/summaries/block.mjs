// The block diagram: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the parser Mermaid generates from its own grammar, and the model is
// filled by pele's port of Mermaid's block database. This module is the shape every summary
// module of this directory has: the summary of the model, the modules of a pele checkout the
// reference needs, what a corpus entry of the type starts with, what the mutator writes into an
// input, and the reference itself.
import { join, resolve } from 'node:path';
import { asYy, list, loadPele, q } from '../lib.mjs';

export const blockSummary = (model) => {
  const out = [];
  const walk = (block, depth) => {
    out.push(
      `block:${depth}|${q(block.id)}|${q(block.type ?? 'na')}|${q(block.label ?? '')}|w=${block.widthInColumns ?? '-'}|c=${block.columns ?? '-'}|cls=${list(block.classes ?? [])}|s=${list(block.styles ?? [])}`
    );
    for (const child of block.children ?? []) walk(child, depth + 1);
  };
  walk(model.root, 0);
  for (const edge of model.edges) {
    out.push(
      `edge:${q(edge.id)}|${q(edge.start ?? '')}|${q(edge.end ?? '')}|${q(edge.label ?? '')}|${q(edge.arrowTypeStart ?? '')}|${q(edge.arrowTypeEnd ?? '')}|${q(edge.thickness ?? '')}|${q(edge.pattern ?? '')}`
    );
  }
  for (const [name, def] of model.classes) {
    out.push(`class:${q(name)}|s=${list(def.styles)}|t=${list(def.textStyles)}`);
  }
  return out.join('\n');
};

export const blockModules = {
  'src/diagrams/block/db.ts': ['BlockDb'],
  'tests/support/jison.ts': ['buildParser'],
  'src/preprocess.ts': ['encodeEntities'],
};

export const blockKeyword = /(^|\n)\s*block(-beta)?\b/;

export const blockFragments = ['block', 'block-beta', 'block:', 'columns 3', 'columns auto', 'space', 'space:3', 'end', '-->', '---', '-.->', '==>', '--o', '--x', '<-->', '~~~', '[', ']', '(', ')', '{', '}', '((', '))', '{{', '}}', '>', '|', '"', '`', '\\[', '/]', ']', '<[', ']>', '(right)', ',left', ',up', ',down', ',x', ',y', 'classDef c fill:red', 'class A c', 'style A fill:red', ':2', '\n', ' ', '%%', 'A', 'é'];

// The bundle of the checkout and the parser Mermaid's grammar generates, built once.
let bundle;
let parser;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, blockModules);
  parser ??= bundle.buildParser(join(resolve(peleDir), 'tests/compat/block/upstream/block.jison'));
  return { bundle, parser };
}

export const blockReference = async (peleDir, src) => {
  const { bundle, parser } = await reference(peleDir);
  const db = new bundle.BlockDb();
  parser.yy = asYy(db, { getLogger: () => ({ debug() {}, log() {}, warn() {}, error() {} }) });
  // Mermaid's Diagram.fromText encodes entities and adds a line break.
  parser.parse(bundle.encodeEntities(src) + '\n');
  return blockSummary(db);
};
