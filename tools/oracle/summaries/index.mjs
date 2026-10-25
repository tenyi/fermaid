// Dev-only: loads the per-type summary modules of this directory.
//
// A type that has a file here is compared with its own reference, so a type can be added
// without touching `lib.mjs`, `golden.mjs` or `diff.mjs`. The file is named after the type and
// exports, all named after the type as well:
//
//   export const <type>Summary = (model) => string;   // the model, as a string
//   export const <type>Modules = { path: [names] };   // the modules of a pele checkout
//   export const <type>Keyword = RegExp;              // what a corpus entry of the type starts with
//   export const <type>Fragments = string[];          // what the mutator writes into an input
//   export const <type>CorpusName = string;           // optional, when pele names it otherwise
//   export const <type>Reference = async (peleDir, src) => string;
//
// The reference throws when it refuses the source; both `golden.mjs` and `diff.mjs` read that
// as an error. It must be the behaviour of Mermaid: Mermaid's own parser where there is one, and
// pele's port of it where there is not.
import { readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));

/// Reads every summary module of this directory and returns them, keyed by the type.
export async function loadSummaries() {
  const out = new Map();
  const files = readdirSync(here)
    .filter((f) => f.endsWith('.mjs') && f !== 'index.mjs' && !f.startsWith('_'))
    .sort();
  for (const file of files) {
    const type = file.slice(0, -'.mjs'.length);
    const mod = await import(pathToFileURL(join(here, file)).href);
    const spec = {
      type,
      summary: mod[`${type}Summary`],
      modules: mod[`${type}Modules`] ?? {},
      keyword: mod[`${type}Keyword`],
      fragments: mod[`${type}Fragments`] ?? [],
      corpusName: mod[`${type}CorpusName`],
      reference: mod[`${type}Reference`],
    };
    for (const what of ['summary', 'keyword', 'reference']) {
      if (spec[what] === undefined) {
        throw new Error(`${file} does not export ${type}${what[0].toUpperCase()}${what.slice(1)}`);
      }
    }
    out.set(type, spec);
  }
  return out;
}
