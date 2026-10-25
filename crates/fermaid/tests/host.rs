//! The icons and the math that the host of the library draws: the `icons` and `math` options.
//!
//! The resolvers here are fakes. What they answer is the host's and is put into the SVG as it
//! is, so these tests check what the library does around the answers: where it puts them, how
//! it makes room for them, and that the text of a diagram cannot reach the output but through
//! them.
#![cfg(feature = "flowchart")]

mod support;

use std::cell::RefCell;
use std::rc::Rc;

use fermaid::{IconResolver, MathMarkup, MathResolver, RenderOptions};
use support::{PAYLOADS, assert_inert};

const ICON: &str = "<path d=\"M2 2h20v20H2z\"/>";
const MATH: &str = "<path d=\"M0 0h30v40H0z\"/>";

fn icons() -> IconResolver {
    Box::new(|name| (name == "fa:fa-car").then(|| ICON.to_string()))
}

fn math(width: f64, height: f64) -> MathResolver {
    Box::new(move |_, _| {
        Some(MathMarkup::Svg {
            svg: MATH.to_string(),
            width,
            height,
        })
    })
}

fn svg(src: &str, options: &RenderOptions) -> String {
    fermaid::render(src, options)
        .unwrap_or_else(|e| panic!("{src:?}: {e}"))
        .svg
}

#[test]
fn without_resolvers_icons_are_left_out_and_math_stays_text() {
    let out = svg(
        "flowchart LR\n  A[fa:fa-car Car] --> B[\"$$x^2$$\"]",
        &RenderOptions::default(),
    );
    assert!(!out.contains("fermaid-icon"));
    assert!(!out.contains("fermaid-math"));
    assert!(out.contains("Car"));
    assert!(!out.contains("fa:fa-car"));
    assert!(out.contains("$$x^2$$"));
}

#[test]
fn draws_an_icon_the_host_has_and_leaves_out_one_it_has_not() {
    let options = RenderOptions {
        icons: Some(icons()),
        ..Default::default()
    };
    let plain = svg("flowchart LR\n  A[Car]", &RenderOptions::default());
    let out = svg(
        "flowchart LR\n  A[fa:fa-car Car] --> B[fa:fa-boat Boat]",
        &options,
    );
    assert_eq!(out.matches("class=\"fermaid-icon\"").count(), 1);
    assert!(out.contains("data-icon=\"fa:fa-car\""));
    assert!(out.contains(ICON));
    // The icon has room of its own, so the label is wider than the same label without it.
    assert!(out.contains("> Car</tspan>"), "{out}");
    assert!(out.contains(">Boat</text>"), "{out}");
    assert!(!out.contains("fa:fa-boat"));
    assert_ne!(out, plain);
    assert_inert(&out, "icon");
}

#[test]
fn an_icon_makes_the_node_wider() {
    let options = RenderOptions {
        icons: Some(icons()),
        ..Default::default()
    };
    let with = fermaid::render("flowchart LR\n  A[fa:fa-car Car]", &options).unwrap();
    let without = fermaid::render("flowchart LR\n  A[Car]", &options).unwrap();
    // 1.25 em of icon, at 16 pixels.
    assert!(
        with.width > without.width + 19.0,
        "{} {}",
        with.width,
        without.width
    );
}

#[test]
fn typesets_math_and_makes_room_for_it() {
    let options = RenderOptions {
        math: Some(math(30.0, 40.0)),
        ..Default::default()
    };
    let out = svg("flowchart LR\n  A[\"$$x^2$$\"]", &options);
    assert!(out.contains("class=\"fermaid-math\""));
    assert!(out.contains(MATH));
    assert!(!out.contains("$$"));
    assert_inert(&out, "math");

    // A line that holds math is as tall as the math is: 40 and a quarter of the font size.
    let tall = fermaid::render("flowchart LR\n  A[\"$$x$$\"]", &options).unwrap();
    let flat = fermaid::render("flowchart LR\n  A[\"x\"]", &options).unwrap();
    assert!(
        tall.height > flat.height + 18.0,
        "{} {}",
        tall.height,
        flat.height
    );
}

#[test]
fn math_that_the_host_does_not_typeset_stays_as_written() {
    let options = RenderOptions {
        math: Some(Box::new(|tex, _| {
            (tex == "ok").then(|| MathMarkup::Svg {
                svg: MATH.to_string(),
                width: 10.0,
                height: 10.0,
            })
        })),
        ..Default::default()
    };
    let out = svg("flowchart LR\n  A[\"$$ok$$ and $$other$$\"]", &options);
    assert_eq!(out.matches("class=\"fermaid-math\"").count(), 1);
    assert!(out.contains("$$other$$"));
}

#[test]
fn ignores_math_with_a_size_that_cannot_be_drawn() {
    for (width, height) in [
        (0.0, 10.0),
        (10.0, -1.0),
        (f64::NAN, 10.0),
        (10.0, f64::INFINITY),
    ] {
        let options = RenderOptions {
            math: Some(math(width, height)),
            ..Default::default()
        };
        let out = svg("flowchart LR\n  A[\"$$x$$\"]", &options);
        assert!(!out.contains("fermaid-math"), "{width} {height}");
        assert!(out.contains("$$x$$"));
    }
}

#[test]
fn asks_the_host_once_for_the_same_math() {
    let asked = Rc::new(RefCell::new(Vec::new()));
    let log = asked.clone();
    let options = RenderOptions {
        math: Some(Box::new(move |tex, size| {
            log.borrow_mut().push((tex.to_string(), size));
            None
        })),
        ..Default::default()
    };
    svg(
        "flowchart LR\n  A[\"$$a$$\"] --> B[\"$$a$$ $$b$$\"]",
        &options,
    );
    let asked = asked.borrow();
    assert_eq!(asked.len(), 2, "{asked:?}");
    assert_eq!(asked[0], ("a".to_string(), 16.0));
    assert_eq!(asked[1].0, "b");
}

#[test]
fn a_title_is_plain_text() {
    let options = RenderOptions {
        math: Some(math(30.0, 40.0)),
        icons: Some(icons()),
        ..Default::default()
    };
    let out = svg("---\ntitle: $$a$$\n---\nflowchart LR\n  A", &options);
    assert!(!out.contains("fermaid-math"));
    assert!(out.contains("$$a$$"));
}

#[test]
fn output_is_deterministic() {
    let options = || RenderOptions {
        icons: Some(icons()),
        math: Some(math(30.0, 40.0)),
        ..Default::default()
    };
    let src = "flowchart LR\n  A[fa:fa-car $$x$$ text] --> B[\"a <b>b</b> fa:fa-car\"]";
    assert_eq!(svg(src, &options()), svg(src, &options()));
}

#[test]
fn the_text_of_a_diagram_reaches_the_output_only_escaped() {
    // The host answers for every name and every expression, with markup of its own. What the
    // diagram says is handed over as a name or as TeX, and drawn nowhere else.
    let options = RenderOptions {
        icons: Some(Box::new(|_| Some(ICON.to_string()))),
        math: Some(Box::new(|_, _| None)),
        ..Default::default()
    };
    for payload in PAYLOADS {
        let src = format!(
            "flowchart LR\n  A[\"fa:fa-car {} $${}$$\"]",
            payload.replace('"', "#quot;"),
            payload.replace('"', "#quot;")
        );
        if let Ok(rendered) = fermaid::render(&src, &options) {
            assert_inert(&rendered.svg, &src);
        }
    }
}
