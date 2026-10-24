//! Parts shared by several diagram types.

#[cfg(feature = "xychart")]
pub(crate) mod chars;
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
    feature = "requirement",
    feature = "sankey"
))]
pub(crate) mod ordered;
#[cfg(any(feature = "kanban", feature = "mindmap"))]
pub(crate) mod outline;
#[cfg(any(feature = "timeline", feature = "journey"))]
pub(crate) mod plain;
#[cfg(any(
    feature = "timeline",
    feature = "journey",
    feature = "kanban",
    feature = "mindmap",
    feature = "gantt"
))]
#[cfg_attr(
    not(any(feature = "timeline", feature = "journey")),
    allow(dead_code, reason = "outlines and gantt charts use only part of it")
)]
pub(crate) mod scan;
#[cfg(feature = "pie")]
pub(crate) mod tokens;
