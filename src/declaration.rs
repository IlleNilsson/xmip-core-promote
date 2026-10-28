//! Promote, the standard unit that promotes a value into the context at
//! receive, as data (ADR-0066, amendment 2026-10-22): compiled with or
//! without its code, so `configure` knows it without linking the path
//! engine.

use node::Stage;
use node::unit::Declaration;
use xcore::settings::{Applies, Kind, Presence, Setting, Settings};

/// Promote's name, as a step names it.
pub const PROMOTE: &str = "Promote";

/// The word a step names the context with as its `language`, for a value
/// the context already holds.
pub const CONTEXT: &str = "context";

/// Promote, as data: at receive alone.
pub const DECLARATION: &Declaration = &Declaration {
    name: PROMOTE,
    capability: "promote",
    stages: &[Stage::Receive],
    settings: &Settings {
        technology: PROMOTE,
        settings: &[
            Setting {
                name: "property",
                kind: Kind::Text,
                presence: Presence::Required,
                meaning: "The context property the value is promoted into.",
                applies: Applies::Both,
            },
            Setting {
                name: "language",
                kind: Kind::Text,
                presence: Presence::Required,
                meaning: "The path language the value is read with, or context for a value the context holds.",
                applies: Applies::Both,
            },
            Setting {
                name: "expression",
                kind: Kind::Text,
                presence: Presence::Required,
                meaning: "The path to the value, in its language, or the context property's name.",
                applies: Applies::Both,
            },
        ],
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promote_is_declared_soundly() {
        assert!(
            DECLARATION.problems().is_empty(),
            "{:?}",
            DECLARATION.problems()
        );
    }
}
