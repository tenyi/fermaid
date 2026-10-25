//! Architecture diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The architecture diagram type.
pub(crate) struct Architecture;

impl Diagram for Architecture {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"architecture\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
