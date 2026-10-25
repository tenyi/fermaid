//! Venn diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The venn diagram type.
pub(crate) struct Venn;

impl Diagram for Venn {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"venn\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
