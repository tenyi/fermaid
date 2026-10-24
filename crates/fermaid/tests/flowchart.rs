//! Flowchart rendering, security, limits and speed.
#![cfg(feature = "flowchart")]

mod support;

use std::fmt::Write as _;
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

include!("compat/flowchart/cases.rs");

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
fn draws_nodes_edges_and_labels() {
    let out =
        render("flowchart TD\n A[Start] -->|yes| B{Choose}\n B --> C((End))\n B -.-> D>Flag]\n");
    assert_eq!(out.kind, DiagramKind::Flowchart);
    assert_eq!(count(&out.svg, "class=\"fermaid-node "), 4);
    assert_eq!(count(&out.svg, "class=\"fermaid-edge\""), 3);
    assert_eq!(count(&out.svg, "class=\"fermaid-edge-label\""), 1);
    assert!(out.svg.contains("fermaid-shape-diam"));
    assert!(out.svg.contains("fermaid-shape-circle"));
    assert!(out.svg.contains("fermaid-shape-odd"));
    assert!(out.svg.contains("stroke-dasharray=\"3 4\""));
    assert!(out.svg.contains(">yes<"));
}

#[test]
fn draws_every_documented_shape() {
    for shape in [
        "rect",
        "rounded",
        "stadium",
        "fr-rect",
        "cyl",
        "circle",
        "dbl-circ",
        "diam",
        "hex",
        "lean-r",
        "lean-l",
        "trap-b",
        "trap-t",
        "text",
        "sm-circ",
        "f-circ",
        "fr-circ",
        "fork",
        "notch-rect",
        "lin-rect",
        "div-rect",
        "win-pane",
        "tri",
        "flip-tri",
        "sl-rect",
        "notch-pent",
        "hourglass",
        "delay",
        "h-cyl",
        "cross-circ",
        "st-rect",
        "doc",
        "flag",
        "datastore",
        "folder",
        "bucket",
        "console",
        "browser",
        "person",
        "bang",
        "cloud",
        "brace",
        "brace-r",
        "braces",
        "bolt",
        "lin-cyl",
        "curv-trap",
        "docs",
        "bow-rect",
        "tag-rect",
        "tag-doc",
        "lin-doc",
    ] {
        let svg = svg(&format!("flowchart LR\n A@{{ shape: {shape} }} --> B\n"));
        assert!(svg.contains(&format!("fermaid-shape-{shape}")), "{shape}");
        assert_inert(&svg, shape);
    }
    let err = fermaid::render(
        "flowchart LR\n A@{ shape: nope }",
        &RenderOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.code, ErrorCode::Semantic);
}

#[test]
fn draws_subgraphs_with_their_own_direction() {
    let svg = svg(
        "flowchart TB\n c1-->a2\n subgraph one\n a1-->a2\n end\n subgraph two\n direction LR\n b1-->b2\n end\n one --> two\n",
    );
    assert_eq!(count(&svg, "class=\"fermaid-cluster\""), 2);
    assert!(svg.contains(">one<") && svg.contains(">two<"));
}

#[test]
fn applies_classes_and_styles_safely() {
    let svg = svg(
        "flowchart LR\n A:::hot --> B\n classDef hot fill:#f96,stroke:#333,color:#fff\n style B fill:javascript:alert,stroke:red\n linkStyle 0 stroke:#0a0\n",
    );
    assert!(svg.contains("fill:#f96;stroke:#333;"));
    assert!(svg.contains("fermaid-node fermaid-shape-rect hot"));
    assert!(!svg.contains("javascript"));
    assert!(svg.contains("stroke:red;"));
    assert!(svg.contains("fill=\"#0a0\""));
}

