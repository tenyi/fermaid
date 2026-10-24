//! Helpers of the compat tests that compare a model with pele's, as `tools/oracle/golden.mjs`
//! summarizes it.

use std::fmt::Write;

/// `JSON.stringify` of a string.
pub(crate) fn json(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A quoted string, or `-` for none.
pub(crate) fn q(s: Option<&str>) -> String {
    s.map_or("-".to_string(), json)
}

/// The test names a `SKIPPED.md` lists, as `` - `name`: reason `` lines under "Skipped tests".
pub(crate) fn skipped(text: &'static str) -> Vec<&'static str> {
    text.lines()
        .skip_while(|line| *line != "## Skipped tests")
        .take_while(|line| !line.starts_with("## Spec files"))
        .filter_map(|line| line.strip_prefix("- `")?.split_once("`:").map(|(n, _)| n))
        .collect()
}

/// The name of an error code, as pele's errors carry it.
pub(crate) fn error_code(code: crate::error::ErrorCode) -> &'static str {
    use crate::error::ErrorCode;
    match code {
        ErrorCode::Syntax => "syntax",
        ErrorCode::Semantic => "semantic",
        ErrorCode::Limit => "limit",
        ErrorCode::Unsupported => "unsupported-diagram",
    }
}
