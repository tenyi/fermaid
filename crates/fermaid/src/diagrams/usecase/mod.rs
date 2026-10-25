//! Use case diagrams: the actors, the use cases, the boundaries and the notes, and the
//! relationships between them.

mod db;
mod graph;
mod json;
mod lexer;
mod parser;
mod render;
mod tokens;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct Usecase;

impl Diagram for Usecase {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let model =
            parser::parse_usecase(&input.source, input.title, input.options.limits.max_depth)?;
        Ok(render::render_usecase(&model, input))
    }
}
