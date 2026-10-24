//! Link and image addresses from diagram text. Ported from pele's `util/url.ts`, which follows
//! `@braintree/sanitize-url` as Mermaid does, and then keeps only the schemes the host allows.

use crate::svg::esc;
use crate::text::entities::decode_entities;
use crate::util::{trim, trim_start};
use crate::{Link, RenderOptions};

const BLANK: &str = "about:blank";

fn is_ctrl(c: char) -> bool {
    matches!(c, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' | '\u{2000}'..='\u{200d}' | '\u{feff}')
}

/// `decodeURIComponent`, or the text as it is when it is not valid percent-encoding.
fn decode(uri: &str) -> String {
    if !uri.contains('%') {
        return uri.to_string();
    }
    let b = uri.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            let hex = b
                .get(i + 1..i + 3)
                .and_then(|h| std::str::from_utf8(h).ok());
            match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                Some(v) => {
                    out.push(v);
                    i += 3;
                    continue;
                }
                None => return uri.to_string(),
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| uri.to_string())
}

/// `&#(\w+)(^\w|;)?` replaced by `String.fromCharCode(digits)`.
fn html_entities(s: &str) -> (String, bool) {
    let mut out = String::with_capacity(s.len());
    let mut found = false;
    let mut rest = s;
    while let Some(at) = rest.find("&#") {
        let after = &rest[at + 2..];
        let len = after
            .find(|c: char| !crate::util::is_word(c))
            .unwrap_or(after.len());
        if len == 0 {
            out.push_str(&rest[..at + 2]);
            rest = after;
            continue;
        }
        found = true;
        out.push_str(&rest[..at]);
        let code = after[..len].parse::<u32>().map_or(0, |n| n & 0xffff);
        out.push(char::from_u32(code).unwrap_or('\u{fffd}'));
        rest = &after[len..];
        if let Some(r) = rest.strip_prefix(';') {
            rest = r;
        }
    }
    out.push_str(rest);
    (out, found)
}

/// Removes `&newline;` and `&tab;`, ignoring case.
fn html_ctrl_entities(s: &str) -> (String, bool) {
    let lower = s.to_ascii_lowercase();
    let mut out = String::with_capacity(s.len());
    let mut found = false;
    let mut i = 0;
    while i < s.len() {
        if lower[i..].starts_with("&newline;") {
            i += 9;
            found = true;
        } else if lower[i..].starts_with("&tab;") {
            i += 5;
            found = true;
        } else {
            let c = s[i..].chars().next().unwrap_or(' ');
            out.push(c);
            i += c.len_utf8();
        }
    }
    (out, found)
}

/// Removes `(\\|%5[cC])((%(6[eE]|72|74))|[nrt])`.
fn ws_escapes(s: &str) -> (String, bool) {
    let mut out = String::with_capacity(s.len());
    let mut found = false;
    let mut i = 0;
    let lower = s.to_ascii_lowercase();
    while i < s.len() {
        let rest = &lower[i..];
        let lead = if rest.starts_with('\\') {
            1
        } else if rest.starts_with("%5c") {
            3
        } else {
            0
        };
        if lead > 0 {
            let tail = &rest[lead..];
            let skip =
                if tail.starts_with("%6e") || tail.starts_with("%72") || tail.starts_with("%74") {
                    3
                } else if tail.starts_with(['n', 'r', 't'])
                    && s[i + lead..].starts_with(['n', 'r', 't'])
                {
                    1
                } else {
                    0
                };
            if skip > 0 {
                i += lead + skip;
                found = true;
                continue;
            }
        }
        let c = s[i..].chars().next().unwrap_or(' ');
        out.push(c);
        i += c.len_utf8();
    }
    (out, found)
}

/// Follows the algorithm of `@braintree/sanitize-url`, which Mermaid applies to click targets.
pub(crate) fn sanitize_url(url: &str) -> String {
    if url.is_empty() {
        return BLANK.to_string();
    }
    let mut decoded = decode(trim(url));
    let mut rounds = 0;
    loop {
        rounds += 1;
        if rounds > 32 {
            return BLANK.to_string();
        }
        let s: String = decoded.chars().filter(|&c| !is_ctrl(c)).collect();
        let (s, _) = html_entities(&s);
        let (s, _) = html_ctrl_entities(&s);
        let s: String = s.chars().filter(|&c| !is_ctrl(c)).collect();
        let (s, _) = ws_escapes(&s);
        decoded = decode(trim(&s));
        let more = decoded.chars().any(is_ctrl)
            || html_entities(&decoded).1
            || html_ctrl_entities(&decoded).1
            || ws_escapes(&decoded).1;
        if !more {
            break;
        }
    }
    if decoded.is_empty() {
        return BLANK.to_string();
    }
    if decoded.starts_with(['.', '/']) {
        return decoded;
    }
    let trimmed = trim_start(&decoded);
    let first_line = trimmed.split('\n').next().unwrap_or("");
    // `^.+(:|&colon;)`: the first line up to its last colon.
    let colon = first_line.rfind(':').map(|i| i + 1);
    let entity = first_line.rfind("&colon;").map(|i| i + 7);
    let Some(end) = colon.max(entity).filter(|&e| e > 1) else {
        return decoded;
    };
    let protocol = trim(&first_line[..end]).to_lowercase();
    let bare = protocol.trim_start_matches(|c: char| !crate::util::is_word(c));
    if ["javascript", "data", "vbscript"]
        .iter()
        .any(|p| bare.starts_with(p))
    {
        return BLANK.to_string();
    }
    let normalized = trimmed.replace('\\', "/");
    if protocol == "mailto:" || protocol.contains("://") {
        return normalized;
    }
    if protocol == "http:" || protocol == "https:" {
        // What `new URL` would accept: a host after the scheme.
        let rest = normalized[protocol.len()..].trim_start_matches('/');
        if rest.is_empty() {
            return BLANK.to_string();
        }
        let host_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
        let path = &rest[host_end..];
        return format!(
            "{protocol}//{}{}",
            rest[..host_end].to_lowercase(),
            if path.is_empty() { "/" } else { path }
        );
    }
    normalized
}

