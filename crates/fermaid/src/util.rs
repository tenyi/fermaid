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

/// `text.replace(/\n\s+/g, '\n')`.
#[cfg_attr(not(any(feature = "state", feature = "class")), allow(dead_code))]
pub(crate) fn collapse_indent(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        out.push(c);
        if c == '\n' {
            while chars.peek().is_some_and(|&n| is_space(n)) {
                chars.next();
            }
        }
    }
    out
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

/// A number as JavaScript's `String(n)` writes it: the shortest digits that read back as `n`,
/// in exponent form below 1e-6 and from 1e21.
pub(crate) fn js_number(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_string();
    }
    if n.is_infinite() {
        return if n > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    if n == 0.0 {
        return "0".to_string();
    }
    // `{:e}` gives the same shortest digits as JavaScript, as `d.ddde±x`.
    let sci = format!("{:e}", n.abs());
    let (mantissa, exp) = sci.split_once('e').unwrap_or((&sci, "0"));
    let digits: String = mantissa.chars().filter(|&c| c != '.').collect();
    let exp: i32 = exp.parse().unwrap_or(0);
    let k = digits.len() as i32;
    let point = exp + 1;
    let mut out = String::new();
    if n < 0.0 {
        out.push('-');
    }
    if k <= point && point <= 21 {
        out.push_str(&digits);
        out.extend(std::iter::repeat_n('0', (point - k) as usize));
    } else if 0 < point && point <= 21 {
        out.push_str(&digits[..point as usize]);
        out.push('.');
        out.push_str(&digits[point as usize..]);
    } else if -6 < point && point <= 0 {
        out.push_str("0.");
        out.extend(std::iter::repeat_n('0', (-point) as usize));
        out.push_str(&digits);
    } else {
        out.push_str(&digits[..1]);
        if k > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        out.push(if exp < 0 { '-' } else { '+' });
        out.push_str(&exp.abs().to_string());
    }
    out
}

/// `n.toFixed(0)` for a finite `n` of at least 0: halves round up.
#[cfg_attr(not(feature = "pie"), allow(dead_code, reason = "used by pie charts"))]
pub(crate) fn to_fixed0(n: f64) -> String {
    let floor = n.floor();
    let rounded = if n - floor >= 0.5 { floor + 1.0 } else { floor };
    if rounded < 1e21 {
        format!("{rounded:.0}")
    } else {
        js_number(rounded)
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
        assert_eq!(js_number(-60.67), "-60.67");
        assert_eq!(js_number(123456789.0), "123456789");
        assert_eq!(js_number(1e20), "100000000000000000000");
        assert_eq!(js_number(1e21), "1e+21");
        assert_eq!(js_number(1.5e300), "1.5e+300");
        assert_eq!(js_number(0.000001), "0.000001");
        assert_eq!(js_number(1.25e-7), "1.25e-7");
        assert_eq!(js_number(0.1 + 0.2), "0.30000000000000004");
    }

    #[test]
    fn rounds_halves_up_to_integers() {
        assert_eq!(to_fixed0(12.5), "13");
        assert_eq!(to_fixed0(0.5), "1");
        assert_eq!(to_fixed0(0.49999999999999994), "0");
        assert_eq!(to_fixed0(78.99), "79");
        assert_eq!(to_fixed0(100.0), "100");
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
