# xmip-core-promote

Promotion: writing known and Path-resolved values into Message Context, where
Subscription evaluation reads them. A default promotion carries a value the
runtime already knows; a Path promotion extracts one from the content.

Promotion changes context, never content, so it creates no new Message. It is
not transformation, and it has no counterpart on that side
(`runtime-model.md` section 9). Demotion, the reverse direction, is
`xmip-core-demote`.

`doc/architecture/runtime-model.md` section 9 governs it, and the content
selector language it uses is `doc/content-selector.md` beside this file;
`architecture.toml` carries the maturity.
