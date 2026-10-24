//! Flowcharts: `graph` / `flowchart`, nodes in every shape, edges, subgraphs, classes, styles
//! and click links.

#[cfg(test)]
mod compat;
mod db;
mod graph;
mod lexer;
mod parser;
mod render;
mod shapes;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;
use crate::preprocess::Value;

pub(crate) struct Flowchart;

impl Diagram for Flowchart {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let inherit_dir = match input.config.get("flowchart") {
            Some(Value::Map(flow)) => matches!(flow.get("inheritDir"), Some(Value::Bool(true))),
            _ => false,
        };
        let limits = &input.options.limits;
        let mut db = db::FlowDb::new(limits.max_edges, inherit_dir);
        if let Some(title) = input.title.filter(|t| !t.is_empty()) {
            db.title = Some(title.to_string());
        }
        parser::parse_flowchart(&input.source, &mut db, limits.max_depth)?;
        Ok(render::render_flowchart(&db, input))
    }
}
