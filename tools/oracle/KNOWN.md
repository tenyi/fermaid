# Known differences from Mermaid's parsers

`diff.mjs` counts the inputs listed here but does not report them. Each entry is a line of the
form `- <type> "<input as JSON>"`, under a heading that says why fermaid differs.

## Node's URL parser leaves dot segments after a segment that starts with a dot

`@braintree/sanitize-url` runs `new URL()`. In Node (whose parser is ada), a path with a
segment that starts with a dot, such as `.b` or `.;.`, keeps its `.` and `..` segments; the
WHATWG URL standard, which browsers and fermaid follow, resolves them. `diff.mjs` recognizes
these by resolving the segments of Mermaid's result itself, so they are not listed one by one.

## A gantt date that does not fit the chart's format

Mermaid gives a date that the chart's `dateFormat` does not match a second chance with V8's own
date parser, `new Date(text)`, which reads a good deal more than the ECMAScript date time string
format: "2014-1-12" and "2014-001", a bare "0", and text with something in front of the date,
such as "done 2014-01-06". fermaid falls back to the ECMAScript date time string format only, so
it refuses a chart that Mermaid draws. `diff.mjs` recognizes this by the reference's task times: a
task that starts or ends at the time V8 gives the text is a known difference.

## Punycode labels that IDNA's tables refuse

fermaid checks that an `xn--` label is valid Punycode and decodes to no control, space or
ASCII character, but does not carry the Unicode tables of UTS #46 that would refuse the rest.

- url "HTTPS://xn--Example.COM/a/b?c=d#e"
