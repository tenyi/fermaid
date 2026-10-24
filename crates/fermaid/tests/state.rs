//! State diagram rendering, security, limits and speed.
#![cfg(feature = "state")]

mod support;

use std::time::{Duration, Instant};

use fermaid::{DiagramKind, ErrorCode, Limits, RenderOptions};
use support::{PAYLOADS, Random, assert_inert, mutate};

struct Case {
    #[allow(dead_code)]
    file: &'static str,
    #[allow(dead_code)]
    name: &'static str,
    src: &'static str,
    #[allow(dead_code)]
    todo: bool,
}

include!("compat/state/cases.rs");

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

fn corpus() -> Vec<&'static str> {
    CASES
        .iter()
        .map(|c| c.src)
        .filter(|s| s.contains("stateDiagram"))
        .collect()
}

#[test]
fn draws_states_transitions_and_pseudo_states() {
    let out = render(
        "stateDiagram-v2\n [*] --> Still\n Still --> Moving : push\n Moving --> Still\n state c <<choice>>\n Moving --> c\n c --> Crash : fast\n c --> [*]\n state f <<fork>>\n Crash --> f\n",
    );
    assert_eq!(out.kind, DiagramKind::State);
    assert_eq!(count(&out.svg, "class=\"fermaid-node fermaid-state\""), 3);
    assert_eq!(count(&out.svg, "fermaid-state-start"), 1);
    assert_eq!(count(&out.svg, "fermaid-state-end"), 1);
    assert_eq!(count(&out.svg, "fermaid-state-choice"), 1);
    assert_eq!(count(&out.svg, "fermaid-state-fork"), 1);
    assert_eq!(count(&out.svg, "fermaid-transition"), 7);
    assert!(out.svg.contains(">push<"));
}

#[test]
fn draws_composites_regions_and_notes() {
    let svg = svg(
        "stateDiagram-v2\n state Active {\n direction LR\n [*] --> A\n A --> B\n --\n [*] --> C\n }\n Active --> Done\n note right of Done : all\n note left of Active\n two\n lines\n end note\n",
    );
    for needle in [
        "fermaid-state-composite",
        "fermaid-divider",
        "class=\"fermaid-note\"",
        "fermaid-note-edge",
        ">all<",
    ] {
        assert!(svg.contains(needle), "{needle}");
    }
    assert_inert(&svg, "composites");
}

#[test]
fn keeps_only_allowed_links_and_styles() {
    for url in [
        "javascript:alert(1)",
        " JaVaScRiPt:alert(1)",
        "data:text/html,x",
        "vbscript:x",
        "jav#9;ascript:alert(1)",
        "#106;avascript#colon;alert(1)",
    ] {
        let out = render(&format!(
            "stateDiagram-v2\n a --> b\n click a \"{url}\" \"t\"\n"
        ));
        assert!(!out.svg.contains("<a "), "{url}");
        assert!(out.links.is_empty());
    }
    let out = render("stateDiagram-v2\n a --> b\n click a \"https://example.com\" \"Docs\"\n");
    assert_eq!(out.links.len(), 1);
    assert!(out.svg.contains("<title>Docs</title>"));
    let svg = svg(
        "stateDiagram-v2\n a --> b\n classDef c fill:url(javascript:alert(1)),stroke:red\n class a c\n style b fill:expression(x)\n",
    );
    assert!(!svg.contains("javascript") && !svg.contains("expression"));
    assert!(svg.contains("stroke:red"));
}

#[test]
fn reports_errors_with_positions() {
    let err = fermaid::render(
        "stateDiagram-v2\n a --> b\n b -->\n",
        &RenderOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.code, ErrorCode::Syntax);
    assert_eq!(err.kind, Some(DiagramKind::State));
    assert_eq!(err.line, 3);
    let err = fermaid::render(
        "stateDiagram-v2\n state a {\n b\n }\n a : one\n a : two\n",
        &RenderOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.code, ErrorCode::Semantic);
}

#[test]
fn enforces_limits() {
    let deep = format!(
        "stateDiagram-v2\n{}x\n{}",
        "state s {\n".repeat(100),
        "}\n".repeat(100)
    );
    assert_eq!(
        fermaid::render(&deep, &RenderOptions::default())
            .unwrap_err()
            .code,
        ErrorCode::Limit
    );
}

