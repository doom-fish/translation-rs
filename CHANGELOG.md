# Changelog

## [0.4.0] - 2026-05-19

### Added

- Added `TranslationStrategy` plus `preferred_strategy` wrappers across `LanguageAvailability`, `TranslationConfiguration`, `TranslationSessionConfiguration`, and `TranslationSession`.
- Added `TranslationAttributedString`, `SkipTranslationAttribute`, and Translation attribute markers for attributed translation payloads.
- Added attributed translation support for `TranslationRequest`, `TranslationResponse`, and direct `TranslationSession::translate_attributed` calls.
- Added Swift bridge support for preferred strategy accessors, attributed request payloads, and attributed response payloads.
- Added `tests/translation_strategy_attributed_tests.rs` covering preferred strategy round-trips, attributed translation, and skip-translation attributes.

## [0.3.2] - 2026-05-18

### Changed

- Added concise rustdoc coverage across the safe public API outside `ffi`, including references to the corresponding Translation.framework types where helpful.

## [0.3.1] - 2026-05-18

### Fixed

- Added panic-safe wrappers (`catch_user_panic`) to all FFI callbacks to prevent panics from unwinding across the C ABI boundary (undefined behavior).
- Added `SAFETY` comments to all unsafe blocks throughout the crate for clarity on memory safety invariants.
- Added comprehensive documentation to public unsafe functions in `private.rs` explaining their safety requirements.
- Updated `doom-fish-utils` version range to `>=0.1, <0.3` for better compatibility with future minor releases.

## [0.3.0] - 2026-05-17

### Added

- Added Tier-1 `async_api` wrappers for `TranslationSession` and `LanguageAvailability` using `doom-fish-utils` completion futures.
- Added non-blocking Swift `@_cdecl` async thunks for single translation, batch translation, preparation, availability status, and supported-language discovery.
- Added async examples plus `tests/async_api_tests.rs` behind the `async` Cargo feature.

### Changed

- Added an `async` Cargo feature that pulls in `doom-fish-utils` only when async wrappers are enabled.
- Bumped the crate to v0.3.0 and documented the new async surface area in the README.

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
