use serde::{Deserialize, Serialize};

use crate::language::Language;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResponse {
    source_language: Language,
    target_language: Language,
    source_text: String,
    target_text: String,
    client_identifier: Option<String>,
}

impl TranslationResponse {
    #[must_use]
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
            client_identifier: None,
        }
    }

    #[must_use]
    pub fn source_language(&self) -> &str {
        self.source_language.identifier()
    }

    #[must_use]
    pub fn target_language(&self) -> &str {
        self.target_language.identifier()
    }

    #[must_use]
    pub fn source_language_object(&self) -> &Language {
        &self.source_language
    }

    #[must_use]
    pub fn target_language_object(&self) -> &Language {
        &self.target_language
    }

    #[must_use]
    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    #[must_use]
    pub fn target_text(&self) -> &str {
        &self.target_text
    }

    #[must_use]
    pub fn client_identifier(&self) -> Option<&str> {
        self.client_identifier.as_deref()
    }

    #[must_use]
    pub fn with_client_identifier(mut self, client_identifier: impl Into<String>) -> Self {
        self.client_identifier = Some(client_identifier.into());
        self
    }
}
