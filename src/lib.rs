#![forbid(unsafe_code)]

//! Promotion: a value out of a Message into its context, under a name a
//! Subscription's filter and every step after read.
//!
//! Three things promote, each compiled once, never per Message. What a
//! filter names is promoted by `xmip-core-route`'s `Gathering` (ADR-0046,
//! amendment 2026-09-27), and what the gates concluded by the arrival
//! itself (ADR-0013's default promotion). What a step configures is a
//! [`Promotion`]: the standard Promote unit's, a property and where its
//! value is read, a path into the content compiled through the path engine
//! or a value the context already holds under another name.

pub mod declaration;

pub use declaration::DECLARATION;

#[cfg(feature = "units")]
mod promotion;

#[cfg(feature = "units")]
pub use promotion::Promotion;
