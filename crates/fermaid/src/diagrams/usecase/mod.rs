//! Use case diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The usecase diagram type.
pub(crate) struct Usecase;

impl Diagram for Usecase {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"usecase\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
