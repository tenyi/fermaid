//! Renders arbitrary text. Any panic is a bug: untrusted input must only ever produce `Err`.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    let options = fermaid::RenderOptions::default();
    if let Ok(rendered) = fermaid::render(src, &options) {
        assert!(rendered.svg.chars().count() <= options.limits.output);
    }
    let _ = fermaid::detect_type(src);
});
