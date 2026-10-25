//! Venn diagrams: sets of labelled circles that overlap, as Mermaid's `venn-beta` reads them.

mod db;
mod layout;
mod lexer;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

/// The venn diagram type.
pub(crate) struct Venn;

impl Diagram for Venn {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let mut model = db::VennDb::default();
        parser::parse_venn(&input.source, &mut model)?;
        let model = model.build(input.title);
        Ok(render::render_venn(&model, input))
    }
}
