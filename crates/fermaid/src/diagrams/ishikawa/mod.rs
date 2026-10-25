//! Ishikawa diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The ishikawa diagram type.
pub(crate) struct Ishikawa;

impl Diagram for Ishikawa {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"ishikawa\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
