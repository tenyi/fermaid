//! Parts shared by several diagram types.

pub(crate) mod fit;
#[cfg(any(feature = "class", feature = "er"))]
pub(crate) mod generics;
#[cfg(test)]
pub(crate) mod golden;
#[cfg(feature = "sequence")]
pub(crate) mod literal;
#[cfg(any(
    feature = "state",
    feature = "class",
    feature = "er",
    feature = "requirement"
))]
pub(crate) mod ordered;
#[cfg(feature = "kanban")]
pub(crate) mod outline;
#[cfg(any(feature = "timeline", feature = "journey"))]
pub(crate) mod plain;
#[cfg(any(feature = "timeline", feature = "journey", feature = "kanban"))]
#[cfg_attr(
    not(any(feature = "timeline", feature = "journey")),
    allow(dead_code, reason = "outlines use only part of it")
)]
pub(crate) mod scan;
#[cfg(feature = "pie")]
pub(crate) mod tokens;
