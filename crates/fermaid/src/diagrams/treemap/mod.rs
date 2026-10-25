//! Treemaps, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The treemap diagram type.
pub(crate) struct Treemap;

impl Diagram for Treemap {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"treemap\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
