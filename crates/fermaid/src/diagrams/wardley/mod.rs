//! Wardley maps, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The wardley diagram type.
pub(crate) struct Wardley;

impl Diagram for Wardley {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"wardley\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
