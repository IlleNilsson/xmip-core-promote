# xmip-core-promote

Promotion — what a Subscription's filter reads from a Message — has one
implementation, and it is `xmip-core-route`'s `Gathering` (ADR-0046,
amendment 2026-09-27): every name a filter uses is compiled once through the
route technology its prefix names, a path into the content through
`content:` and the path engine, a context value bare, and read from each
Message at arrival. What the gates concluded is promoted by the runtime's
arrival itself, ADR-0013's default promotion. This repository therefore
holds no code; whether it stays mounted is the owner's to decide.

The content selector language, `doc/content-selector.md` beside this file,
is the `dot` path technology (`xmip-core-path-dot`), which `content:`
reads through. Demotion, the reverse direction, is `xmip-core-demote`.