#[test]
fn turns_down_to_fit_a_narrow_host() {
    let src = "stateDiagram-v2\n direction LR\n [*] --> First\n First --> Second\n Second --> Third\n Third --> Fourth\n Fourth --> [*]\n";
    let wide = render(src);
    let options = RenderOptions {
        max_width: Some(320.0),
        ..RenderOptions::default()
    };
    let narrow = fermaid::render(src, &options).unwrap();
    assert!(narrow.width < wide.width);
    assert!(narrow.height > wide.height);
}

#[test]
fn output_is_deterministic() {
    let src = "stateDiagram-v2\n state a {\n x --> y\n --\n z\n }\n a --> a : again\n note right of a : n\n";
    assert_eq!(svg(src), svg(src));
}

#[test]
fn is_inert_for_corpus_hostile_and_mutated_inputs() {
    let corpus = corpus();
    assert!(corpus.len() > 50);
    for src in &corpus {
        if let Some(svg) = try_render(src) {
            assert_inert(&svg, src);
        }
    }
    let mut rendered = 0;
    for payload in PAYLOADS {
        let p = *payload;
        for src in [
            format!("stateDiagram-v2\n {p} --> b"),
            format!("stateDiagram-v2\n a --> {p}"),
            format!("stateDiagram-v2\n a : {p}"),
            format!("stateDiagram-v2\n a : title\n a : {p}\n a : {p}"),
            format!("stateDiagram-v2\n a --> b : {p}"),
            format!("stateDiagram-v2\n a --> a : {p}"),
            format!("stateDiagram-v2\n state \"{p}\" as a\n a --> b"),
            format!("stateDiagram-v2\n state \"x\" as a:{p}\n"),
            format!("stateDiagram-v2\n state {p} {{\n a --> b\n }}"),
            format!("stateDiagram-v2\n state \"{p}\" as c {{\n a --> b\n }}\n c --> c : {p}"),
            format!(
                "stateDiagram-v2\n state c {{\n a --> b : {p}\n --\n {p} --> d\n }}\n c --> a : {p}"
            ),
            format!("stateDiagram-v2\n state {p} <<fork>>\n [*] --> {p}"),
            format!("stateDiagram-v2\n a --> b\n note right of a : {p}"),
            format!("stateDiagram-v2\n a --> b\n note left of a\n {p}\n {p}\n end note"),
            format!("stateDiagram-v2\n a:::{p} --> b"),
            format!("stateDiagram-v2\n a --> b\n classDef c fill:{p}\n class a c"),
            format!(
                "stateDiagram-v2\n a --> b\n classDef c {p}:red,color:{p},font-family:{p},stroke:{p}\n class a c"
            ),
            format!("stateDiagram-v2\n a --> b\n style a,{p} color:{p},stroke:{p},font-size:{p}"),
            format!("stateDiagram-v2\n a --> b\n click a \"{p}\" \"{p}\"\n"),
            format!("stateDiagram-v2\n a --> b\n click a href \"{p}\"\n"),
            format!("stateDiagram-v2\n accTitle: {p}\n accDescr: {p}\n a --> b"),
            format!("stateDiagram-v2\n direction LR {p}\n a --> b"),
            format!(
                "---\ntitle: \"{p}\"\nconfig:\n  state:\n    nodeSpacing: \"{p}\"\n---\nstateDiagram-v2\n a --> b"
            ),
        ] {
            let Some(svg) = try_render(&src) else {
                continue;
            };
            assert_inert(&svg, &src);
            rendered += 1;
        }
    }
    assert!(rendered > 400, "only {rendered} hostile inputs rendered");
    let mut fragments: Vec<&str> = PAYLOADS.to_vec();
    fragments.extend([
        "-->",
        "[*]",
        "state ",
        "{",
        "}",
        "--",
        "\n",
        "note left of a : x",
        "<<fork>>",
        "<<choice>>",
        "classDef c fill:red",
        "class a c",
        ":::c",
        ": label",
        "direction LR",
        "end note",
    ]);
    let mut rnd = Random::new(9);
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
    let head = "stateDiagram-v2\n";
    let cases = [
        format!("{head}{}", "a direction XX ".repeat(N / 15)),
        format!("{head}{}", "direction TB direction lr ".repeat(N / 26)),
        format!(
            "{head}{}",
            ("direction".to_string() + &"\n".repeat(24)).repeat(N / 33)
        ),
        format!("{head}{}\"{}", "a ".repeat(N / 2), "b ".repeat(N / 2)),
        format!("{head}state {}", "<<fork> [[join] ".repeat(N / 16)),
        format!("{head}state {}", "x <<choice>> ".repeat(N / 13)),
        format!("{head}{}", "state ".repeat(N / 6)),
        format!("{head}state a{}b", " ".repeat(N)),
        format!("{head}state {}", "w ".repeat(N / 2)),
        format!(
            "{head}{}state {} x {{",
            "state a\n".repeat(2000),
            "w".repeat(N / 2)
        ),
        format!("{head}{}", "a%%{".repeat(N / 4)),
        format!("{head}note right of a\n{}", "\n ".repeat(N / 2)),
        format!("{head}note right of a\n{}", " end not\n".repeat(N / 9)),
        format!("{head}note left of{}: x", " ".repeat(N)),
        format!("{head}class a{}", ", ".repeat(N / 2)),
        format!("{head}a {}", ":x".repeat(N / 2)),
        format!("{head}{}{}", "-".repeat(N / 2), ":".repeat(N / 2)),
        format!("{head}accDescr {{{}", "x ".repeat(N / 2)),
        format!("{head}{}", repeat(5000, |i| format!("state s{i}{{\n}}\n"))),
        format!(
            "{head}state s{{\n{}z\n}}\n",
            repeat(8000, |i| format!("a{i}\n--\n"))
        ),
        format!("{head}{}", repeat(5000, |i| format!("n{i}-->n{}\n", i + 1))),
        format!("{head}{}", repeat(5000, |i| format!("c-->n{i}\n"))),
        format!(
            "{head}state f <<fork>>\n{}",
            repeat(5000, |i| format!("f-->n{i}\n"))
        ),
        format!("{head}{}{}", "a-->b\n".repeat(3000), "a-->a\n".repeat(3000)),
        format!("{head}{}", "[*]-->[*]\n".repeat(N / 12)),
        format!(
            "{head}{}",
            repeat(2500, |i| format!(
                "note {} of a:n\n",
                if i % 2 == 1 { "left" } else { "right" }
            ))
        ),
        format!(
            "{head}{}",
            repeat(2000, |i| format!("note right of s{i}:n\n"))
        ),
        format!("{head}{}", "a:x\n".repeat(N / 4)),
        format!("{head}a : {}", "word ".repeat(N / 5)),
        format!("{head}a --> b : {}", "#35;<br>x ".repeat(N / 10)),
        format!("{head}a\n{}", repeat(5000, |i| format!("class a c{i}\n"))),
        format!("{head}class {}z c\n", repeat(8000, |i| format!("s{i},"))),
        format!("{head}a-->b\n{}", "style a,b fill:red\n".repeat(3000)),
        format!("{head}a-->b\n{}", "click a \"u\" \"t\"\n".repeat(3000)),
        format!("{head}a\nclick a \"&{}x\" \"t\"\n", "#38;".repeat(N / 4)),
        // Composites redeclared inside each other: a chain of 2,000 groups, with no nesting in
        // the text.
        format!(
            "{head}{}",
            repeat(2000, |i| format!(
                "state a{i}{{\nstate a{}{{\n}}\n}}\n",
                (i + 1) % 2000
            ))
        ),
        format!(
            "{head}{}",
            repeat(1500, |i| format!(
                "state a{i}{{\nstate a{}{{\n}}\n}}\n",
                i + 1
            )) + "a1500 --> x\nx --> a0\n"
        ),
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
        (
            "basic",
            "stateDiagram-v2\n [*] --> Still\n Still --> [*]\n Still --> Moving\n Moving --> Still\n Moving --> Crash\n Crash --> [*]\n",
        ),
        (
            "composite",
            "stateDiagram-v2\n [*] --> Active\n state Active {\n [*] --> NumLockOff\n NumLockOff --> NumLockOn : EvNumLockPressed\n NumLockOn --> NumLockOff : EvNumLockPressed\n --\n [*] --> CapsLockOff\n CapsLockOff --> CapsLockOn : EvCapsLockPressed\n }\n",
        ),
        (
            "notes_and_choice",
            "---\ntitle: Choices\n---\nstateDiagram-v2\n direction LR\n state if_state <<choice>>\n [*] --> IsPositive\n IsPositive --> if_state\n if_state --> False : if n < 0\n if_state --> True : if n >= 0\n note right of True : positive\n classDef bad fill:#f00,color:white\n class False bad\n",
        ),
    ];
    for (name, src) in cases {
        insta::assert_snapshot!(name, svg(src));
    }
}
