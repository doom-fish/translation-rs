# Changelog

## [0.2.0] - 2026-05-16

### Added

- Split the Swift bridge and safe Rust API into eight logical areas: `TranslationSession`, `LanguageAvailability`, `TranslationConfiguration`, `TranslationResponse`, `TranslationError`, `Language`, `LanguagePair`, and `LanguageRecognition`.
- Added typed `Language` / `LanguagePair` wrappers with Swift-backed canonicalization and serde support.
- Added mutable `TranslationConfiguration` state with invalidation version tracking.
- Added `TranslationSession::can_request_downloads`, `is_ready`, `cancel`, and streaming batch translation support.
- Added `TranslationError::error_description` and `failure_reason` accessors that preserve `Translation.framework` error metadata.
- Added numbered examples and integration tests for every logical area.
- Added `COVERAGE.md` documenting framework coverage against the active macOS SDK.

### Changed

- Lowered the Swift package deployment target to macOS 14 and weak-linked `Translation.framework` from Rust.
- Switched the Swift async bridge helpers to the `DispatchSemaphore` pattern used by `screencapturekit-rs`.
- Expanded the README to document the typed helper APIs, runtime availability boundaries, and numbered examples.

## [0.1.0] - 2026-05-16

### Added

- `LanguageAvailability` wrapper for supported-language discovery plus installed/supported/unsupported pair status checks.
- `TranslationSessionConfiguration`, `TranslationSession`, `TranslationRequest`, and `TranslationResponse` for manual translation workflows.
- Single-string and batched translation helpers backed by `TranslationSession.translate` and `translations(from:)`.
- `prepare_translation` for language-pack preparation on supported systems.
- `detect_language` fallback powered by `NLLanguageRecognizer`.
- End-to-end smoke example `examples/02_framework_smoke.rs`.
