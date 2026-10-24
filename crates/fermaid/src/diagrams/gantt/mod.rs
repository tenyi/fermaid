//! Gantt charts: tasks scheduled by date, in sections, on a time axis.
//!
//! Dates are read and drawn in UTC, so that output does not depend on the host's time zone,
//! and "today" is [`RenderOptions::now`](crate::RenderOptions::now).

mod axis;
mod dates;
mod db;
mod lexer;
mod parser;
mod render;

#[cfg(test)]
mod compat;

use crate::diagram::{Diagram, Input, Output};
use crate::error::Error;
use crate::preprocess::Value;

pub(crate) struct Gantt;

impl Diagram for Gantt {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        let mut db = db::GanttDb::new();
        if let Some(title) = input.title.filter(|t| !t.is_empty()) {
            db.title = Some(title.to_string());
        }
        if let Some(Value::Map(gantt)) = input.config.get("gantt") {
            if let Some(Value::String(mode)) = gantt.get("displayMode") {
                db.display_mode = mode.clone();
            }
        }
        parser::parse_gantt(&input.source, &mut db)?;
        render::render_gantt(&mut db, input)
    }
}
