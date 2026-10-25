//! Packet diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The packet diagram type.
pub(crate) struct Packet;

impl Diagram for Packet {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"packet\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
