//! XY charts: bars or a line over a category or a number axis, as Mermaid's `xychart` reads it.

mod db;
mod lexer;
mod parser;
mod render;
mod ticks;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct XyChart;

impl Diagram for XyChart {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let mut db = db::XyChartDb {
            title: input.title.map(str::to_string),
            ..Default::default()
        };
        parser::parse_xychart(&input.source, &mut db)?;
        render::render_xychart(&db, input)
    }
}
