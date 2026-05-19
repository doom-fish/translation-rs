use serde::{Deserialize, Serialize};

use crate::language::Language;
use crate::language_pair::LanguagePair;
use crate::translation_session::TranslationStrategy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Tracks mutable source and target languages for Translation.framework workflows.
pub struct TranslationConfiguration {
    source: Option<Language>,
    target: Option<Language>,
    #[serde(default)]
    preferred_strategy: TranslationStrategy,
    #[serde(default)]
    version: u64,
}

impl TranslationConfiguration {
    #[must_use]
    /// Creates an empty translation configuration.
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    /// Creates a configuration from a `LanguagePair`.
    pub fn from_language_pair(pair: impl Into<LanguagePair>) -> Self {
        let pair = pair.into();
        Self {
            source: Some(pair.source().clone()),
            target: pair.target().cloned(),
            preferred_strategy: TranslationStrategy::default(),
            version: 0,
        }
    }

    #[must_use]
    /// Returns the configured source language.
    pub fn source(&self) -> Option<&Language> {
        self.source.as_ref()
    }

    #[must_use]
    /// Returns the configured target language.
    pub fn target(&self) -> Option<&Language> {
        self.target.as_ref()
    }

    #[must_use]
    /// Returns the configured source language identifier.
    pub fn source_identifier(&self) -> Option<&str> {
        self.source().map(Language::identifier)
    }

    #[must_use]
    /// Returns the configured target language identifier.
    pub fn target_identifier(&self) -> Option<&str> {
        self.target().map(Language::identifier)
    }

    #[must_use]
    /// Returns the preferred Translation.framework strategy.
    pub const fn preferred_strategy(&self) -> TranslationStrategy {
        self.preferred_strategy
    }

    #[must_use]
    /// Returns the invalidation version tracked for this configuration.
    pub const fn version(&self) -> u64 {
        self.version
    }

    /// Sets the source language.
    pub fn set_source(&mut self, source: Option<Language>) {
        self.source = source;
    }

    /// Sets the target language.
    pub fn set_target(&mut self, target: Option<Language>) {
        self.target = target;
    }

    /// Sets the preferred Translation.framework strategy.
    pub fn set_preferred_strategy(&mut self, preferred_strategy: TranslationStrategy) {
        self.preferred_strategy = preferred_strategy;
    }

    /// Clears the source language.
    pub fn clear_source(&mut self) {
        self.source = None;
    }

    /// Clears the target language.
    pub fn clear_target(&mut self) {
        self.target = None;
    }

    /// Increments the invalidation version used by Translation updates.
    pub fn invalidate(&mut self) {
        self.version = self.version.saturating_add(1);
    }

    #[must_use]
    /// Returns a copy with the given source language.
    pub fn with_source(mut self, source: impl Into<Language>) -> Self {
        self.set_source(Some(source.into()));
        self
    }

    #[must_use]
    /// Returns a copy with the given target language.
    pub fn with_target(mut self, target: impl Into<Language>) -> Self {
        self.set_target(Some(target.into()));
        self
    }

    #[must_use]
    /// Returns a copy with the given preferred strategy.
    pub fn with_preferred_strategy(mut self, preferred_strategy: TranslationStrategy) -> Self {
        self.set_preferred_strategy(preferred_strategy);
        self
    }

    #[must_use]
    /// Returns a copy with no source language.
    pub fn without_source(mut self) -> Self {
        self.clear_source();
        self
    }

    #[must_use]
    /// Returns a copy with no target language.
    pub fn without_target(mut self) -> Self {
        self.clear_target();
        self
    }

    #[must_use]
    /// Returns the configured language pair when a source language exists.
    pub fn language_pair(&self) -> Option<LanguagePair> {
        self.source
            .clone()
            .map(|source| LanguagePair::new(source, self.target.clone()))
    }
}
