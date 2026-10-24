//! Gantt chart rendering, security, limits and speed.
#![cfg(feature = "gantt")]

mod support;

use std::time::{Duration, Instant};

use fermaid::{DiagramKind, ErrorCode, Limits, RenderOptions};
use support::{PAYLOADS, Random, assert_inert, mutate};

const FULL: &str = "gantt
dateFormat YYYY-MM-DD
title Adding GANTT diagram functionality to mermaid
excludes weekends
section A section
Completed task :done, des1, 2014-01-06,2014-01-08
Active task :active, des2, 2014-01-09, 3d
Future task : des3, after des2, 5d
Future task2 : des4, after des3, 5d
section Critical tasks
Completed task in the critical line :crit, done, 2014-01-06,24h
Implement parser and jison :crit, done, after des1, 2d
Create tests for parser :crit, active, 3d
Future task in critical line :crit, 5d
Create tests for renderer :2d
Add to mermaid :until isadded
Functionality added :milestone, isadded, 2014-01-25, 0d
section Documentation
Describe gantt syntax :active, a1, after des1, 3d
Add gantt diagram to demo page :after a1 , 20h
Release :vert, v1, 2014-01-20, 0d
click des2 href \"https://example.com/des2\"
";

const COMPACT: &str = "---
displayMode: compact
---
gantt
dateFormat YYYY-MM-DD
topAxis
tickInterval 1week
weekday monday
section S
A :a, 2024-01-01, 3d
B :b, 2024-01-02, 3d
C :c, 2024-01-05, 2d
section T
D :d, after a, 1d
";

/// 2014-01-15, in the middle of `FULL`.
const NOW: f64 = 1_389_744_000_000.0;

fn options(now: Option<f64>) -> RenderOptions {
    RenderOptions {
        now,
        ..RenderOptions::default()
    }
}

fn render(src: &str) -> fermaid::Rendered {
    fermaid::render(src, &options(Some(NOW))).unwrap_or_else(|e| panic!("{src:?}: {e}"))
}

fn svg(src: &str) -> String {
    render(src).svg
}

fn count(svg: &str, needle: &str) -> usize {
    svg.matches(needle).count()
}

/// Task groups, leaving out the group that holds them.
fn tasks(svg: &str) -> usize {
    count(svg, "<g class=\"fermaid-task") - count(svg, "<g class=\"fermaid-tasks\"")
}

fn try_render(src: &str) -> Option<String> {
    fermaid::render(src, &options(Some(NOW)))
        .ok()
        .map(|r| r.svg)
}

#[test]
fn draws_tasks_sections_and_markers() {
    let out = render(FULL);
    assert_eq!(out.kind, DiagramKind::Gantt);
    assert_eq!(tasks(&out.svg), 13);
    assert_eq!(count(&out.svg, " fermaid-crit"), 4);
    assert_eq!(count(&out.svg, "fermaid-milestone"), 1);
    assert_eq!(count(&out.svg, "class=\"fermaid-section-label\""), 3);
    assert_eq!(count(&out.svg, "class=\"fermaid-vert\""), 1);
    assert!(out.svg.contains("fermaid-excluded"));
    assert!(out.svg.contains("fermaid-today"));
    assert!(out.svg.contains(">2014-01-15<"));
    assert_eq!(out.links.len(), 1);
    assert_eq!(out.links[0].href, "https://example.com/des2");
}

#[test]
fn today_follows_the_given_time() {
    // At the default time, 1970-01-01, the chart is long past and has no marker.
    let out = fermaid::render(FULL, &options(None)).unwrap();
    assert!(!out.svg.contains("fermaid-today"));
    let off = FULL.replace("excludes weekends", "todayMarker off");
    assert!(!svg(&off).contains("fermaid-today"));
}

#[test]
fn packs_rows_in_compact_mode() {
    let out = render(COMPACT);
    assert_eq!(tasks(&out.svg), 4);
    // A and C share a row, as do the top and bottom axis labels.
    let normal = render(&COMPACT.replace("displayMode: compact", "title: x"));
    assert!(out.height < normal.height);
}

#[test]
fn reports_errors() {
    let err = fermaid::render("gantt\nA :a, 2024-01-01\n", &options(None)).unwrap_err();
    assert_eq!(err.kind, Some(DiagramKind::Gantt));
    let err = fermaid::render("gantt\ndateFormat\n", &options(None)).unwrap_err();
    assert_eq!(err.code, ErrorCode::Syntax);
    assert_eq!(err.line, 2);
    let err = fermaid::render("gantt\nA :a, 2024-13-45, 1d\n", &options(None)).unwrap_err();
    assert_eq!(err.code, ErrorCode::Semantic);
}

#[test]
fn fits_a_narrow_host() {
    let narrow = fermaid::render(
        FULL,
        &RenderOptions {
            max_width: Some(400.0),
            ..options(Some(NOW))
        },
    )
    .unwrap();
    assert!(narrow.width < render(FULL).width);
}

#[test]
fn output_is_deterministic() {
    assert_eq!(svg(FULL), svg(FULL));
}

#[test]
fn is_inert_for_hostile_and_mutated_inputs() {
    let mut rendered = 0;
    for payload in PAYLOADS {
        let p = payload.replace('\n', " ");
        for src in [
            format!("gantt\ntitle {p}\nA :2024-01-01, 1d"),
            format!("gantt\nsection {p}\n{p} :a, 2024-01-01, 1d"),
            format!("gantt\n{p} :vert, v, 2024-01-01, 0d\nA :2024-01-01, 2d"),
            format!("gantt\nA :a, 2024-01-01, 1d\nclick a href \"{p}\""),
            format!("gantt\naxisFormat {p}\nA :2024-01-01, 1d"),
            format!("gantt\ntodayMarker {p}\nA :1970-01-01, 1d"),
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
        "\n",
        ":",
        ", ",
        "after a1",
        ":crit, ",
        ":milestone, ",
        "excludes weekends\n",
        "tickInterval 1day\n",
        "dateFormat X\n",
        "click des1 href ",
    ]);
    let corpus = [FULL, COMPACT];
    let mut rnd = Random::new(13);
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
        ..options(None)
    };
    const N: usize = 50000;
    let head = "gantt\ndateFormat YYYY-MM-DD\n";
    let cases = [
        format!("{head}{} :2024-01-01, 1d", "x".repeat(N)),
        format!(
            "{head}{}",
            repeat(3000, |i| format!("t{i} :2024-01-01, {i}d\n"))
        ),
        format!(
            "{head}a0 :a0, 2024-01-01, 1d\n{}",
            repeat(3000, |i| format!("t :a{}, after a{i}, 1d\n", i + 1))
        ),
        format!(
            "{head}{}",
            repeat(3000, |i| format!("t :a{i}, after a{}, 1d\n", i + 1))
        ),
        format!("{head}excludes weekends\nA :2000-01-01, 9999y"),
        format!("{head}A :0001-01-01, 9999-12-31"),
        format!("{head}tickInterval 1millisecond\nA :2024-01-01, 1y"),
        format!("gantt\ndateFormat {}\nA :x, 1d", "Y".repeat(N / 2)),
        format!(
            "{head}excludes {}\nA :2024-01-01, 300d",
            "2024-01-01,".repeat(N / 11)
        ),
        format!("{head}{}", "%%\n".repeat(N / 3)),
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
    let cases = [("full", FULL.to_string()), ("compact", COMPACT.to_string())];
    for (name, src) in cases {
        insta::assert_snapshot!(name, svg(&src));
    }
}
