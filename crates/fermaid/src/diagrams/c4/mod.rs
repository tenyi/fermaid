//! C4 diagrams: the shapes, the boundaries that hold them, and the relations between them.

mod db;
mod layout;
mod lexer;
mod parser;
mod render;
mod route;
mod tokens;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct C4;

impl Diagram for C4 {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let actions = parser::parse_c4(&input.source)?;
        let model = db::build(actions);
        render::render_c4(&model, input)
    }
}
