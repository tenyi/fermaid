//! Info diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The info diagram type.
pub(crate) struct Info;

impl Diagram for Info {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"info\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
