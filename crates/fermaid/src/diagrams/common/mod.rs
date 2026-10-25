//! Parts shared by several diagram types.

#[cfg(any(feature = "xychart", feature = "quadrantchart", feature = "block"))]
pub(crate) mod chars;
pub(crate) mod fit;
#[cfg(any(feature = "class", feature = "er"))]
pub(crate) mod generics;
// The helpers of the golden tests are only used by the types that have a compat module.
#[cfg(all(
    test,
    any(
        feature = "class",
        feature = "er",
        feature = "flowchart",
        feature = "gantt",
        feature = "git",
        feature = "journey",
        feature = "kanban",
        feature = "mindmap",
        feature = "pie",
        feature = "quadrantchart",
        feature = "requirement",
        feature = "sankey",
        feature = "sequence",
        feature = "state",
        feature = "timeline",
        feature = "xychart",
        feature = "block"
    )
))]
pub(crate) mod golden;
#[cfg(feature = "sequence")]
pub(crate) mod literal;
#[cfg(any(
    feature = "state",
    feature = "class",
    feature = "er",
    feature = "requirement",
    feature = "sankey",
    feature = "quadrantchart",
    feature = "git",
    feature = "block"
))]
pub(crate) mod ordered;
#[cfg(any(feature = "kanban", feature = "mindmap"))]
pub(crate) mod outline;
#[cfg(any(feature = "timeline", feature = "journey", feature = "git"))]
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
#[cfg(any(feature = "pie", feature = "git"))]
pub(crate) mod tokens;
