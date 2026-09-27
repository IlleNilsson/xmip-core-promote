#![forbid(unsafe_code)]

use context::{ContextValue, MessageContext};
use contract::ContractError;
use path::{CompiledPath, Content, Path, PathEngine};

// Not Eq. ContextValue carries Decimal(f64), and f64 has no total equality.
#[derive(Clone, Debug, PartialEq)]
pub struct DefaultPromotion {
    pub key: String,
    pub value: ContextValue,
}

/// A promotion as configuration writes it: the Path to read and the context
/// key the value goes under.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPromotion {
    pub path: Path,
    pub context_key: String,
}

impl PathPromotion {
    /// This promotion with its Path compiled through `engine`, once, when
    /// configuration is read.
    ///
    /// # Errors
    /// The Path's language is not loaded, or refuses its expression.
    pub fn compile(&self, engine: &PathEngine) -> Result<CompiledPromotion, ContractError> {
        Ok(CompiledPromotion {
            path: engine.compile(&self.path)?,
            context_key: self.context_key.clone(),
        })
    }
}

/// A promotion ready for every Message: its Path compiled.
#[derive(Debug)]
pub struct CompiledPromotion {
    pub path: CompiledPath,
    pub context_key: String,
}

pub fn apply_default(
    context: MessageContext,
    values: impl IntoIterator<Item = DefaultPromotion>,
) -> MessageContext {
    values.into_iter().fold(context, |current, item| {
        current.with_value(item.key, item.value)
    })
}

/// Promote what each compiled Path reads from `content`, parsed once for all
/// of them; a Path that finds nothing promotes nothing.
///
/// # Errors
/// A Path could not read the content.
pub fn apply_path(
    context: MessageContext,
    content: &Content<'_>,
    promotions: &[CompiledPromotion],
) -> Result<MessageContext, ContractError> {
    let mut result = context;

    for promotion in promotions {
        if let Some(value) = promotion.path.read(content)? {
            // A structured field and a promoted property are one type
            // (core::ScalarValue), so a read value drops straight in — no
            // conversion, because there is nothing to convert between.
            result = result.with_value(promotion.context_key.clone(), value);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use contract::fixture::stream;
    use path::{CompiledExpression, PathLanguage, Rewriting};

    /// A language whose expression is a `key=` the text holds a value after.
    struct KeyValue;

    struct Key(String);

    impl PathLanguage for KeyValue {
        fn language(&self) -> &'static str {
            "key-value"
        }

        fn compile(&self, expression: &str) -> Result<Box<dyn CompiledExpression>, ContractError> {
            Ok(Box::new(Key(format!("{expression}="))))
        }
    }

    impl CompiledExpression for Key {
        fn read(&self, content: &Content<'_>) -> Result<Option<ContextValue>, ContractError> {
            Ok(content
                .text()?
                .split(';')
                .find_map(|pair| pair.strip_prefix(&self.0))
                .map(|value| ContextValue::Text(value.to_string())))
        }

        fn write(&self, _: &mut Rewriting, _: ContextValue) -> Result<(), ContractError> {
            Err(ContractError::new("read-only"))
        }
    }

    #[test]
    fn defaults_are_promoted_in_order_and_the_last_wins() {
        let context = apply_default(
            MessageContext::new(),
            [
                DefaultPromotion {
                    key: "region".to_string(),
                    value: ContextValue::Text("eu".to_string()),
                },
                DefaultPromotion {
                    key: "region".to_string(),
                    value: ContextValue::Text("se".to_string()),
                },
            ],
        );
        assert_eq!(
            context.get("region"),
            Some(&ContextValue::Text("se".to_string()))
        );
    }

    #[test]
    fn a_path_promotion_reads_the_field_and_skips_what_is_not_there() {
        let engine = PathEngine::new(vec![Box::new(KeyValue)]);
        let promotions = [
            PathPromotion {
                path: Path::new("key-value", "order"),
                context_key: "order".to_string(),
            },
            PathPromotion {
                path: Path::new("key-value", "missing"),
                context_key: "missing".to_string(),
            },
        ]
        .map(|promotion| promotion.compile(&engine).expect("compiles"));
        let order = stream("order=A-1;status=open");
        let context =
            apply_path(MessageContext::new(), &Content::of(&order), &promotions).expect("promoted");
        assert_eq!(
            context.get("order"),
            Some(&ContextValue::Text("A-1".to_string()))
        );
        assert_eq!(context.get("missing"), None);

        let unloaded = PathPromotion {
            path: Path::new("xpath", "/order"),
            context_key: "order".to_string(),
        };
        assert!(unloaded.compile(&engine).is_err());
    }
}
