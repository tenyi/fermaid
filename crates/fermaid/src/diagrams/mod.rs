//! The diagram types, one module each behind its own Cargo feature.

#[cfg(feature = "class")]
pub(crate) mod class;
#[cfg(any(
    feature = "pie",
    feature = "flowchart",
    feature = "sequence",
    feature = "state",
    feature = "class"
))]
pub(crate) mod common;
#[cfg(feature = "flowchart")]
pub(crate) mod flowchart;
#[cfg(feature = "pie")]
pub(crate) mod pie;
#[cfg(feature = "sequence")]
pub(crate) mod sequence;
#[cfg(feature = "state")]
pub(crate) mod state;
