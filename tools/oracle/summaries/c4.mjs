// The C4 diagram: what Mermaid makes of a source, as a string.
//
// The diagram is parsed with the parser Mermaid generates from its own grammar, and the model is
// filled by pele's port of Mermaid's C4 database. The grammar hands the database its arguments
// spread out, where pele's port takes them as a list, so the calls are adapted here.
import { join, resolve } from 'node:path';
import { asYy, list, loadPele, q } from '../lib.mjs';

// The calls of the grammar, as pele's port of Mermaid's database takes them. The grammar hands a
// named argument over as an object with one key, where pele's port takes the key and the value
// apart, so the arguments are taken apart here.
const args = (list) =>
  list.map((attr) =>
    attr && typeof attr === 'object'
      ? { key: Object.keys(attr)[0], value: Object.values(attr)[0] }
      : attr
  );

const calls = (model) => ({
  setC4Type: (type) => model.setC4Type(type),
  setTitle: (title) => model.setTitle(title),
  setAccDescription: (descr) => model.setAccDescription(descr),
  addPersonOrSystem: (kind, ...attrs) => model.addPersonOrSystem(kind, args(attrs)),
  addContainer: (kind, ...attrs) => model.addContainer(kind, args(attrs)),
  addComponent: (kind, ...attrs) => model.addComponent(kind, args(attrs)),
  addPersonOrSystemBoundary: (...attrs) => model.addPersonOrSystemBoundary(args(attrs)),
  addContainerBoundary: (...attrs) => model.addContainerBoundary(args(attrs)),
  addDeploymentNode: (nodeType, ...attrs) => model.addDeploymentNode(nodeType, args(attrs)),
  addRel: (kind, ...attrs) => model.addRel(kind, args(attrs)),
  updateElStyle: (kind, ...attrs) => model.updateElStyle(kind, args(attrs)),
  updateRelStyle: (kind, ...attrs) => model.updateRelStyle(kind, args(attrs)),
  updateLayoutConfig: (kind, ...attrs) => model.updateLayoutConfig(kind, args(attrs)),
  popBoundaryParseStack: () => model.popBoundaryParseStack(),
});

// The named arguments that set no known field.
const extras = (element) =>
  list([...(element.extra ?? [])].map(([name, value]) => `${name}=${value}`));

// The fields a shape and a boundary have in common.
const fields = (element) => [
  q(element.alias),
  q(element.parent?.alias),
  q(element.label),
  q(element.labelAttr?.key),
  q(element.aliasAttr?.key),
  q(element.type),
  q(element.descr),
  q(element.techn),
  q(element.sprite),
  q(element.tags),
  q(element.link),
  q(element.bgColor),
  q(element.fontColor),
  q(element.borderColor),
  q(element.shadowing),
  q(element.shape),
  q(element.legendText),
  q(element.legendSprite),
  extras(element),
];

export const c4Summary = (model) => {
  const out = [];
  out.push(`c4Type:${q(model.c4Type)}`);
  out.push(`title:${q(model.title)}`);
  out.push(`accTitle:${q(model.accTitle)}`);
  out.push(`accDescr:${q(model.accDescr)}`);
  out.push(`shapeInRow:${model.shapeInRow}`);
  out.push(`boundaryInRow:${model.boundaryInRow}`);
  for (const shape of model.shapes) {
    out.push(`shape:${[q(shape.kind), ...fields(shape)].join('|')}`);
  }
  for (const boundary of model.boundaries) {
    out.push(`boundary:${[q(boundary.nodeType), ...fields(boundary)].join('|')}`);
  }
  for (const rel of model.rels) {
    out.push(
      `rel:${[
        q(rel.type),
        q(rel.from),
        q(rel.to),
        q(rel.label),
        q(rel.labelAttr?.key),
        q(rel.fromAttr?.key),
        q(rel.toAttr?.key),
        q(rel.techn),
        q(rel.descr),
        q(rel.sprite),
        q(rel.tags),
        q(rel.link),
        q(rel.textColor),
        q(rel.lineColor),
        rel.offsetX === undefined ? '-' : String(rel.offsetX),
        rel.offsetY === undefined ? '-' : String(rel.offsetY),
        extras(rel),
      ].join('|')}`
    );
  }
  return out.join('\n');
};

export const c4Modules = {
  'src/diagrams/c4/db.ts': ['C4Db'],
  'tests/support/jison.ts': ['buildParser'],
  'src/preprocess.ts': ['encodeEntities'],
};

export const c4Keyword = /(^|\n)\s*C4(Context|Container|Component|Dynamic|Deployment)\b/;

export const c4Fragments = [
  'C4Context',
  'C4Container',
  'C4Component',
  'C4Dynamic',
  'C4Deployment',
  'Person',
  'Person_Ext',
  'System',
  'SystemDb',
  'SystemQueue',
  'System_Ext',
  'SystemDb_Ext',
  'SystemQueue_Ext',
  'Container',
  'ContainerDb',
  'ContainerQueue',
  'Container_Ext',
  'ContainerDb_Ext',
  'ContainerQueue_Ext',
  'Component',
  'ComponentDb',
  'ComponentQueue',
  'Component_Ext',
  'ComponentDb_Ext',
  'ComponentQueue_Ext',
  'Enterprise_Boundary',
  'System_Boundary',
  'Boundary',
  'Container_Boundary',
  'Deployment_Node',
  'Node',
  'Node_L',
  'Node_R',
  'Rel',
  'BiRel',
  'Rel_Up',
  'Rel_Down',
  'Rel_Left',
  'Rel_Right',
  'Rel_Back',
  'RelIndex',
  'UpdateElementStyle',
  'UpdateRelStyle',
  'UpdateLayoutConfig',
  'title ',
  'accTitle: ',
  'accDescr: ',
  'accDescr {',
  '}',
  '(',
  ')',
  '{',
  '}',
  ',',
  '"',
  '$link="https://x"',
  '$tags="a,b"',
  '$sprite="users"',
  '$descr="d"',
  '$techn="t"',
  '$type="t"',
  '$shape="box"',
  '$bgColor="red"',
  '$fontColor="red"',
  '$borderColor="red"',
  '$shadowing="true"',
  '$legendText="l"',
  '$legendSprite="s"',
  'c4ShapeInRow',
  'c4BoundaryInRow',
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
  bundle ??= await loadPele(peleDir, c4Modules);
  parser ??= bundle.buildParser(
    join(resolve(peleDir), 'tests/compat/c4/upstream/c4Diagram.jison')
  );
  return { bundle, parser };
}

export const c4Reference = async (peleDir, src) => {
  const { bundle, parser } = await reference(peleDir);
  const model = new bundle.C4Db();
  parser.yy = asYy(model, calls(model));
  try {
    // Mermaid's Diagram.fromText encodes entities and adds a line break.
    parser.parse(bundle.encodeEntities(src) + '\n');
  } catch (e) {
    // Mermaid's own parser refuses the source with a parse error.
    if (e && typeof e === 'object') e.code = 'syntax';
    throw e;
  }
  return c4Summary(model);
};
