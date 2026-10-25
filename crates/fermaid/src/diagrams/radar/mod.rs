//! Radar charts, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The radar diagram type.
pub(crate) struct Radar;

impl Diagram for Radar {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"radar\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
