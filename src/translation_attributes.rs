use core::ops::Range;

use serde::{Deserialize, Serialize};

use crate::translation_error::TranslationError;

/// Marker for Translation.framework's `AttributeScopes.TranslationAttributes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TranslationAttributes;

/// Marker for Translation.framework's `AttributeScopes.TranslationAttributes.EncodingConfiguration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TranslationAttributesEncodingConfiguration;

/// Marker for Translation.framework's `AttributeScopes.TranslationAttributes.DecodingConfiguration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TranslationAttributesDecodingConfiguration;

impl TranslationAttributes {
    #[must_use]
    /// Returns the Translation attribute scope marker.
    pub const fn translation() -> Self {
        Self
    }

    #[must_use]
    /// Returns the marker for Translation attribute encoding.
    pub const fn encoding_configuration() -> TranslationAttributesEncodingConfiguration {
        TranslationAttributesEncodingConfiguration
    }

    #[must_use]
    /// Returns the marker for Translation attribute decoding.
    pub const fn decoding_configuration() -> TranslationAttributesDecodingConfiguration {
        TranslationAttributesDecodingConfiguration
    }

    #[must_use]
    /// Creates a `SkipTranslationAttribute` value for a run.
    pub const fn skips_translation(value: SkipTranslationAttributeValue) -> SkipTranslationAttribute {
        SkipTranslationAttribute::new(value)
    }
}

/// Rust value type for `SkipTranslationAttribute`.
pub type SkipTranslationAttributeValue = bool;

/// Mirrors `AttributeScopes.TranslationAttributes.SkipTranslationAttribute`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SkipTranslationAttribute(SkipTranslationAttributeValue);

impl SkipTranslationAttribute {
    /// Swift attribute key name exposed by Translation.framework.
    pub const NAME: &str = "Translation.DoNotTranslate";

    #[must_use]
    /// Creates a new skip-translation attribute value.
    pub const fn new(value: SkipTranslationAttributeValue) -> Self {
        Self(value)
    }

    #[must_use]
    /// Returns the wrapped boolean value.
    pub const fn value(self) -> SkipTranslationAttributeValue {
        self.0
    }

    #[must_use]
    /// Returns an enabled skip-translation marker.
    pub const fn enabled() -> Self {
        Self(true)
    }

    #[must_use]
    /// Returns a disabled skip-translation marker.
    pub const fn disabled() -> Self {
        Self(false)
    }
}

impl From<SkipTranslationAttributeValue> for SkipTranslationAttribute {
    fn from(value: SkipTranslationAttributeValue) -> Self {
        Self::new(value)
    }
}

impl From<SkipTranslationAttribute> for SkipTranslationAttributeValue {
    fn from(value: SkipTranslationAttribute) -> Self {
        value.value()
    }
}

/// Run-scoped Translation attributes stored on a `TranslationAttributedString`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationAttributedRun {
    start: usize,
    end: usize,
    value: SkipTranslationAttribute,
}

impl TranslationAttributedRun {
    #[must_use]
    /// Creates a run carrying a skip-translation attribute.
    pub const fn new(start: usize, end: usize, value: SkipTranslationAttribute) -> Self {
        Self { start, end, value }
    }

    #[must_use]
    /// Returns the inclusive-exclusive character range start.
    pub const fn start(&self) -> usize {
        self.start
    }

    #[must_use]
    /// Returns the inclusive-exclusive character range end.
    pub const fn end(&self) -> usize {
        self.end
    }

    #[must_use]
    /// Returns the run range.
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[must_use]
    /// Returns the skip-translation value for the run.
    pub const fn value(&self) -> SkipTranslationAttribute {
        self.value
    }
}

/// Rust-side attributed text wrapper for Translation.framework payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TranslationAttributedString {
    text: String,
    #[serde(default)]
    skip_translation_runs: Vec<TranslationAttributedRun>,
}

impl TranslationAttributedString {
    #[must_use]
    /// Creates attributed text from plain text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            skip_translation_runs: Vec::new(),
        }
    }

    #[must_use]
    /// Returns the plain-text content.
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    /// Returns the stored skip-translation runs.
    pub fn skip_translation_runs(&self) -> &[TranslationAttributedRun] {
        &self.skip_translation_runs
    }

    /// Replaces the plain-text content and clears any stale runs.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.skip_translation_runs.clear();
    }

    /// Adds a skip-translation run using character offsets.
    pub fn add_skip_translation_run(
        &mut self,
        range: Range<usize>,
        value: impl Into<SkipTranslationAttribute>,
    ) -> Result<(), TranslationError> {
        self.validate_range(&range)?;
        self.skip_translation_runs
            .push(TranslationAttributedRun::new(range.start, range.end, value.into()));
        self.skip_translation_runs.sort_by_key(TranslationAttributedRun::start);
        Ok(())
    }

    /// Adds an enabled skip-translation run using character offsets.
    pub fn add_skip_translation(
        &mut self,
        range: Range<usize>,
    ) -> Result<(), TranslationError> {
        self.add_skip_translation_run(range, SkipTranslationAttribute::enabled())
    }

    /// Returns a copy with an enabled skip-translation run.
    pub fn with_skip_translation(
        mut self,
        range: Range<usize>,
    ) -> Result<Self, TranslationError> {
        self.add_skip_translation(range)?;
        Ok(self)
    }

    /// Returns a copy with the provided skip-translation value.
    pub fn with_skip_translation_value(
        mut self,
        range: Range<usize>,
        value: impl Into<SkipTranslationAttribute>,
    ) -> Result<Self, TranslationError> {
        self.add_skip_translation_run(range, value)?;
        Ok(self)
    }

    /// Adds an enabled skip-translation run for the first matching substring.
    pub fn add_skip_translation_for_substring(
        &mut self,
        substring: &str,
    ) -> Result<(), TranslationError> {
        let range = self.substring_range(substring)?;
        self.add_skip_translation(range)
    }

    /// Returns a copy with an enabled skip-translation run for the first matching substring.
    pub fn with_skip_translation_for_substring(
        mut self,
        substring: &str,
    ) -> Result<Self, TranslationError> {
        self.add_skip_translation_for_substring(substring)?;
        Ok(self)
    }

    fn validate_range(&self, range: &Range<usize>) -> Result<(), TranslationError> {
        let character_len = self.text.chars().count();
        if range.start > range.end || range.end > character_len {
            return Err(TranslationError::InvalidArgument(format!(
                "attributed text range {}..{} is outside 0..{}",
                range.start, range.end, character_len
            )));
        }
        Ok(())
    }

    fn substring_range(&self, substring: &str) -> Result<Range<usize>, TranslationError> {
        if substring.is_empty() {
            return Err(TranslationError::InvalidArgument(
                "skip-translation substring must be non-empty".to_owned(),
            ));
        }
        let start_byte = self.text.find(substring).ok_or_else(|| {
            TranslationError::InvalidArgument(format!(
                "substring '{substring}' was not found in attributed text"
            ))
        })?;
        let end_byte = start_byte + substring.len();
        Ok(self.text[..start_byte].chars().count()..self.text[..end_byte].chars().count())
    }
}

impl From<String> for TranslationAttributedString {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for TranslationAttributedString {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}
