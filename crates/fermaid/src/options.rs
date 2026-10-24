use crate::DiagramKind;
use crate::text::TextMeasurer;

/// Caps that protect a host from untrusted diagram text. A diagram's own configuration cannot
/// raise them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// The longest source accepted, in characters. Mermaid's default `maxTextSize`.
    pub source: usize,
    /// The longest SVG returned, in characters. A source within its limit can still describe
    /// a drawing of many megabytes, which would stall the page it is put into.
    pub output: usize,
    /// The most edges a diagram may have. `A & B --> C & D` multiplies, so a few kilobytes of
    /// source can ask for millions.
    pub max_edges: usize,
    /// The deepest nesting accepted, such as subgraphs within subgraphs. A stack overflow
    /// aborts the process in Rust, so recursion is bounded explicitly.
    pub max_depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            source: 50_000,
            output: 4_000_000,
            max_edges: 5_000,
            max_depth: 64,
        }
    }
}

/// How a diagram is rendered. Every field has a default, so set only what you need:
///
/// ```
/// let options = fermaid::RenderOptions {
///     font_size: Some(14.0),
///     ..Default::default()
/// };
/// ```
pub struct RenderOptions {
    /// The fonts labels are drawn in where the page sets no `--fermaid-font`. Text is measured
    /// with `measurer` regardless.
    pub font_family: Option<String>,
    /// Like `font_family`, for monospaced text and `--fermaid-font-mono`.
    pub font_family_mono: Option<String>,
    /// Font size in pixels. Each diagram type has its own default, usually 16.
    pub font_size: Option<f64>,
    /// Prefix of the ids in the SVG, so that several diagrams can share a page.
    pub id_prefix: String,
    /// Space around the drawing in pixels. Each diagram type has its own default.
    pub padding: Option<f64>,
    /// Whether the SVG scales down to fit its container.
    pub responsive: bool,
    pub limits: Limits,
    /// Measures text; [`MetricsMeasurer`](crate::MetricsMeasurer) when `None`.
    pub measurer: Option<Box<dyn TextMeasurer>>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            font_family: None,
            font_family_mono: None,
            font_size: None,
            id_prefix: "fermaid".to_string(),
            padding: None,
            responsive: true,
            limits: Limits::default(),
            measurer: None,
        }
    }
}

/// A link a diagram's `click` statements or link syntax point to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The id of the element the link is on, as written in the diagram.
    pub id: String,
    pub href: String,
    /// Whether the link points into the page rather than away from it.
    pub internal: bool,
}

/// A rendered diagram.
#[derive(Debug, Clone, PartialEq)]
pub struct Rendered {
    pub svg: String,
    pub width: f64,
    pub height: f64,
    pub links: Vec<Link>,
    pub kind: DiagramKind,
}
