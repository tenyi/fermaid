//! The diagram types, one module each behind its own Cargo feature.

#[cfg(any(feature = "pie", feature = "flowchart", feature = "sequence"))]
pub(crate) mod common;
#[cfg(feature = "flowchart")]
pub(crate) mod flowchart;
#[cfg(feature = "pie")]
pub(crate) mod pie;
#[cfg(feature = "sequence")]
pub(crate) mod sequence;
