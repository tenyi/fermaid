//! Requirement diagrams: requirements and elements with their fields, relationships between
//! them, `classDef`, `class` and `style`.

mod db;
mod graph;
mod lexer;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct Requirement;

impl Diagram for Requirement {
    fn math(&self) -> bool {
        true
    }

    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let mut db = db::RequirementDb::new();
        if let Some(title) = input.title.filter(|t| !t.is_empty()) {
            db.title = Some(title.to_string());
        }
        parser::parse_requirement(&input.source, &mut db)?;
        Ok(render::render_requirement(&db, input))
    }
}
