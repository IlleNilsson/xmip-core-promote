# Content Selectors

Moved here from the estate root on 2026-09-12 (ADR-0020 clause 3: the document lives where its subject lives).


Promotion and demotion cannot use Path, and this is the distinction that most
needs stating: promotion happens against a stream that has deliberately *not*
been materialized. XPath needs a document. The whole point of stream-first is
that there isn't one yet.

So Xmip has its own selector language, used only for promotion and demotion:

```text
order.customer.name
orders[0].id
orders[n].id
headers['desiredProperty']
envelope.body.items[3]['sku']
```

**These are not XPath and not JSONPath.** The brackets are Xmip selector
notation and nothing else. Four segment kinds:

| Kind | Form | Means |
| --- | --- | --- |
| Name | `order`, `customer` | a named step |
| Number | `[0]`, `[3]` | an ordinal occurrence |
| Key | `['desiredProperty']` | a named key |
| Any | `[n]` | a streaming wildcard |

A selector also declares **how far into the stream it needs to reach**, which
is what makes it evaluable without materialization:

| Evaluation | Means |
| --- | --- |
| `stream-prefix` | resolvable near the beginning of the stream |
| `stream-scan` | resolvable by scanning forward, still without materializing |
| `materialized-section` | needs a materialized section or shape |

That declaration is the load-bearing part. It lets the runtime know the cost of
a promotion before performing it, and it lets a Receive Location be configured
with promotions that are cheap by construction.

**The expression is user-facing; the parsed structure is Module-facing.** The
goal is simple usage, not simple implementation — a Content Module may need
sophisticated internals, and an artifact should still see one selector language
across every representation.

### Promotion and demotion

**Promote** extracts selected properties out of a Stream, as far and as fast as
needed, and then stops.

**Demote** is its send-side counterpart, and it is easy to overlook because
nothing in the receive path suggests it exists. It takes selected Xmip context,
promoted properties or Message properties and writes them *into* the outgoing
stream, envelope, metadata, headers or transport-facing properties, as the Send
artifact configures.

Both use Content Selectors. The rule for both:

> On receive, promote as fast and as far as needed, then stop. The original
> Stream stays referenced by the Message Section unless Transformation,
> Assignment or serialization produces a new one. On send, demote only the
> configured properties and serialize only when the outgoing stream requires it.

Recovered from `content-handlers.md` during the ADR-0020 consolidation,
2026-08-26. The selector language, its evaluation modes and demotion survived
only there, and the document was one classification away from being deleted as
superseded handler-era prose.

