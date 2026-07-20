#![forbid(unsafe_code)]

use xmip_context::{ContextValue, MessageContext};
use xmip_contract::{ContractError, StructuredValue, StructureReader};
use xmip_path::{Path, PathEngine};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultPromotion {
    pub key: String,
    pub value: ContextValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPromotion {
    pub path: Path,
    pub context_key: String,
}

pub fn apply_default(context: MessageContext, values: impl IntoIterator<Item = DefaultPromotion>) -> MessageContext {
    values.into_iter().fold(context, |current, item| current.with_value(item.key, item.value))
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
            result = result.with_value(promotion.context_key.clone(), convert(value));
        }
    }

    Ok(result)
}

fn convert(value: StructuredValue) -> ContextValue {
    match value {
        StructuredValue::Null => ContextValue::Null,
        StructuredValue::Bool(value) => ContextValue::Bool(value),
        StructuredValue::Integer(value) => ContextValue::Integer(value),
        StructuredValue::Decimal(value) => ContextValue::Decimal(value),
        StructuredValue::Text(value) => ContextValue::Text(value),
        StructuredValue::Binary(value) => ContextValue::Binary(value),
    }
}
