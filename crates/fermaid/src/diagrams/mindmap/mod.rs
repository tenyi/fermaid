//! Mind maps: a tree of ideas, written as an indented outline.

mod db;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::diagrams::common::outline::parse_outline;
use crate::error::Error;

pub(crate) struct Mindmap;

impl Diagram for Mindmap {
    fn math(&self) -> bool {
        true
    }

    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let mut db = db::MindmapDb::default();
        db.title = input.title.map(str::to_string);
        parse_outline(&input.source, &mut db, false)?;
        Ok(render::render_mindmap(&db, input))
    }
}
