//! `fermaid`: renders a Mermaid diagram to SVG.

use std::io::{self, Read, Write};
use std::process::ExitCode;

const USAGE: &str = "\
Usage: fermaid [INPUT|-] [-o OUTPUT] [--font-size N] [--max-width N] [--id-prefix PREFIX]

Renders the Mermaid diagram in INPUT (or standard input) to SVG.

Options:
  -o, --output OUTPUT    Write the SVG to OUTPUT instead of standard output
      --font-size N      Font size in pixels
      --max-width N      The width the diagram should fit, in pixels
      --id-prefix PREFIX Prefix of the ids in the SVG (default: fermaid)
  -h, --help             Show this help
  -V, --version          Show the version";

struct Args {
    input: Option<String>,
    output: Option<String>,
    options: fermaid::RenderOptions,
}

enum Parsed {
    Run(Args),
    Exit(String),
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Parsed, String> {
    let mut parsed = Args {
        input: None,
        output: None,
        options: fermaid::RenderOptions::default(),
    };
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} needs a value"));
        match arg.as_str() {
            "-h" | "--help" => return Ok(Parsed::Exit(USAGE.to_string())),
            "-V" | "--version" => {
                return Ok(Parsed::Exit(format!(
                    "fermaid {}",
                    env!("CARGO_PKG_VERSION")
                )));
            }
            "-o" | "--output" => parsed.output = Some(value(&arg)?),
            "--font-size" => parsed.options.font_size = Some(pixels(&value(&arg)?, "font size")?),
            "--max-width" => parsed.options.max_width = Some(pixels(&value(&arg)?, "width")?),
            "--id-prefix" => parsed.options.id_prefix = value(&arg)?,
            "-" => parsed.input = None,
            flag if flag.starts_with('-') => return Err(format!("unknown option: {flag}")),
            _ if parsed.input.is_some() => return Err(format!("unexpected argument: {arg}")),
            _ => parsed.input = Some(arg),
        }
    }
    Ok(Parsed::Run(parsed))
}

fn pixels(text: &str, what: &str) -> Result<f64, String> {
    text.parse()
        .ok()
        .filter(|n: &f64| n.is_finite() && *n > 0.0)
        .ok_or(format!("invalid {what}: {text}"))
}

fn read_input(path: Option<&str>) -> io::Result<String> {
    match path {
        Some(path) => std::fs::read_to_string(path),
        None => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text)?;
            Ok(text)
        }
    }
}

fn run(args: Args) -> Result<(), String> {
    let name = args.input.as_deref().unwrap_or("<stdin>");
    let text = read_input(args.input.as_deref()).map_err(|e| format!("{name}: {e}"))?;
    let rendered = fermaid::render(&text, &args.options).map_err(|e| {
        let mut message = name.to_string();
        if e.line > 0 {
            message += &format!(":{}:{}", e.line, e.column);
        }
        message += &format!(": {}", e.message);
        if !e.snippet.is_empty() && !e.message.contains(&e.snippet) {
            message += &format!("\n{}", e.snippet);
        }
        message
    })?;
    match args.output {
        Some(path) => std::fs::write(&path, rendered.svg).map_err(|e| format!("{path}: {e}")),
        None => io::stdout()
            .write_all(rendered.svg.as_bytes())
            .map_err(|e| format!("<stdout>: {e}")),
    }
}

fn main() -> ExitCode {
    match parse_args(std::env::args().skip(1)) {
        Ok(Parsed::Exit(text)) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Ok(Parsed::Run(args)) => match run(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(message) => {
                eprintln!("fermaid: {message}");
                ExitCode::FAILURE
            }
        },
        Err(message) => {
            eprintln!("fermaid: {message}\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Parsed, String> {
        parse_args(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn parses_options() {
        let Ok(Parsed::Run(args)) = parse(&[
            "in.mmd",
            "-o",
            "out.svg",
            "--font-size",
            "14",
            "--max-width",
            "320",
            "--id-prefix",
            "d1",
        ]) else {
            panic!("expected arguments");
        };
        assert_eq!(args.input.as_deref(), Some("in.mmd"));
        assert_eq!(args.output.as_deref(), Some("out.svg"));
        assert_eq!(args.options.font_size, Some(14.0));
        assert_eq!(args.options.max_width, Some(320.0));
        assert_eq!(args.options.id_prefix, "d1");
    }

    #[test]
    fn rejects_bad_arguments() {
        assert!(parse(&["--font-size", "-1"]).is_err());
        assert!(parse(&["--font-size"]).is_err());
        assert!(parse(&["--max-width", "wide"]).is_err());
        assert!(parse(&["--bogus"]).is_err());
        assert!(parse(&["a", "b"]).is_err());
        assert!(matches!(
            parse(&["-"]),
            Ok(Parsed::Run(Args { input: None, .. }))
        ));
        assert!(matches!(parse(&["-h"]), Ok(Parsed::Exit(_))));
    }
}
