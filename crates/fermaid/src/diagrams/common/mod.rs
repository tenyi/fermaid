//! Parts shared by several diagram types.

pub(crate) mod fit;
#[cfg(all(test, any(feature = "flowchart", feature = "sequence")))]
pub(crate) mod golden;
#[cfg(feature = "sequence")]
pub(crate) mod literal;
#[cfg(feature = "pie")]
pub(crate) mod tokens;
