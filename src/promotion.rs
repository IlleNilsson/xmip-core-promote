//! A configured promotion: a property and where its value is read.

use context::MessageContext;
use contract::ContractError;
use path::{CompiledPath, Content};
use xcore::ScalarValue;

/// Where a promoted value is read.
#[derive(Debug)]
enum Source {
    /// A path into the content, compiled.
    Content(CompiledPath),
    /// A value the context holds under this name: a header the transport
    /// set, or a property promoted before.
    Context(String),
}

/// One configured promotion: the property written, and where its value is
/// read.
#[derive(Debug)]
pub struct Promotion {
    property: String,
    source: Source,
}

impl Promotion {
    /// `property` read through `path`, compiled.
    #[must_use]
    pub fn content(property: impl Into<String>, path: CompiledPath) -> Self {
        Self {
            property: property.into(),
            source: Source::Content(path),
        }
    }

    /// `property` read from the context's value under `name`.
    #[must_use]
    pub fn context(property: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            property: property.into(),
            source: Source::Context(name.into()),
        }
    }

    /// The property it writes.
    #[must_use]
    pub fn property(&self) -> &str {
        &self.property
    }

    /// The value promoted from `content` and `context`, where there is one.
    ///
    /// # Errors
    /// The content is not in the form the path reads.
    pub fn read(
        &self,
        content: &Content<'_>,
        context: &MessageContext,
    ) -> Result<Option<ScalarValue>, ContractError> {
        match &self.source {
            Source::Content(path) => path.read(content),
            Source::Context(name) => Ok(context.get(name).cloned()),
        }
    }

    /// `context` with the value promoted, where there is one; as it was
    /// where there is none.
    ///
    /// # Errors
    /// As [`Promotion::read`].
    pub fn promote(
        &self,
        content: &Content<'_>,
        context: MessageContext,
    ) -> Result<MessageContext, ContractError> {
        Ok(match self.read(content, &context)? {
            Some(value) => context.with_value(self.property.clone(), value),
            None => context,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use path::{Path, PathEngine};

    /// The language `whole`: the Stream's text, whatever the expression.
    struct Whole;

    struct Text;

    impl path::PathLanguage for Whole {
        fn language(&self) -> &'static str {
            "whole"
        }

        fn compile(&self, _: &str) -> Result<Box<dyn path::CompiledExpression>, ContractError> {
            Ok(Box::new(Text))
        }
    }

    impl path::CompiledExpression for Text {
        fn read(&self, content: &Content<'_>) -> Result<Option<ScalarValue>, ContractError> {
            Ok(Some(ScalarValue::Text(content.text()?.into())))
        }

        fn write(&self, _: &mut path::Rewriting, _: ScalarValue) -> Result<(), ContractError> {
            Err(ContractError::new("read only"))
        }
    }

    #[test]
    fn a_promotion_reads_the_content_or_the_context_into_its_property() {
        let engine = PathEngine::new(vec![Box::new(Whole)]);
        let stream = stream::Stream::new(xcore::StreamId::new(1), b"A-1".to_vec(), None);
        let content = Content::of(&stream);
        let from_content = Promotion::content(
            "Order",
            engine.compile(&Path::new("whole", "")).expect("compiles"),
        );
        let header = Promotion::context("Channel", "http.x-channel");
        let context =
            MessageContext::new().with_value("http.x-channel", ScalarValue::Text("web".into()));
        let promoted = header
            .promote(
                &content,
                from_content.promote(&content, context).expect("promotes"),
            )
            .expect("promotes");
        assert_eq!(
            promoted.get("Order"),
            Some(&ScalarValue::Text("A-1".into()))
        );
        assert_eq!(
            promoted.get("Channel"),
            Some(&ScalarValue::Text("web".into()))
        );
        let absent = Promotion::context("Missing", "nothing");
        let unchanged = absent
            .promote(&content, MessageContext::new())
            .expect("promotes");
        assert_eq!(unchanged.get("Missing"), None);
        assert_eq!(absent.property(), "Missing");
    }
}
