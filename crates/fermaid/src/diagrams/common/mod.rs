//! Parts shared by several diagram types.

pub(crate) mod fit;
#[cfg(feature = "class")]
pub(crate) mod generics;
#[cfg(test)]
pub(crate) mod golden;
#[cfg(feature = "sequence")]
pub(crate) mod literal;
#[cfg(any(feature = "state", feature = "class"))]
pub(crate) mod ordered;
#[cfg(feature = "pie")]
pub(crate) mod tokens;
