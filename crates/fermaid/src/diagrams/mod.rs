//! The diagram types, one module each behind its own Cargo feature.

#[cfg(feature = "class")]
pub(crate) mod class;
#[cfg(any(
    feature = "pie",
    feature = "flowchart",
    feature = "sequence",
    feature = "state",
    feature = "class",
    feature = "er"
))]
pub(crate) mod common;
#[cfg(feature = "er")]
pub(crate) mod er;
#[cfg(feature = "flowchart")]
pub(crate) mod flowchart;
#[cfg(feature = "pie")]
pub(crate) mod pie;
#[cfg(feature = "sequence")]
pub(crate) mod sequence;
#[cfg(feature = "state")]
pub(crate) mod state;
