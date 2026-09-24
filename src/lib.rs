#![forbid(unsafe_code)]

use context::{ContextValue, MessageContext};
use contract::{ContractError, StructureReader};
use path::{Path, PathEngine};

// Not Eq. ContextValue carries Decimal(f64), and f64 has no total equality.
#[derive(Clone, Debug, PartialEq)]
pub struct DefaultPromotion {
    pub key: String,
    pub value: ContextValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPromotion {
    pub path: Path,
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

pub fn apply_path(
    context: MessageContext,
    reader: &dyn StructureReader,
    engine: &dyn PathEngine,
    promotions: &[PathPromotion],
) -> Result<MessageContext, ContractError> {
    let mut result = context;

    for promotion in promotions {
        if let Some(value) = engine.read(reader, &promotion.path)? {
            // A structured field and a promoted property are one type now
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
    use contract::{ContractDescriptor, ContractId, StructureWriter, StructuredValue};
    use path::PathCost;

    /// A reader over one field, and an engine that reads by field name.
    struct OneField(ContractDescriptor);

    impl StructureReader for OneField {
        fn contract(&self) -> &ContractDescriptor {
            &self.0
        }

        fn read(&self, path: &str) -> Result<Option<StructuredValue>, ContractError> {
            Ok((path == "order.id").then(|| StructuredValue::Text("A-1".to_string())))
        }
    }

    struct ByName;

    impl PathEngine for ByName {
        fn language(&self) -> &'static str {
            "dot"
        }

        fn read(
            &self,
            reader: &dyn StructureReader,
            path: &Path,
        ) -> Result<Option<StructuredValue>, ContractError> {
            reader.read(&path.expression)
        }

        fn write(
            &self,
            _: &mut dyn StructureWriter,
            _: &Path,
            _: StructuredValue,
        ) -> Result<(), ContractError> {
            Err(ContractError {
                message: "read-only".to_string(),
            })
        }

        fn cost(&self, _: &Path) -> PathCost {
            PathCost::StreamPrefix
        }
    }

    fn descriptor() -> ContractDescriptor {
        ContractDescriptor {
            id: ContractId("order".to_string()),
            version: "1".to_string(),
            representation: "application/json".to_string(),
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
        let reader = OneField(descriptor());
        let promotions = [
            PathPromotion {
                path: Path::new("dot", "order.id"),
                context_key: "order".to_string(),
            },
            PathPromotion {
                path: Path::new("dot", "order.missing"),
                context_key: "missing".to_string(),
            },
        ];
        let context =
            apply_path(MessageContext::new(), &reader, &ByName, &promotions).expect("promoted");
        assert_eq!(
            context.get("order"),
            Some(&ContextValue::Text("A-1".to_string()))
        );
        assert_eq!(context.get("missing"), None);
    }
}
