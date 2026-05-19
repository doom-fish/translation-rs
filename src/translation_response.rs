use serde::{Deserialize, Serialize};

use crate::language::Language;
use crate::translation_attributes::TranslationAttributedString;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Serializable counterpart to `TranslationSession.Response` in Translation.framework.
pub struct TranslationResponse {
    source_language: Language,
    target_language: Language,
    source_text: String,
    target_text: String,
    #[serde(default)]
    attributed_source_text: Option<TranslationAttributedString>,
    #[serde(default)]
    attributed_target_text: Option<TranslationAttributedString>,
    client_identifier: Option<String>,
}

impl TranslationResponse {
    #[must_use]
    /// Creates a translation response from source and translated text.
    pub fn new(
        source_language: impl Into<Language>,
        target_language: impl Into<Language>,
        source_text: impl Into<String>,
        target_text: impl Into<String>,
    ) -> Self {
        Self {
            source_language: source_language.into(),
            target_language: target_language.into(),
            source_text: source_text.into(),
            target_text: target_text.into(),
            attributed_source_text: None,
            attributed_target_text: None,
            client_identifier: None,
        }
    }

    #[must_use]
    /// Creates a translation response from attributed source and target text.
    pub fn from_attributed_text(
        source_language: impl Into<Language>,
        target_language: impl Into<Language>,
        source_text: impl Into<TranslationAttributedString>,
        target_text: impl Into<TranslationAttributedString>,
    ) -> Self {
        let attributed_source_text = source_text.into();
        let attributed_target_text = target_text.into();
        Self {
            source_language: source_language.into(),
            target_language: target_language.into(),
            source_text: attributed_source_text.text().to_owned(),
            target_text: attributed_target_text.text().to_owned(),
            attributed_source_text: Some(attributed_source_text),
            attributed_target_text: Some(attributed_target_text),
            client_identifier: None,
        }
    }

    #[must_use]
    /// Returns the source language identifier.
    pub fn source_language(&self) -> &str {
        self.source_language.identifier()
    }

    #[must_use]
    /// Returns the target language identifier.
    pub fn target_language(&self) -> &str {
        self.target_language.identifier()
    }

    #[must_use]
    /// Returns the source language object.
    pub fn source_language_object(&self) -> &Language {
        &self.source_language
    }

    #[must_use]
    /// Returns the target language object.
    pub fn target_language_object(&self) -> &Language {
        &self.target_language
    }

    #[must_use]
    /// Returns the original source text.
    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    #[must_use]
    /// Returns the translated target text.
    pub fn target_text(&self) -> &str {
        &self.target_text
    }

    #[must_use]
    /// Returns the attributed source text when the request carried attributed input.
    pub fn attributed_source_text(&self) -> Option<&TranslationAttributedString> {
        self.attributed_source_text.as_ref()
    }

    #[must_use]
    /// Returns the attributed target text when Translation.framework preserved attributes.
    pub fn attributed_target_text(&self) -> Option<&TranslationAttributedString> {
        self.attributed_target_text.as_ref()
    }

    #[must_use]
    /// Returns the client identifier, if one was set on the request.
    pub fn client_identifier(&self) -> Option<&str> {
        self.client_identifier.as_deref()
    }

    #[must_use]
    /// Returns a copy with attributed source text.
    pub fn with_attributed_source_text(
        mut self,
        attributed_source_text: impl Into<TranslationAttributedString>,
    ) -> Self {
        let attributed_source_text = attributed_source_text.into();
        attributed_source_text.text().clone_into(&mut self.source_text);
        self.attributed_source_text = Some(attributed_source_text);
        self
    }

    #[must_use]
    /// Returns a copy with attributed target text.
    pub fn with_attributed_target_text(
        mut self,
        attributed_target_text: impl Into<TranslationAttributedString>,
    ) -> Self {
        let attributed_target_text = attributed_target_text.into();
        attributed_target_text.text().clone_into(&mut self.target_text);
        self.attributed_target_text = Some(attributed_target_text);
        self
    }

    #[must_use]
    /// Returns a copy with the given client identifier.
    pub fn with_client_identifier(mut self, client_identifier: impl Into<String>) -> Self {
        self.client_identifier = Some(client_identifier.into());
        self
    }
}
