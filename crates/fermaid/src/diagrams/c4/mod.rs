//! C4 diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The c4 diagram type.
pub(crate) struct C4;

impl Diagram for C4 {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"c4\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
