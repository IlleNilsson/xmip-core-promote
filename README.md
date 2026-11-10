# xmip-core-promote

Promotion: a value out of a Message into its context, under a name a
Subscription's filter and every step after read. Three things promote, and
each is compiled once, never per Message:

- what a filter names, by `xmip-core-route`'s `Gathering` (ADR-0046,
  amendment 2026-09-27): every name compiled through the route technology
  its prefix names and read at arrival;
- what the gates concluded, by the runtime's arrival itself, ADR-0013's
  default promotion;
- what a step configures: a `Promotion`, the standard Promote unit's
  (`xmip-core-unit`), a property and where its value is read — a path into
  the content compiled through the path engine (XPath, JSONPath, the
  content selector, an index for a CSV column, a segment and element for
  EDI, a field for HL7 v2, a regular expression), or a value the context
  holds under another name, a header the transport set.

The content selector language, `doc/content-selector.md` beside this file,
is the `dot` path technology (`xmip-core-path-dot`). A path reads the
content in the form its language parses, once per Message however many
paths read it, so a promotion holds what that form holds: the document,
for XPath and JSONPath. Demotion, the reverse direction, is
`xmip-core-demote`.

ADR-0066, amendments 2026-10-22, governs the Promote unit;
`architecture.toml` carries the maturity.
