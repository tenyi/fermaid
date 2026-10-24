//! The diagram types, one module each behind its own Cargo feature.

#[cfg(feature = "class")]
pub(crate) mod class;
#[cfg(any(
    feature = "pie",
    feature = "flowchart",
    feature = "sequence",
    feature = "state",
    feature = "class",
    feature = "er",
    feature = "requirement",
    feature = "timeline",
    feature = "journey",
    feature = "kanban",
    feature = "mindmap"
))]
pub(crate) mod common;
#[cfg(feature = "er")]
pub(crate) mod er;
#[cfg(feature = "flowchart")]
pub(crate) mod flowchart;
#[cfg(feature = "journey")]
pub(crate) mod journey;
#[cfg(feature = "kanban")]
pub(crate) mod kanban;
#[cfg(feature = "mindmap")]
pub(crate) mod mindmap;
#[cfg(feature = "pie")]
pub(crate) mod pie;
#[cfg(feature = "requirement")]
pub(crate) mod requirement;
#[cfg(feature = "sequence")]
pub(crate) mod sequence;
#[cfg(feature = "state")]
pub(crate) mod state;
#[cfg(feature = "timeline")]
pub(crate) mod timeline;
