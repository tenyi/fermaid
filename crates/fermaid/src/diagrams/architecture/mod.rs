//! Architecture diagrams: groups of services, drawn as boxes with the edges between them.

mod db;
mod icons;
mod layout;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct Architecture;

impl Diagram for Architecture {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let ast = parser::parse_architecture(&input.source)?;
        let model = db::build_architecture(&ast, input.title)?;
        render::render_architecture(&model, input)
    }
}
