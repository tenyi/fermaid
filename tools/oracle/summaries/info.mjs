// The information diagram: what Mermaid makes of a source, as a string.
//
// The type has a grammar of its own in Mermaid, the Langium one of
// `packages/parser/src/language/info/info.langium`, so the reference reads the source with it.
// The model is what that grammar reads: a title, an accessible title and an accessible
// description.
import { loadPele, q } from '../lib.mjs';

export const infoSummary = (model) =>
  `title:${q(model.title)}|acc:${q(model.accTitle)}|${q(model.accDescr)}`;

export const infoModules = {
  'src/preprocess.ts': ['preprocess', 'encodeEntities'],
  '@mermaid-js/parser': ['parse as langiumParse'],
};

export const infoKeyword = /(^|\n)\s*info\b/;

export const infoFragments = [
  'info',
  'info ',
  'info\n',
  'showInfo',
  'showInfo ',
  'title ',
  'title',
  'accTitle: ',
  'accTitle:',
  'accDescr: ',
  'accDescr:',
  'accDescr {',
  '}',
  '{',
  ':',
  '\n',
  ' ',
  '\t',
  '%%',
  '%% c',
  'x',
  'Hello',
  'é',
];

// The bundle of the checkout and Mermaid's grammar, built once.
let bundle;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, infoModules);
  return bundle;
}

export const infoReference = async (peleDir, src) => {
  const bundle = await reference(peleDir);
  // Mermaid's Diagram.fromText reads the front matter, the directives and the comments out of
  // the text before any parser sees it, and encodes the entities of the whole diagram.
  const pre = bundle.preprocess(src);
  let ast;
  try {
    ast = await bundle.langiumParse('info', bundle.encodeEntities(pre.text) + '\n');
  } catch (e) {
    // A source Mermaid's grammar refuses is a syntax error.
    const error = new Error(e.message ?? String(e));
    error.code = e.code ?? 'syntax';
    throw error;
  }
  return infoSummary({
    title: ast.title,
    accTitle: ast.accTitle,
    accDescr: ast.accDescr,
  });
};
