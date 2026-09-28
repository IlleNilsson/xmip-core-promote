#![forbid(unsafe_code)]

//! Promotion: what a Subscription's filter reads from a Message.
//!
//! Its one implementation is `xmip-core-route`'s `Gathering` (ADR-0046,
//! amendment 2026-09-27): every name a filter uses is compiled once, through
//! the route technology its prefix names — a path into the content through
//! `content:`, a context value bare — and read from each Message at arrival.
//! What the gates concluded is promoted by the runtime's arrival itself
//! (ADR-0013's default promotion). Nothing else is written here.
