//! Ishikawa diagrams: the causes of an effect, drawn as a fishbone.

mod db;
mod lexer;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

/// The ishikawa diagram type.
pub(crate) struct Ishikawa;

impl Diagram for Ishikawa {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let ast = parser::parse_ishikawa(&input.source)?;
        let model = db::build_ishikawa(
            &ast,
            input.title.map(str::to_string),
            input.options.limits.max_depth,
        )?;
        Ok(render::render_ishikawa(&model, input))
    }
}
