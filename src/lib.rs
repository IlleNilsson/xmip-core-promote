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