#[test]
fn keeps_only_allowed_links() {
    let src = "flowchart LR\n A --> B --> C --> D\n click A \"https://example.com\" \"Tip\"\n click B \"javascript:alert(1)\"\n click C \"file:///etc/passwd\"\n click D \"/docs\" _blank\n";
    let out = render(src);
    assert!(
        out.svg
            .contains("<a href=\"https://example.com/\" rel=\"noopener\">")
    );
    assert!(out.svg.contains("<title>Tip</title>"));
    assert!(
        out.svg
            .contains("<a href=\"/docs\" target=\"_blank\" rel=\"noopener\">")
    );
    assert!(!out.svg.contains("javascript:") && !out.svg.contains("file:"));
    let hrefs: Vec<&str> = out.links.iter().map(|l| l.href.as_str()).collect();
    assert_eq!(hrefs, ["https://example.com/", "/docs"]);

    let options = RenderOptions {
        link_schemes: Some(vec![]),
        link_rel: Some(String::new()),
        ..RenderOptions::default()
    };
    let out = fermaid::render(src, &options).unwrap();
    assert_eq!(out.links.len(), 1);
    assert!(out.svg.contains("<a href=\"/docs\" target=\"_blank\">"));
}

#[test]
fn reports_errors_with_positions() {
    let err = fermaid::render(
        "flowchart TD\n A --> B\n B --> )\n",
        &RenderOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.code, ErrorCode::Syntax);
    assert_eq!(err.kind, Some(DiagramKind::Flowchart));
    assert_eq!((err.line, err.column), (3, 8));
    assert_eq!(err.snippet, " B --> )\n-------^");
    let err = fermaid::render(
        "flowchart TD\n A --> B\n linkStyle 5 stroke:red\n",
        &RenderOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.code, ErrorCode::Semantic);
}

#[test]
fn enforces_limits() {
    let options = RenderOptions {
        limits: Limits {
            max_edges: 10,
            ..Limits::default()
        },
        ..RenderOptions::default()
    };
    let wide = "flowchart TD\n a & b & c & d --> e & f & g\n";
    assert_eq!(
        fermaid::render(wide, &options).unwrap_err().code,
        ErrorCode::Limit
    );
    let deep = format!(
        "flowchart TD\n{}x\n{}",
        "subgraph s\n".repeat(100),
        "end\n".repeat(100)
    );
    assert_eq!(
        fermaid::render(&deep, &RenderOptions::default())
            .unwrap_err()
            .code,
        ErrorCode::Limit
    );
    let options = RenderOptions {
        limits: Limits {
            output: 1000,
            ..Limits::default()
        },
        ..RenderOptions::default()
    };
    assert_eq!(
        fermaid::render("flowchart TD\n A --> B --> C\n", &options)
            .unwrap_err()
            .code,
        ErrorCode::Limit
    );
}

#[test]
fn turns_down_to_fit_a_narrow_host() {
    let src =
        "flowchart LR\n A[First step] --> B[Second step] --> C[Third step] --> D[Fourth step]\n";
    let wide = render(src);
    let options = RenderOptions {
        max_width: Some(320.0),
        ..RenderOptions::default()
    };
    let narrow = fermaid::render(src, &options).unwrap();
    assert!(narrow.width < wide.width);
    assert!(narrow.height > wide.height);
    let fixed = RenderOptions {
        max_width: Some(320.0),
        auto_direction: false,
        ..RenderOptions::default()
    };
    assert!(fermaid::render(src, &fixed).unwrap().width > narrow.width);
}

#[test]
fn output_is_deterministic() {
    let src = "flowchart TD\n subgraph s\n a-->b & c\n end\n c-->d-->a\n d-.->|x| b\n";
    assert_eq!(svg(src), svg(src));
}

