// The Wardley map: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the grammar Mermaid generates from its own, and the model is filled
// by pele's port of Mermaid's Wardley database. This module is the shape every summary module of
// this directory has: the summary of the model, the modules of a pele checkout the reference
// needs, what a corpus entry of the type starts with, what the mutator writes into an input, and
// the reference itself.
import { list, loadPele, q } from '../lib.mjs';

/// A number as JavaScript writes it, or `-` when there is none.
const num = (n) => (n === undefined || n === null ? '-' : String(n));

/// A flag as JavaScript writes it, or `-` when Mermaid leaves it unset.
const flag = (v) => (v === undefined || v === null ? '-' : String(v));

export const wardleySummary = (model) => {
  const out = [];
  for (const node of model.nodes.values()) {
    out.push(
      `node:${q(node.id)}|${q(node.label)}|x=${num(node.x)}|y=${num(node.y)}|class=${q(node.className)}` +
        `|lx=${num(node.labelOffsetX)}|ly=${num(node.labelOffsetY)}|pipe=${flag(node.inPipeline)}` +
        `|parent=${flag(node.isPipelineParent)}|inertia=${flag(node.inertia)}|strategy=${q(node.sourceStrategy)}`
    );
  }
  for (const link of model.links) {
    out.push(
      `link:${q(link.source)}|${q(link.target)}|dashed=${link.dashed}|label=${q(link.label)}|flow=${q(link.flow)}`
    );
  }
  for (const trend of model.trends.values()) {
    out.push(`trend:${q(trend.nodeId)}|x=${num(trend.targetX)}|y=${num(trend.targetY)}`);
  }
  for (const pipeline of model.pipelines.values()) {
    out.push(`pipeline:${q(pipeline.nodeId)}|members=${list(pipeline.componentIds)}`);
  }
  for (const annotation of model.annotations) {
    const at = annotation.coordinates
      .map((c) => `${num(c.x)},${num(c.y)}`)
      .join(';');
    out.push(`annotation:${num(annotation.number)}|at=${at}|text=${q(annotation.text)}`);
  }
  for (const note of model.notes) {
    out.push(`note:${q(note.text)}|x=${num(note.x)}|y=${num(note.y)}`);
  }
  for (const accelerator of model.accelerators) {
    out.push(`accelerator:${q(accelerator.name)}|x=${num(accelerator.x)}|y=${num(accelerator.y)}`);
  }
  for (const deaccelerator of model.deaccelerators) {
    out.push(
      `deaccelerator:${q(deaccelerator.name)}|x=${num(deaccelerator.x)}|y=${num(deaccelerator.y)}`
    );
  }
  out.push(
    `box:${model.annotationsBox ? `${num(model.annotationsBox.x)},${num(model.annotationsBox.y)}` : '-'}`
  );
  out.push(`axes:${list(model.axes.stages)}|${list(model.axes.stageBoundaries)}`);
  out.push(`size:${model.size ? `${num(model.size.width)},${num(model.size.height)}` : '-'}`);
  out.push(`title:${q(model.title)}|acc:${q(model.accTitle)}|${q(model.accDescr)}`);
  return out.join('\n');
};

export const wardleyModules = {
  'src/diagrams/wardley/model.ts': ['buildWardley'],
  'src/preprocess.ts': ['encodeEntities'],
  '@mermaid-js/parser': ['parse as langiumParse'],
};

export const wardleyKeyword = /(^|\n)\s*wardley-beta\b/;

export const wardleyFragments = [
  'wardley-beta',
  'component ',
  'anchor ',
  'note ',
  'evolution ',
  'evolve ',
  'pipeline ',
  'annotations ',
  'annotation ',
  'accelerator ',
  'deaccelerator ',
  'size ',
  'label ',
  'inertia',
  ' (build)',
  ' (buy)',
  ' (outsource)',
  ' (market)',
  ' [0.5, 0.5]',
  ' [0.25]',
  ' [-50, 10]',
  ' -> ',
  ' --> ',
  ' -.-> ',
  ' +<> ',
  ' +< ',
  ' +> ',
  " +'x'<>",
  ' ; a label',
  ' {',
  '}',
  '@0.5',
  '/ Second',
  '"a name"',
  "'a name'",
  ':',
  ',',
  '[',
  ']',
  '(',
  ')',
  "'",
  '"',
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
  bundle ??= await loadPele(peleDir, wardleyModules);
  return bundle;
}

export const wardleyReference = async (peleDir, src) => {
  const m = await reference(peleDir);
  // Mermaid's Diagram.fromText encodes entities and adds a line break.
  const ast = plain(await m.langiumParse('wardley', m.encodeEntities(src) + '\n'));
  return wardleySummary(m.buildWardley(ast, undefined));
};
