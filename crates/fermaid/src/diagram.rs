//! The interface each diagram type implements, and the lookup from a detected type to the
//! implementation compiled in through its Cargo feature.

use crate::error::Error;
use crate::preprocess::Config;
use crate::text::TextMeasurer;
use crate::{DiagramKind, Link, RenderOptions};

/// What a diagram type is given to render.
#[cfg_attr(
    not(any(
        feature = "pie",
        feature = "flowchart",
        feature = "sequence",
        feature = "state",
        feature = "class",
        feature = "er",
        feature = "timeline",
        feature = "kanban",
        feature = "mindmap"
    )),
    allow(
        dead_code,
        reason = "read by the diagram types, none of which is compiled in"
    )
)]
pub(crate) struct Input<'a> {
    /// The text after preprocessing and entity encoding. It ends with a line break, as
    /// Mermaid's parsers expect.
    pub source: String,
    pub config: &'a Config,
    /// The title from front matter.
    pub title: Option<&'a str>,
    /// The number of front matter lines removed before `source`, for error positions.
    #[allow(
        dead_code,
        reason = "read by the types that keep comments, none of which is written yet"
    )]
    pub line_offset: usize,
    pub options: &'a RenderOptions,
    pub measurer: &'a dyn TextMeasurer,
}

pub(crate) struct Output {
    pub svg: String,
    pub width: f64,
    pub height: f64,
    pub links: Vec<Link>,
}

pub(crate) trait Diagram: Sync {
    /// Whether the parser is given the text with its `%%` comment lines kept, so that the
    /// positions it reports match the source.
    fn keep_comments(&self) -> bool {
        false
    }

    fn render(&self, input: &Input<'_>) -> Result<Output, Error>;
}

/// The implementation of a diagram type, when its feature is enabled.
pub(crate) fn lookup(kind: DiagramKind) -> Option<&'static dyn Diagram> {
    match kind {
        #[cfg(feature = "flowchart")]
        DiagramKind::Flowchart => Some(&crate::diagrams::flowchart::Flowchart),
        #[cfg(feature = "pie")]
        DiagramKind::Pie => Some(&crate::diagrams::pie::Pie),
        #[cfg(feature = "sequence")]
        DiagramKind::Sequence => Some(&crate::diagrams::sequence::Sequence),
        #[cfg(feature = "state")]
        DiagramKind::State => Some(&crate::diagrams::state::State),
        #[cfg(feature = "class")]
        DiagramKind::Class => Some(&crate::diagrams::class::Class),
        #[cfg(feature = "er")]
        DiagramKind::Er => Some(&crate::diagrams::er::Er),
        #[cfg(feature = "requirement")]
        DiagramKind::Requirement => Some(&crate::diagrams::requirement::Requirement),
        #[cfg(feature = "timeline")]
        DiagramKind::Timeline => Some(&crate::diagrams::timeline::Timeline),
        #[cfg(feature = "journey")]
        DiagramKind::Journey => Some(&crate::diagrams::journey::Journey),
        #[cfg(feature = "kanban")]
        DiagramKind::Kanban => Some(&crate::diagrams::kanban::Kanban),
        #[cfg(feature = "mindmap")]
        DiagramKind::Mindmap => Some(&crate::diagrams::mindmap::Mindmap),
        _ => None,
    }
}
