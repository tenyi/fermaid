//! Cynefin diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The cynefin diagram type.
pub(crate) struct Cynefin;

impl Diagram for Cynefin {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"cynefin\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
