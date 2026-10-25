// The swimlane diagram: what Mermaid makes of a source, as a string.
//
// A swimlane diagram is a flowchart whose top-level subgraphs are the lanes: Mermaid reads it with
// the flowchart's own grammar and fills the flowchart's own model, and only the layout and the
// drawing of the lanes are of the swimlane's own. So the model here is the flowchart's, and the
// reference is the parser jison generates from Mermaid's flowchart grammar, driving pele's port of
// Mermaid's flowchart database.
//
// This module is the shape every summary module of this directory has: the summary of the model,
// the modules of a pele checkout the reference needs, what a corpus entry of the type starts
// with, what the mutator writes into an input, and the reference itself.
import { join, resolve } from 'node:path';
import { asYy, flowSummary, loadPele } from '../lib.mjs';

export const swimlaneSummary = (model) => flowSummary(model);

export const swimlaneModules = {
  'tests/support/jison.ts': ['buildParser'],
  'src/diagrams/flowchart/db.ts': ['FlowDb'],
  'src/preprocess.ts': ['preprocess'],
};

export const swimlaneKeyword = /(^|\n)\s*swimlane-beta\b/;

export const swimlaneFragments = [
  'swimlane-beta',
  'swimlane-beta TD',
  'swimlane-beta LR',
  'subgraph ',
  'end\n',
  'direction LR\n',
  '-->',
  '---',
  '-.->',
  '==>',
  '--o',
  '--x',
  '<-->',
  '~~~',
  '[',
  ']',
  '(',
  ')',
  '{',
  '}',
  '((',
  '))',
  '>',
  '|',
  '"',
  '`',
  'subgraph lane [Title]\n',
  'classDef c fill:red\n',
  'class A c\n',
  'style A fill:red\n',
  'linkStyle 0 stroke:red\n',
  'click A "https://x"\n',
  'title T\n',
  'accTitle: t\n',
  'accDescr: d\n',
  '@{ shape: circle }',
  'A',
  'B',
  'é',
  '%%',
  '\n',
  ' ',
  ';',
  '&',
  ':::',
];

// The bundle of the checkout and the parser Mermaid's flowchart grammar generates, built once.
let bundle;
let parser;
async function reference(peleDir) {
  bundle ??= await loadPele(peleDir, swimlaneModules);
  parser ??= bundle.buildParser(
    join(resolve(peleDir), 'tests/compat/flowchart/upstream/flow.jison')
  );
  return { bundle, parser };
}

export const swimlaneReference = async (peleDir, src) => {
  const { bundle, parser } = await reference(peleDir);
  const db = new bundle.FlowDb({ maxEdges: 500 });
  // The grammar asks the database whether it has seen a graph before, so that only the first
  // `swimlane-beta` is followed by a direction.
  let first = true;
  parser.yy = asYy(db, { lex: { firstGraph: () => first && !(first = false) } });
  // Mermaid prepares the text before any parser sees it: the front matter and the comments are
  // taken off first.
  const text = bundle.preprocess(src).text;
  // Mermaid's flowParser.ts applies this before handing the text to the generated parser.
  parser.parse(text.replace(/}\s*\n/g, '}\n'));
  return flowSummary(db);
};