/// A URL as written in a diagram: entity codes are read first, so they cannot hide a scheme.
pub(crate) fn safe_url(url: &str) -> String {
    sanitize_url(&decode_entities(url))
}

const LINK_SCHEMES: &[&str] = &["http", "https", "mailto", "tel"];
const IMAGE_SCHEMES: &[&str] = &["http", "https"];

/// Keeps a URL only when it is relative or its scheme is one of `schemes`.
fn allow(url: &str, schemes: &[&str]) -> Option<String> {
    let b = url.as_bytes();
    if b.len() >= 2 && matches!(b[0], b'\\' | b'/') && matches!(b[1], b'\\' | b'/') {
        return None;
    }
    let scheme_len = url
        .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-')))
        .unwrap_or(url.len());
    let has_scheme =
        scheme_len > 0 && b[0].is_ascii_alphabetic() && url[scheme_len..].starts_with(':');
    if !has_scheme {
        return Some(url.to_string());
    }
    let scheme = url[..scheme_len].to_ascii_lowercase();
    schemes
        .iter()
        .any(|s| s.eq_ignore_ascii_case(&scheme))
        .then(|| url.to_string())
}

fn schemes(list: &Option<Vec<String>>, default: &'static [&'static str]) -> Vec<String> {
    list.clone()
        .unwrap_or_else(|| default.iter().map(|s| (*s).to_string()).collect())
}

/// A link address the host allows, or `None`. A refused link is left out of the drawing.
pub(crate) fn link_url(url: &str, options: &RenderOptions) -> Option<String> {
    let list = schemes(&options.link_schemes, LINK_SCHEMES);
    allow(url, &list.iter().map(String::as_str).collect::<Vec<_>>())
}

pub(crate) fn image_url(url: &str, options: &RenderOptions) -> Option<String> {
    let list = schemes(&options.image_schemes, IMAGE_SCHEMES);
    allow(url, &list.iter().map(String::as_str).collect::<Vec<_>>())
}

pub(crate) fn rel_attr(options: &RenderOptions) -> String {
    match options.link_rel.as_deref() {
        Some("") => String::new(),
        Some(rel) => format!(" rel=\"{}\"", esc(rel)),
        None => " rel=\"noopener\"".to_string(),
    }
}

/// Wraps `body` in a link to `url`, when the host allows it, and records the link.
pub(crate) fn linked(
    body: &str,
    url: &str,
    id: &str,
    links: &mut Vec<Link>,
    options: &RenderOptions,
    attrs: &str,
) -> String {
    let Some(href) = link_url(url, options) else {
        return body.to_string();
    };
    let out = format!(
        "<a href=\"{}\"{attrs}{}>{body}</a>",
        esc(&href),
        rel_attr(options)
    );
    links.push(Link {
        id: id.to_string(),
        href,
        internal: false,
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_urls() {
        assert_eq!(sanitize_url("javascript:alert(1)"), BLANK);
        assert_eq!(sanitize_url(" JaVaScRiPt:alert(1)"), BLANK);
        assert_eq!(sanitize_url("jav&#x09;ascript:alert(1)"), BLANK);
        assert_eq!(sanitize_url("%0ajavascript:alert(1)"), BLANK);
        assert_eq!(sanitize_url("data:text/html,x"), BLANK);
        assert_eq!(
            sanitize_url("https://Example.com/a"),
            "https://example.com/a"
        );
        assert_eq!(sanitize_url("http://x"), "http://x/");
        assert_eq!(sanitize_url("/relative"), "/relative");
        assert_eq!(sanitize_url("page.html"), "page.html");
        assert_eq!(sanitize_url("mailto:a@b.c"), "mailto:a@b.c");
        let encoded = crate::text::entities::encode_entities("#106;avascript:alert(1)");
        assert_eq!(safe_url(&encoded), BLANK);
    }

    #[test]
    fn allows_only_listed_schemes() {
        let options = RenderOptions::default();
        assert_eq!(
            link_url("https://x/", &options).as_deref(),
            Some("https://x/")
        );
        assert_eq!(link_url("page", &options).as_deref(), Some("page"));
        assert_eq!(link_url("file:///etc/passwd", &options), None);
        assert_eq!(link_url("//evil/share", &options), None);
        assert_eq!(link_url("obsidian://open", &options), None);
        assert_eq!(image_url("mailto:x", &options), None);
    }
}
