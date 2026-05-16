# Changelog

## [0.1.0] - 2026-05-16

### Added

- `LanguageAvailability` wrapper for supported-language discovery plus installed/supported/unsupported pair status checks.
- `TranslationSessionConfiguration`, `TranslationSession`, `TranslationRequest`, and `TranslationResponse` for manual translation workflows.
- Single-string and batched translation helpers backed by `TranslationSession.translate` and `translations(from:)`.
- `prepare_translation` for language-pack preparation on supported systems.
- `detect_language` fallback powered by `NLLanguageRecognizer`.
- End-to-end smoke example `examples/02_framework_smoke.rs`.
