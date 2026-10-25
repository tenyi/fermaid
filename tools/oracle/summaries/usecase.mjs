// The use case diagram: what Mermaid makes of a source, as a string.
//
// Mermaid has no grammar file for use case diagrams: its parser is written with Chevrotain, in a
// lexer, a parser and a visitor. pele's port of them is the reference here, as the project's
// reference for a type without a grammar of its own is the port of Mermaid's parser.
//
// This module is the shape every summary module of this directory has: the summary of the
// model, the modules of a pele checkout the reference needs, what a corpus entry of the type
// starts with, what the mutator writes into an input, and the reference itself.
import { list, loadPele, q } from '../lib.mjs';

// Mermaid hides the entity codes of the text before it parses it, and adds a line break to it.
// pele's port of the parser is called with the text as it is, so the reference does both here.
const OPEN = 'ﬂ°';
const CLOSE = '¶ß';

const isSpace = (c) => /\s/.test(c);
const isLineEnd = (c) => c === '\n' || c === '\r' || c === '\u2028' || c === '\u2029';

function dropColorSemicolons(text, keyword) {
  let out = '';
  let last = 0;
  let from = text.indexOf(keyword);
  while (from >= 0) {
    const start = from;
    let lineEnd = text.length;
    let next = text.length;
    for (let i = start; i < text.length; i++) {
      const c = text[i];
      if (isLineEnd(c)) {
        lineEnd = i;
        next = i + 1;
        break;
      }
    }
    let afterColon = false;
    let hash = -1;
    const rest = text.slice(start + keyword.length, lineEnd);
    for (let j = 0; j < rest.length; j++) {
      const c = rest[j];
      if (c === ':') {
        afterColon = true;
      } else if (c === '#') {
        if (afterColon) {
          hash = start + keyword.length + j;
          break;
        }
      } else if (isSpace(c)) {
        afterColon = false;
      }
    }
    if (hash >= 0) {
      const semi = text.lastIndexOf(';', lineEnd - 1);
      if (semi > hash) {
        out += text.slice(last, semi);
        last = semi + 1;
      }
    }
    from = lineEnd >= text.length ? -1 : text.indexOf(keyword, next);
  }
  if (last === 0) {
    return text;
  }
  out += text.slice(last);
  return out;
}

function encodeEntities(text) {
  if (!text.includes('#')) {
    return text;
  }
  const source = dropColorSemicolons(dropColorSemicolons(text, 'style'), 'classDef');
  let out = '';
  let rest = source;
  for (;;) {
    const at = rest.indexOf('#');
    if (at < 0) {
      break;
    }
    out += rest.slice(0, at);
    const after = rest.slice(at + 1);
    const match = /^[A-Za-z0-9_]+/.exec(after);
    if (match && after[match[0].length] === ';') {
      const inner = match[0];
      out += OPEN;
      if (/^[0-9]+$/.test(inner)) {
        out += '°';
      }
      out += inner;
      out += CLOSE;
      rest = after.slice(match[0].length + 1);
    } else {
      out += '#';
      rest = after;
    }
  }
  return out + rest;
}

export const usecaseSummary = (model, graph) => {
  const out = [];
  for (const node of graph.nodes) {
    out.push(
      `node:${q(node.id)}|${q(node.label)}|${q(node.labelType)}|${q(node.shape)}|${node.isGroup}|${node.padding}|${q(node.cssClasses)}|${list(node.cssStyles)}|${list(node.cssCompiledStyles)}|${q(node.actorType)}|${q(node.business)}|${q(node.icon)}|${q(node.stereotype)}|${q(node.parentId)}|${q(node.noteTarget)}|${list((node.jsonRows ?? []).map((row) => `${row.key}=${row.value}`))}|${q(node.boundaryType)}`
    );
  }
  for (const edge of graph.edges) {
    out.push(
      `edge:${q(edge.id)}|${q(edge.start)}|${q(edge.end)}|${q(edge.relationshipType)}|${q(edge.pattern)}|${q(edge.arrowTypeStart)}|${q(edge.arrowTypeEnd)}|${q(edge.label)}|${q(edge.labelType)}|${q(edge.classes)}|${list(edge.style)}|${list(edge.cssCompiledStyles)}|${edge.animate}|${q(edge.animation)}|${edge.minlen}|${edge.explicitId}|${edge.internal}`
    );
  }
  for (const statement of model.statements ?? []) {
    out.push(
      `stmt:${q(statement.kind)}|${q(statement.group)}|${q(statement.ref)}|${list((statement.nodes ?? []).map((node) => node.id))}|${list((statement.edges ?? []).map((edge) => edge.id))}`
    );
  }
  out.push(
    `model:${q(model.title)}|${q(model.accTitle)}|${q(model.accDescr)}|${q(model.direction)}`
  );
  return out.join('\n');
};

export const usecaseModules = {
  'src/diagrams/usecase/parser.ts': ['parseUsecase'],
  'src/diagrams/usecase/graph.ts': ['buildUsecaseGraph'],
};

export const usecaseKeyword = /(^|\n)\s*usecase-beta\b/;

export const usecaseFragments = [
  'usecase-beta',
  'actor',
  'systemBoundary',
  'end',
  '-->',
  '--',
  '<--',
  '..>',
  ': include',
  ': extend',
  '--|>',
  '<<',
  '>>',
  '@{',
  '}',
  ':::',
  'classDef',
  'class',
  'style',
  'note for',
  'direction',
  'accTitle:',
  'accDescr',
  'json',
  '(',
  ')',
  '[',
  ']',
  '"',
  '`',
  ',',
  ':',
  '\n',
  ' ',
  '%%',
  'A',
  'é',
];

// The bundle of the checkout and the parser, built once.
let bundle;
export const usecaseReference = async (peleDir, src) => {
  bundle ??= await loadPele(peleDir, usecaseModules);
  const model = bundle.parseUsecase(encodeEntities(src) + '\n');
  return usecaseSummary(model, bundle.buildUsecaseGraph(model));
};
