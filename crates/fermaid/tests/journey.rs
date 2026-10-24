//! User journey rendering, security, limits and speed.
#![cfg(feature = "journey")]

mod support;

use std::time::{Duration, Instant};

use fermaid::{DiagramKind, ErrorCode, Limits, RenderOptions};
use support::{PAYLOADS, Random, assert_inert, mutate};

const FULL: &str = "journey\n title My working day\n section Go to work\n Make tea: 5: Me\n Go upstairs: 3: Me, Cat\n Do work: 1: Me, Cat, Dog\n section Go home\n Go downstairs: 5: Me\n Sit down: x\n Supercalifragilisticexpialidocious_is_a_very_long_word: 2: Bird\n";

fn render(src: &str) -> fermaid::Rendered {
    fermaid::render(src, &RenderOptions::default()).unwrap_or_else(|e| panic!("{src:?}: {e}"))
}

fn svg(src: &str) -> String {
    render(src).svg
}

fn count(svg: &str, needle: &str) -> usize {
    svg.matches(needle).count()
}

fn try_render(src: &str) -> Option<String> {
    fermaid::render(src, &RenderOptions::default())
        .ok()
        .map(|r| r.svg)
}

#[test]
fn draws_tasks_scores_and_people() {
    let out = render(FULL);
    assert_eq!(out.kind, DiagramKind::Journey);
    assert_eq!(count(&out.svg, "fermaid-node fermaid-task"), 6);
    assert_eq!(count(&out.svg, "fermaid-cluster fermaid-section"), 2);
    assert_eq!(count(&out.svg, "fermaid-face-happy"), 2);
    assert_eq!(count(&out.svg, "fermaid-face-sad"), 2);
    assert_eq!(count(&out.svg, "fermaid-legend-item"), 4);
    for needle in [
        ">Make tea<",
        ">Go to work<",
        "<title>Cat</title>",
        "fermaid-title",
    ] {
        assert!(out.svg.contains(needle), "{needle}");
    }
    assert!(out.links.is_empty());
}

#[test]
fn shows_markup_as_text() {
    let svg = svg("journey\n <b>bold</b> :fa-car: 3: <i>me</i>\n");
    assert!(svg.contains("&lt;b&gt;bold&lt;/b&gt;"));
    assert!(!svg.contains("<b>") && !svg.contains("<i>"));
}

#[test]
fn reports_errors_with_positions() {
    let err = fermaid::render("journey\n Make tea\n", &RenderOptions::default()).unwrap_err();
    assert_eq!(err.code, ErrorCode::Syntax);
    assert_eq!(err.kind, Some(DiagramKind::Journey));
    assert_eq!(err.line, 2);
}

#[test]
fn fits_a_narrow_host() {
    let src = "journey\n a: 1: x\n b: 2: y\n c: 3\n d: 4\n e: 5\n";
    let wide = render(src);
    let options = RenderOptions {
        max_width: Some(320.0),
        ..RenderOptions::default()
    };
    let narrow = fermaid::render(src, &options).unwrap();
    assert!(narrow.width < wide.width);
}

#[test]
fn output_is_deterministic() {
    assert_eq!(svg(FULL), svg(FULL));
}

#[test]
fn is_inert_for_hostile_and_mutated_inputs() {
    let mut rendered = 0;
    for payload in PAYLOADS {
        let p = *payload;
        for src in [
            format!("journey\n title {p}\n a: 3: x"),
            format!("journey\n {p}: 3: {p}"),
            format!("journey\n section {p}\n a: {p}: {p}, {p}"),
            format!("journey\n accTitle: {p}\n accDescr: {p}\n a: 1"),
            format!("---\ntitle: \"{p}\"\n---\njourney\n a: 2"),
        ] {
            let Some(svg) = try_render(&src) else {
                continue;
            };
            assert_inert(&svg, &src);
            rendered += 1;
        }
    }
    assert!(rendered > 50, "only {rendered} hostile inputs rendered");
    let mut fragments: Vec<&str> = PAYLOADS.to_vec();
    fragments.extend([
        "section ",
        "title ",
        " : ",
        ":",
        "timeline TD",
        "<br>",
        "\n",
    ]);
    let corpus = [FULL];
    let mut rnd = Random::new(11);
    for _ in 0..1500 {
        let src = mutate(&corpus, &fragments, &mut rnd);
        if let Some(svg) = try_render(&src) {
            assert_inert(&svg, &src);
        }
    }
}

fn repeat(n: usize, line: impl Fn(usize) -> String) -> String {
    (0..n).map(line).collect()
}

#[test]
fn stays_fast_on_large_and_hostile_input() {
    // Debug builds are several times slower than release.
    let budget = Duration::from_secs(if cfg!(debug_assertions) { 30 } else { 3 });
    let options = RenderOptions {
        limits: Limits {
            source: usize::MAX,
            output: usize::MAX,
            ..Limits::default()
        },
        ..RenderOptions::default()
    };
    const N: usize = 50000;
    let head = "journey\n";
    let cases = [
        format!("{head}{}", "x".repeat(N)),
        format!("{head}{}", "x ".repeat(N / 2)),
        format!("{head}a: 3: {}", "x,".repeat(N / 2)),
        format!("{head}{}", repeat(4000, |i| format!("t: 3: p{i}\n"))),
        format!("{head}{}", "a: 3: me\n".repeat(N / 9)),
        format!("{head}{}", "section s\na: 1\n".repeat(N / 15)),
        format!("{head}{}", "%%".repeat(N / 2)),
        format!("{head}accDescr {{{}", "x ".repeat(N / 2)),
        format!("{head}title {}", "x".repeat(N)),
    ];
    for src in &cases {
        let started = Instant::now();
        let _ = fermaid::render(src, &options);
        let took = started.elapsed();
        assert!(
            took < budget,
            "{:?} took {took:?}",
            &src[..src.len().min(40)]
        );
    }
}

#[test]
fn snapshots() {
    let cases = [
        ("full", FULL.to_string()),
        ("titled", format!("---\ntitle: Day\n---\n{FULL}")),
    ];
    for (name, src) in cases {
        insta::assert_snapshot!(name, svg(&src));
    }
}
