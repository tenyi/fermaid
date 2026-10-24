//! Git graphs: the history of a branch, with the commits on it and the branches it grows.

mod db;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct GitGraph;

impl Diagram for GitGraph {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let ast = parser::parse_git(&input.source)?;
        let model = db::build_git(&ast, input.config, input.title)?;
        render::render_git(&model, input)
    }
}
