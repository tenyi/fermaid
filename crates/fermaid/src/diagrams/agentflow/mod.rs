//! Agent flow diagrams, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The agentflow diagram type.
pub(crate) struct Agentflow;

impl Diagram for Agentflow {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"agentflow\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
