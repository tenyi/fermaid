//! Swimlane diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The swimlane diagram type.
pub(crate) struct Swimlane;

impl Diagram for Swimlane {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"swimlane\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
