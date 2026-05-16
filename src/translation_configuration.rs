use serde::{Deserialize, Serialize};

use crate::language::Language;
use crate::language_pair::LanguagePair;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TranslationConfiguration {
    source: Option<Language>,
    target: Option<Language>,
    #[serde(default)]
    version: u64,
}

impl TranslationConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_language_pair(pair: impl Into<LanguagePair>) -> Self {
        let pair = pair.into();
        Self {
            source: Some(pair.source().clone()),
            target: pair.target().cloned(),
            version: 0,
        }
    }

    #[must_use]
    pub fn source(&self) -> Option<&Language> {
        self.source.as_ref()
    }

    #[must_use]
    pub fn target(&self) -> Option<&Language> {
        self.target.as_ref()
    }

    #[must_use]
    pub fn source_identifier(&self) -> Option<&str> {
        self.source().map(Language::identifier)
    }

    #[must_use]
    pub fn target_identifier(&self) -> Option<&str> {
        self.target().map(Language::identifier)
    }

    #[must_use]
    pub const fn version(&self) -> u64 {
        self.version
    }

    pub fn set_source(&mut self, source: Option<Language>) {
        self.source = source;
    }

    pub fn set_target(&mut self, target: Option<Language>) {
        self.target = target;
    }

    pub fn clear_source(&mut self) {
        self.source = None;
    }

    pub fn clear_target(&mut self) {
        self.target = None;
    }

    pub fn invalidate(&mut self) {
        self.version = self.version.saturating_add(1);
    }

    #[must_use]
    pub fn with_source(mut self, source: impl Into<Language>) -> Self {
        self.set_source(Some(source.into()));
        self
    }

    #[must_use]
    pub fn with_target(mut self, target: impl Into<Language>) -> Self {
        self.set_target(Some(target.into()));
        self
    }

    #[must_use]
    pub fn without_source(mut self) -> Self {
        self.clear_source();
        self
    }

    #[must_use]
    pub fn without_target(mut self) -> Self {
        self.clear_target();
        self
    }

    #[must_use]
    pub fn language_pair(&self) -> Option<LanguagePair> {
        self.source
            .clone()
            .map(|source| LanguagePair::new(source, self.target.clone()))
    }
}
