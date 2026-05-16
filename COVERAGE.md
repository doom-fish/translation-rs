# Translation.framework coverage

Audited against:

- SDK: `MacOSX.sdk/System/Library/Frameworks/Translation.framework/Versions/A/Modules/Translation.swiftmodule/arm64e-apple-macos.swiftinterface`
- Crate: `translation-rs` v0.2.0

## LanguageAvailability

| API | Status | Notes |
| --- | --- | --- |
| `LanguageAvailability.init()` | ✅ implemented | `LanguageAvailability::new()` |
| `LanguageAvailability.Status.installed` | ✅ implemented | `LanguageAvailabilityStatus::Installed` |
| `LanguageAvailability.Status.supported` | ✅ implemented | `LanguageAvailabilityStatus::Supported` |
| `LanguageAvailability.Status.unsupported` | ✅ implemented | `LanguageAvailabilityStatus::Unsupported` |
| `LanguageAvailability.supportedLanguages` | ✅ implemented | `supported_languages` + `supported_language_objects` |
| `LanguageAvailability.status(from:to:)` | ✅ implemented | `status_for_languages`, `status_for_language_pair`, `status_for_pair` |
| `LanguageAvailability.status(for:to:)` | ✅ implemented | `status_for_text_in_language`, `status_for_text` |

## TranslationError

| API | Status | Notes |
| --- | --- | --- |
| `TranslationError.unsupportedSourceLanguage` | ✅ implemented | `TranslationError::UnsupportedSourceLanguage` |
| `TranslationError.unsupportedTargetLanguage` | ✅ implemented | `TranslationError::UnsupportedTargetLanguage` |
| `TranslationError.unsupportedLanguagePairing` | ✅ implemented | `TranslationError::UnsupportedLanguagePairing` |
| `TranslationError.unableToIdentifyLanguage` | ✅ implemented | `TranslationError::UnableToIdentifyLanguage` |
| `TranslationError.nothingToTranslate` | ✅ implemented | `TranslationError::NothingToTranslate` |
| `TranslationError.alreadyCancelled` | ✅ implemented | `TranslationError::AlreadyCancelled` |
| `TranslationError.notInstalled` | ✅ implemented | `TranslationError::NotInstalled` |
| `TranslationError.internalError` | ✅ implemented | `TranslationError::Framework` |
| `TranslationError.errorDescription` | ✅ implemented | `TranslationError::error_description()` |
| `TranslationError.failureReason` | ✅ implemented | `TranslationError::failure_reason()` |

## TranslationSession

| API | Status | Notes |
| --- | --- | --- |
| `TranslationSession.sourceLanguage` | ✅ implemented | `TranslationSession::source_language()` |
| `TranslationSession.targetLanguage` | ✅ implemented | `TranslationSession::target_language()` |
| `TranslationSession.canRequestDownloads` | ✅ implemented | `TranslationSession::can_request_downloads()` |
| `TranslationSession.isReady` | ✅ implemented | `TranslationSession::is_ready()` |
| `TranslationSession.Request.init(sourceText:clientIdentifier:)` | ✅ implemented | `TranslationRequest::new` |
| `TranslationSession.Request.sourceText` | ✅ implemented | `source_text` + `set_source_text` |
| `TranslationSession.Request.clientIdentifier` | ✅ implemented | `client_identifier`, `set_client_identifier`, `clear_client_identifier` |
| `TranslationSession.Response.init(...)` | ✅ implemented | `TranslationResponse::new` |
| `TranslationSession.Response.sourceLanguage` | ✅ implemented | `source_language` + `source_language_object` |
| `TranslationSession.Response.targetLanguage` | ✅ implemented | `target_language` + `target_language_object` |
| `TranslationSession.Response.sourceText` | ✅ implemented | `source_text` |
| `TranslationSession.Response.targetText` | ✅ implemented | `target_text` |
| `TranslationSession.Response.clientIdentifier` | ✅ implemented | `client_identifier` + `with_client_identifier` |
| `TranslationSession.BatchResponse.makeAsyncIterator()` | ✅ implemented | `TranslationBatchResponse` wrapper |
| `TranslationSession.BatchResponse.AsyncIterator.next()` | ✅ implemented | `TranslationBatchResponse::try_next()` |
| `TranslationSession.Configuration.init(source:target:)` | ✅ implemented | `TranslationConfiguration::new` / setters |
| `TranslationSession.Configuration.source` | ✅ implemented | `TranslationConfiguration::source()` / `set_source()` |
| `TranslationSession.Configuration.target` | ✅ implemented | `TranslationConfiguration::target()` / `set_target()` |
| `TranslationSession.Configuration.version` | ✅ implemented | `TranslationConfiguration::version()` |
| `TranslationSession.Configuration.invalidate()` | ✅ implemented | `TranslationConfiguration::invalidate()` |
| `TranslationSession.translate(_:)` | ✅ implemented | `TranslationSession::translate()` |
| `TranslationSession.translate(batch:)` | ✅ implemented | `TranslationSession::translate_batch_streaming()` |
| `TranslationSession.translations(from:)` | ✅ implemented | `TranslationSession::translate_batch()` |
| `TranslationSession.prepareTranslation()` | ✅ implemented | `TranslationSession::prepare_translation()` |
| `TranslationSession.cancel()` | ✅ implemented | `TranslationSession::cancel()` |
| `TranslationSession.init(installedSource:target:)` | ✅ implemented | `TranslationSession::new`, `from_language_pair`, `from_translation_configuration` |

## Companion ergonomics (non-Translation.framework surface)

These are crate-level helpers added for parity with the rest of the Apple SDK crates and to make the framework surface ergonomic from Rust.

| API | Status | Notes |
| --- | --- | --- |
| `Language` | ✅ implemented | Swift-backed `Locale.Language` canonicalization |
| `LanguagePair` | ✅ implemented | Optional target-language pairs plus canonicalization |
| `LanguageRecognition` | ✅ implemented | `recognize_language` / `detect_language` via `NLLanguageRecognizer` |
