//! The CSS custom properties a diagram is themed with.

/// Private aliases set once on the root element, so each use is short and the public
/// `--fermaid-*` tokens keep their fallbacks. The text color is set as well: an icon is drawn
/// in currentColor, which would otherwise be whatever color the page gives the container.
pub(crate) const ROOT_STYLE: &str = concat!(
    "--_bg:var(--fermaid-bg,#fff);--_fg:var(--fermaid-fg,#1f1f1f);--_m:var(--fermaid-muted,#6e6e6e);",
    "--_l:var(--fermaid-line,#1f1f1f);--_s:var(--fermaid-surface,#f3f3f3);--_a:var(--fermaid-surface-alt,#e6e6e6);",
    "--_b:var(--fermaid-border,#8a8a8a);--_c:var(--fermaid-accent,#5b7bd5);color:var(--_fg)",
);

pub(crate) const FONT_MONO: &str = "var(--_fm)";
pub(crate) const RADIUS: &str = "var(--fermaid-radius,4px)";

fn is_family_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

/// Matches one font family name at the start of `s`: quoted, or a run of letters, digits,
/// `_`, `-`, `.` and spaces. Returns the rest of `s`.
fn family(s: &str) -> Option<&str> {
    let first = s.chars().next()?;
    if first == '"' || first == '\'' {
        let body = &s[1..];
        let end = body.find(|c: char| "\"'\\\n<>{}();:@".contains(c))?;
        return body[end..].strip_prefix(first);
    }
    if !is_family_char(first) {
        return None;
    }
    Some(s.trim_start_matches(|c: char| is_family_char(c) || c == ' ' || c == '.'))
}

/// Whether `list` is a list of font names, quoted or not, with nothing that could end the
/// declaration it is put in.
fn is_family_list(list: &str) -> bool {
    let mut rest = crate::util::trim_start(list);
    loop {
        let Some(after) = family(rest) else {
            return false;
        };
        rest = crate::util::trim_start(after);
        match rest.strip_prefix(',') {
            Some(next) => rest = crate::util::trim_start(next),
            None => return rest.is_empty(),
        }
    }
}

fn families<'a>(list: Option<&'a str>, generic: &'a str) -> &'a str {
    list.filter(|l| is_family_list(l)).unwrap_or(generic)
}

/// The fonts the labels were measured in are the ones they are drawn in, unless the page names
/// others. The rest of what text inherits is pinned, as none of it was measured.
pub(crate) fn font_style(size: f64, family: Option<&str>, mono: Option<&str>) -> String {
    format!(
        ";--_fm:var(--fermaid-font-mono,{});font:{}px sans-serif;\
         letter-spacing:normal;word-spacing:normal;text-transform:none;direction:ltr;\
         font-family:var(--fermaid-font,{})",
        families(mono, "monospace"),
        super::num(size),
        families(family, "sans-serif"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_font_lists() {
        assert!(is_family_list("Inter"));
        assert!(is_family_list(
            " \"Noto Sans TC\", 'PingFang TC' , sans-serif "
        ));
        assert!(is_family_list("Font 1.5, x"));
        assert!(is_family_list("\"a, b\""));
    }

    #[test]
    fn rejects_unsafe_font_lists() {
        assert!(!is_family_list(""));
        assert!(!is_family_list("Inter;color:red"));
        assert!(!is_family_list("x, "));
        assert!(!is_family_list("\"x\\\""));
        assert!(!is_family_list("url(x)"));
        assert!(!is_family_list("'a\"b'"));
    }

    #[test]
    fn writes_font_style() {
        assert_eq!(
            font_style(16.0, Some("x;y"), None),
            ";--_fm:var(--fermaid-font-mono,monospace);font:16px sans-serif;letter-spacing:normal;\
             word-spacing:normal;text-transform:none;direction:ltr;font-family:var(--fermaid-font,sans-serif)"
        );
    }
}
