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
    /// The most edges a flowchart may have. `A & B --> C & D` multiplies, so a few kilobytes of
    /// source can ask for millions. The other diagram types have no edge limit of their own, as
    /// pele has none for them either.
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
    /// The width the host has for the diagram, in pixels. A type that can draw itself narrower
    /// does, so that its text is not shrunk with it.
    pub max_width: Option<f64>,
    /// Whether a diagram that runs across may be drawn running down when it does not fit
    /// `max_width`, which is done under `direction_breakpoint` pixels (640 when `None`).
    pub auto_direction: bool,
    pub direction_breakpoint: Option<f64>,
    /// Space around the drawing in pixels. Each diagram type has its own default.
    pub padding: Option<f64>,
    /// Whether the SVG scales down to fit its container.
    pub responsive: bool,
    pub limits: Limits,
    /// Schemes a link may use besides a relative address. `None` allows `http`, `https`,
    /// `mailto` and `tel`; an empty list allows only relative links.
    pub link_schemes: Option<Vec<String>>,
    /// Schemes an image may use besides a relative address. `None` allows `http` and `https`.
    pub image_schemes: Option<Vec<String>>,
    /// The `rel` attribute of links. `None` writes `noopener`; an empty string writes none.
    pub link_rel: Option<String>,
    /// Measures text; [`MetricsMeasurer`](crate::MetricsMeasurer) when `None`.
    pub measurer: Option<Box<dyn TextMeasurer>>,
    /// The current time, in milliseconds since 1970-01-01 UTC, for what a gantt chart dates
    /// from today. `None` is 1970-01-01, so that output never depends on the clock.
    pub now: Option<f64>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            font_family: None,
            font_family_mono: None,
            font_size: None,
            id_prefix: "fermaid".to_string(),
            max_width: None,
            auto_direction: true,
            direction_breakpoint: None,
            padding: None,
            responsive: true,
            limits: Limits::default(),
            link_schemes: None,
            image_schemes: None,
            link_rel: None,
            measurer: None,
            now: None,
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
