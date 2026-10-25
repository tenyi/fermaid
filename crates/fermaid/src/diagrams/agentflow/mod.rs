//! Agent flow diagrams: the agents, the flows that hold them, and the edges between them.

mod db;
mod graph;
mod lexer;
mod metadata;
mod parser;
mod render;
mod shapes;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;
use crate::preprocess::Value;

pub(crate) struct Agentflow;

impl Diagram for Agentflow {
    /// Mermaid's agentflow parser reads the text with its comment lines kept, so that the
    /// positions it reports match the source.
    fn keep_comments(&self) -> bool {
        true
    }

    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let inherit_dir = match input.config.get("flowchart") {
            Some(Value::Map(flow)) => matches!(flow.get("inheritDir"), Some(Value::Bool(true))),
            _ => false,
        };
        let limits = &input.options.limits;
        let mut db = db::AgentflowDb::new(limits.max_edges, inherit_dir);
        db.title = input.title.map(str::to_string);
        parser::parse_agentflow(&input.source, &mut db)?;
        render::render_agentflow(&mut db, input)
    }
}
