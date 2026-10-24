//! Building blocks for SVG output: escaping, number formatting, labels and the document root.
//! Ported from pele's `svg/` modules.

mod document;
mod theme;
mod title;

pub(crate) use document::{Accessible, svg_document};
pub(crate) use theme::{FONT_MONO, ROOT_STYLE, font_style};
pub(crate) use title::{title_label, with_title};

use std::fmt::Write;

use crate::text::Style;
use crate::text::entities::decode_entities;
use crate::text::label::Label;

/// Escapes text for XML and drops the control characters XML does not allow.
pub(crate) fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\0'..='\x08' | '\x0b' | '\x0c' | '\x0e'..='\x1f' | '\x7f' => {}
            c => out.push(c),
        }
    }
    out
}

/// Escapes text that is only ever read as text: a tooltip, an accessible name, a `data-id`.
/// Entity codes are turned into their characters first. A value that was checked before it is
/// written, such as a style, a colour or a URL, must go through [`esc`] instead, so that what
/// was checked is what is written.
pub(crate) fn esc_text(text: &str) -> String {
    esc(&decode_entities(text))
}

/// Writes a coordinate with at most two decimals and no trailing zeros.
pub(crate) fn num(value: f64) -> String {
    let n = crate::util::round(value * 100.0);
    if n == 0.0 || !n.is_finite() {
        return "0".to_string();
    }
    if n.abs() >= 1e11 {
        return crate::util::js_number(n / 100.0);
    }
    let neg = n < 0.0;
    let m = n.abs() as u64;
    let (q, r) = (m / 100, m % 100);
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    let _ = write!(out, "{q}");
    if r != 0 {
        if r % 10 == 0 {
            let _ = write!(out, ".{}", r / 10);
        } else {
            let _ = write!(out, ".{r:02}");
        }
    }
    out
}

const BOLD: &str = " font-weight=\"var(--_w)\"";
const ITALIC: &str = " font-style=\"italic\"";
const UNDERLINE: &str = " text-decoration=\"underline\"";
const MONO: &str = " font-family=\"var(--_fm)\"";
const SUB: &str = " baseline-shift=\"sub\" font-size=\"smaller\"";
const SUP: &str = " baseline-shift=\"super\" font-size=\"smaller\"";

pub(crate) fn span_style(style: Style) -> String {
    let mut out = String::new();
    for (flag, attrs) in [
        (Style::BOLD, BOLD),
        (Style::ITALIC, ITALIC),
        (Style::MONO, MONO),
        (Style::UNDERLINE, UNDERLINE),
    ] {
        if style.has(flag) {
            out.push_str(attrs);
        }
    }
    if style.has(Style::SUB) {
        out.push_str(SUB);
    } else if style.has(Style::SUP) {
        out.push_str(SUP);
    }
    out
}

/// The emphasis every span of a label has. It is written once, on the text element, where a
/// CSS rule for the label reaches it.
pub(crate) fn shared_style(label: &Label) -> Style {
    let mut shared = Style::BOLD | Style::ITALIC | Style::MONO;
    let mut any = false;
    for span in label.lines.iter().flatten() {
        if span.text.is_empty() {
            continue;
        }
        shared = shared & span.style;
        any = true;
    }
    if any { shared } else { Style::NONE }
}

/// Adds that emphasis to a text element's attributes, unless they give a weight of their own.
pub(crate) fn text_style(attrs: &str, shared: Style) -> String {
    let mut out = attrs.to_string();
    if shared.has(Style::BOLD) && !attrs.contains(" font-weight=\"") {
        out.push_str(BOLD);
    }
    if shared.has(Style::ITALIC) && !attrs.contains(" font-style=\"") {
        out.push_str(ITALIC);
    }
    if shared.has(Style::MONO) && !attrs.contains(" font-family=\"") {
        out.push_str(MONO);
    }
    out
}

/// Writes a measured label as SVG text centered on (cx, cy).
pub(crate) fn label_svg(label: &Label, cx: f64, cy: f64, attrs: &str) -> String {
    let lines = &label.lines;
    if lines.is_empty() {
        return String::new();
    }
    let lh = label.line_height;
    let shift = label.size * 0.35;
    let mut top = cy - label.height / 2.0;

    if let [line] = lines.as_slice() {
        if let [only] = line.as_slice() {
            if only.style.is_empty() {
                return format!(
                    "<text{attrs} x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                    num(cx),
                    num(top + lh / 2.0 + shift),
                    esc(&only.text)
                );
            }
        }
    }

    let shared = shared_style(label);
    let attrs = text_style(attrs, shared);
    let mut body = String::new();
    for line in lines {
        let y = top + lh / 2.0 + shift;
        for (j, span) in line.iter().enumerate() {
            let pos = if j == 0 {
                format!(" x=\"{}\" y=\"{}\"", num(cx), num(y))
            } else {
                String::new()
            };
            let _ = write!(
                body,
                "<tspan{pos}{}>{}</tspan>",
                span_style(span.style & !shared),
                esc(&span.text)
            );
        }
        if line.is_empty() {
            let _ = write!(body, "<tspan x=\"{}\" y=\"{}\"> </tspan>", num(cx), num(y));
        }
        top += lh;
    }
    format!("<text{attrs} text-anchor=\"middle\" xml:space=\"preserve\">{body}</text>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::MetricsMeasurer;
    use crate::text::label::layout_label;

    #[test]
    fn formats_numbers() {
        assert_eq!(num(0.0), "0");
        assert_eq!(num(0.004), "0");
        assert_eq!(num(1.0), "1");
        assert_eq!(num(1.5), "1.5");
        assert_eq!(num(1.25), "1.25");
        assert_eq!(num(1.05), "1.05");
        // -2.345 * 100 is -234.50000000000003 in binary floating point, as in pele.
        assert_eq!(num(-2.345), "-2.35");
        assert_eq!(num(0.005), "0.01");
        assert_eq!(num(-0.005), "0");
        assert_eq!(num(-0.5), "-0.5");
        assert_eq!(num(123456.789), "123456.79");
        // pele writes "NaN" here; fermaid writes a valid coordinate instead.
        assert_eq!(num(f64::NAN), "0");
    }

    #[test]
    fn escapes_markup_and_controls() {
        assert_eq!(
            esc("<a href=\"x\">&\u{1}\t\n</a>"),
            "&lt;a href=&quot;x&quot;&gt;&amp;\t\n&lt;/a&gt;"
        );
        assert_eq!(esc_text("#35;&lt;"), "#35;&lt;");
    }

    #[test]
    fn writes_labels() {
        let plain = layout_label(
            Some("A<B"),
            false,
            &MetricsMeasurer,
            16.0,
            1000.0,
            Style::NONE,
        );
        assert_eq!(
            label_svg(&plain, 50.0, 20.0, " class=\"t\""),
            "<text class=\"t\" x=\"50\" y=\"25.6\" text-anchor=\"middle\">A&lt;B</text>"
        );
        let styled = layout_label(
            Some("<b>x</b><br><b><i>y</i></b>"),
            false,
            &MetricsMeasurer,
            16.0,
            1000.0,
            Style::NONE,
        );
        assert_eq!(
            label_svg(&styled, 0.0, 0.0, ""),
            "<text font-weight=\"var(--_w)\" text-anchor=\"middle\" xml:space=\"preserve\">\
             <tspan x=\"0\" y=\"-4.9\">x</tspan><tspan x=\"0\" y=\"16.1\" font-style=\"italic\">y</tspan></text>"
        );
    }
}
