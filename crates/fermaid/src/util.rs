//! Character classes and helpers that follow JavaScript semantics, so that logic ported from
//! pele treats text the way it does.

/// Whitespace as JavaScript's `\s` and `String.prototype.trim` see it.
pub(crate) fn is_space(c: char) -> bool {
    matches!(
        c,
        '\t'..='\r'
            | ' '
            | '\u{a0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
            | '\u{feff}'
    )
}

/// A character of `\w`.
pub(crate) fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// A line terminator: what `.` does not match.
pub(crate) fn is_line_end(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

pub(crate) fn trim(s: &str) -> &str {
    s.trim_matches(is_space)
}

pub(crate) fn trim_start(s: &str) -> &str {
    s.trim_start_matches(is_space)
}

pub(crate) fn trim_end(s: &str) -> &str {
    s.trim_end_matches(is_space)
}

/// `Math.round`: halves round up, toward positive infinity.
pub(crate) fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// A number as JavaScript's `String(n)` writes it, for the values a diagram's text can hold.
pub(crate) fn js_number(n: f64) -> String {
    if n.is_nan() {
        "NaN".to_string()
    } else if n.is_infinite() {
        if n > 0.0 { "Infinity" } else { "-Infinity" }.to_string()
    } else if n == 0.0 {
        "0".to_string()
    } else {
        format!("{n}")
    }
}

/// Text slices kept as `char`s, so that ported index arithmetic never lands inside a character.
pub(crate) mod chars {
    use super::is_space;

    pub(crate) fn trim(s: &[char]) -> &[char] {
        trim_end(trim_start(s))
    }

    pub(crate) fn trim_start(s: &[char]) -> &[char] {
        let n = s.iter().take_while(|&&c| is_space(c)).count();
        &s[n..]
    }

    pub(crate) fn trim_end(s: &[char]) -> &[char] {
        let n = s.iter().rev().take_while(|&&c| is_space(c)).count();
        &s[..s.len() - n]
    }

    pub(crate) fn starts_with(s: &[char], at: usize, pat: &[char]) -> bool {
        s.get(at..).is_some_and(|rest| rest.starts_with(pat))
    }

    /// `indexOf(pat, from)`.
    pub(crate) fn find(s: &[char], pat: &[char], from: usize) -> Option<usize> {
        (from..=s.len().checked_sub(pat.len())?).find(|&i| s[i..].starts_with(pat))
    }

    /// `lastIndexOf(pat, from)`: the last match that starts at or before `from`.
    pub(crate) fn rfind(s: &[char], pat: &[char], from: usize) -> Option<usize> {
        let last = s.len().checked_sub(pat.len())?;
        (0..=from.min(last))
            .rev()
            .find(|&i| s[i..].starts_with(pat))
    }

    pub(crate) fn string(s: &[char]) -> String {
        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_halves_up() {
        assert_eq!(round(2.5), 3.0);
        assert_eq!(round(-2.5), -2.0);
        assert_eq!(round(-2.6), -3.0);
    }

    #[test]
    fn trims_javascript_whitespace() {
        assert_eq!(trim("\u{feff}\u{3000} a b\u{2028}"), "a b");
        assert_eq!(trim("\u{85}a"), "\u{85}a");
    }

    #[test]
    fn writes_numbers_like_javascript() {
        assert_eq!(js_number(3.0), "3");
        assert_eq!(js_number(-0.0), "0");
        assert_eq!(js_number(1.5), "1.5");
        assert_eq!(js_number(f64::NEG_INFINITY), "-Infinity");
    }

    #[test]
    fn finds_char_patterns() {
        let s: Vec<char> = "a--b--".chars().collect();
        let pat = ['-', '-'];
        assert_eq!(chars::find(&s, &pat, 0), Some(1));
        assert_eq!(chars::find(&s, &pat, 2), Some(4));
        assert_eq!(chars::rfind(&s, &pat, 3), Some(1));
        assert_eq!(chars::rfind(&s, &pat, 0), None);
    }
}
