//! Wardley maps: a grid of axes with the components of a strategy placed on it.

mod db;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

/// The wardley diagram type.
pub(crate) struct Wardley;

impl Diagram for Wardley {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let ast = parser::parse_wardley(&input.source)?;
        let model = db::build_wardley(&ast, input.title)?;
        render::render_wardley(&model, input)
    }
}
