//! Event modeling diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The eventmodeling diagram type.
pub(crate) struct EventModeling;

impl Diagram for EventModeling {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"eventmodeling\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
