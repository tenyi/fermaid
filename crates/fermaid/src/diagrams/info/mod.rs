//! Information diagrams: the version of the library, under the title of the diagram.

mod db;
mod parser;
mod render;
mod tokens;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

/// The info diagram type.
pub(crate) struct Info;

impl Diagram for Info {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let ast = parser::parse_info(&input.source)?;
        let model = db::build_info(&ast, input.title);
        Ok(render::render_info(&model, input))
    }
}
