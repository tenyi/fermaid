//! Tree views, which fermaid does not draw yet.

use crate::diagram::{Diagram, Input, Output};
use crate::error::{Error, ErrorCode};

/// The treeView diagram type.
pub(crate) struct TreeView;

impl Diagram for TreeView {
    fn render(&self, input: &Input<'_>) -> Result<Output, Error> {
        Err(Error::new(
            ErrorCode::Unsupported,
            "Diagram type \"treeView\" is not supported yet.",
        )
        .at(&input.source, 0))
    }
}