#[test]
fn is_inert_for_corpus_hostile_and_mutated_inputs() {
    let corpus: Vec<&str> = CASES
        .iter()
        .map(|c| c.src)
        .filter(|s| s.contains("graph") || s.contains("flowchart"))
        .collect();
    for src in &corpus {
        if let Some(svg) = try_render(src) {
            assert_inert(&svg, src);
        }
    }
    let mut rendered = 0;
    for payload in PAYLOADS {
        for src in [
            format!("flowchart TD\n A[\"{payload}\"] --> B"),
            format!("flowchart TD\n A -->|\"{payload}\"| B"),
            format!("flowchart TD\n subgraph s [\"{payload}\"]\n A\n end"),
            format!("flowchart TD\n A --> B\n click A \"{payload}\" \"{payload}\""),
            format!("flowchart TD\n A --> B\n click A href \"{payload}\""),
            format!("flowchart TD\n A --> B\n style A fill:{payload}"),
            format!("flowchart TD\n A:::c --> B\n classDef c color:{payload}"),
            format!("flowchart TD\n A@{{ img: \"{payload}\", label: \"x\" }}"),
            format!("flowchart TD\n A@{{ icon: \"{payload}\" }}"),
            format!("flowchart TD\n A[\"`**{payload}**`\"]"),
            format!(
                "---\ntitle: \"{payload}\"\n---\nflowchart TD\n accTitle: {payload}\n accDescr: {payload}\n A"
            ),
        ] {
            let Some(svg) = try_render(&src) else {
                continue;
            };
            assert_inert(&svg, &src);
            assert!(!svg.contains("<script"), "{src:?}");
            rendered += 1;
        }
    }
    assert!(rendered > 300, "only {rendered} hostile inputs rendered");
    let mut fragments: Vec<&str> = PAYLOADS.to_vec();
    fragments.extend([
        "-->",
        "---",
        "-.->",
        "==>",
        "[",
        "]",
        "(",
        ")",
        "{",
        "}",
        "|",
        "\n",
        "subgraph x\n",
        "end\n",
        ":::c",
        "&",
    ]);
    let mut rnd = Random::new(5);
    for _ in 0..1500 {
        let src = mutate(&corpus, &fragments, &mut rnd);
        if let Some(svg) = try_render(&src) {
            assert_inert(&svg, &src);
        }
    }
}

fn graph_with(edges: usize, nodes: usize, seed: u32) -> String {
    let mut rnd = Random::new(seed);
    let mut src = String::from("flowchart TD\n");
    for _ in 0..edges {
        let a = (rnd.next() * nodes as f64) as usize;
        let b = (rnd.next() * nodes as f64) as usize;
        let _ = writeln!(src, "n{a}-->n{b}");
    }
    src
}

#[test]
fn stays_fast_on_large_and_hostile_input() {
    // Debug builds are several times slower than release, where 5,000 edges take about 0.1 s.
    let budget = Duration::from_secs(if cfg!(debug_assertions) { 20 } else { 2 });
    let options = RenderOptions {
        limits: Limits {
            source: usize::MAX,
            output: usize::MAX,
            ..Limits::default()
        },
        ..RenderOptions::default()
    };
    let cases = [
        graph_with(5000, 1000, 1),
        format!(
            "flowchart LR\n{}",
            (0..2000).fold(String::new(), |mut s, i| {
                let _ = writeln!(s, "a{i}-->a{}", i + 1);
                s
            })
        ),
        format!(
            "flowchart TD\n{} --> {}\n",
            vec!["a"; 70].join(" & "),
            vec!["b"; 70].join(" & ")
        ),
        format!("flowchart TD\nA[\"{}\"]", "x ".repeat(25000)),
        format!("flowchart TD\n{}", "A-->".repeat(12000)),
        format!("flowchart TD\n{}", "%%{\n".repeat(12000)),
        format!("flowchart TD\n{}", "direction".repeat(5000)),
        format!("flowchart TD\n{}", "@".repeat(50000)),
        format!("flowchart TD\nA{}", " ".repeat(50000)),
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
            "decision",
            "flowchart TD\n A[Christmas] -->|Get money| B(Go shopping)\n B --> C{Let me think}\n C -->|One| D[Laptop]\n C -->|Two| E[iPhone]\n C -->|Three| F[Car]\n",
        ),
        (
            "subgraphs",
            "flowchart TB\n c1-->a2\n subgraph one\n a1-->a2\n end\n subgraph two\n b1-->b2\n end\n subgraph three\n c1-->c2\n end\n one --> two\n three --> two\n two --> c2\n",
        ),
        (
            "styles_and_links",
            "---\ntitle: Styled\n---\nflowchart LR\n A:::hot -- text --> B((Circle))\n B ==> C[(Database)]\n C -.-> D{{Hexagon}}\n D --o E>Flag] --x A\n classDef hot fill:#f96\n click B \"https://example.com\"\n",
        ),
        (
            "self_loop",
            "flowchart LR\n A --> A\n A -->|again| A\n A --> B\n",
        ),
    ];
    for (name, src) in cases {
        insta::assert_snapshot!(name, svg(src));
    }
}
