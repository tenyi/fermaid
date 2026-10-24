//! Timelines: periods with their events, grouped into sections, running across or down.

mod db;
mod lexer;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;

pub(crate) struct Timeline;

impl Diagram for Timeline {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let mut db = db::TimelineDb::new();
        db.title = input.title.map(str::to_string);
        parser::parse_timeline(&input.source, &mut db)?;
        Ok(render::render_timeline(&db, input))
    }
}
