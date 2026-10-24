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
#[cfg(any(feature = "timeline", feature = "journey"))]
pub(crate) mod plain;
#[cfg(any(feature = "timeline", feature = "journey"))]
pub(crate) mod scan;
#[cfg(feature = "pie")]
pub(crate) mod tokens;
