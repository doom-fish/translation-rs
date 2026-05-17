# translation-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 58
VERIFIED: 58
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00

The Translation.framework is Swift-only. Audit source: `Translation.framework/Modules/Translation.swiftmodule/arm64e-apple-macos.swiftinterface` (macOS 26.2.sdk). All 58 public symbols from the framework are enumerated and cross-referenced with the crate's safe Rust wrappers via swift-bridge thunks and Rust safe APIs. Availability attributes (macOS 15.0+ for most, macOS 26.0+ for new additions) are preserved in the Rust API.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `LanguageAvailability` | class | `Translation.swiftinterface` | `LanguageAvailability` |
| `LanguageAvailability.Status` | enum | `Translation.swiftinterface` | `LanguageAvailabilityStatus` |
| `LanguageAvailability.Status.installed` | case | `Translation.swiftinterface` | `LanguageAvailabilityStatus::Installed` |
| `LanguageAvailability.Status.supported` | case | `Translation.swiftinterface` | `LanguageAvailabilityStatus::Supported` |
| `LanguageAvailability.Status.unsupported` | case | `Translation.swiftinterface` | `LanguageAvailabilityStatus::Unsupported` |
| `LanguageAvailability.Status.==` | func | `Translation.swiftinterface` | `LanguageAvailabilityStatus` derives `PartialEq` / `Eq` |
| `LanguageAvailability.Status.hash(into:)` | func | `Translation.swiftinterface` | `LanguageAvailabilityStatus` derives `Hash` |
| `LanguageAvailability.Status.hashValue` | var | `Translation.swiftinterface` | `LanguageAvailabilityStatus` derives `Hash` |
| `LanguageAvailability.supportedLanguages` | var (async) | `Translation.swiftinterface` | `LanguageAvailability::supported_languages` via `trl_language_availability_supported_languages_json` |
| `LanguageAvailability.status(from:to:)` | func (async) | `Translation.swiftinterface` | `LanguageAvailability::status_for_languages` via `trl_language_availability_status_from_to` |
| `LanguageAvailability.status(for:to:)` | func (async throws) | `Translation.swiftinterface` | `LanguageAvailability::status_for_text` via `trl_language_availability_status_for_text` |
| `LanguageAvailability.init()` | func | `Translation.swiftinterface` | `LanguageAvailability::new` via `trl_language_availability_new` |
| `TranslationError` | struct | `Translation.swiftinterface` | `TranslationError` |
| `TranslationError.unsupportedSourceLanguage` | var | `Translation.swiftinterface` | `TranslationError::UnsupportedSourceLanguage` |
| `TranslationError.unsupportedTargetLanguage` | var | `Translation.swiftinterface` | `TranslationError::UnsupportedTargetLanguage` |
| `TranslationError.unsupportedLanguagePairing` | var | `Translation.swiftinterface` | `TranslationError::UnsupportedLanguagePairing` |
| `TranslationError.unableToIdentifyLanguage` | var | `Translation.swiftinterface` | `TranslationError::UnableToIdentifyLanguage` |
| `TranslationError.nothingToTranslate` | var | `Translation.swiftinterface` | `TranslationError::NothingToTranslate` |
| `TranslationError.alreadyCancelled` | var (iOS 26.0+, macOS 26.0+) | `Translation.swiftinterface` | `TranslationError::AlreadyCancelled` |
| `TranslationError.notInstalled` | var (iOS 26.0+, macOS 26.0+) | `Translation.swiftinterface` | `TranslationError::NotInstalled` |
| `TranslationError.internalError` | var | `Translation.swiftinterface` | `TranslationError::Framework` |
| `TranslationError.~=` | func | `Translation.swiftinterface` | Rust pattern matching on `TranslationError` variants |
| `TranslationError.errorDescription` | var | `Translation.swiftinterface` | `TranslationError::error_description` |
| `TranslationError.failureReason` | var | `Translation.swiftinterface` | `TranslationError::failure_reason` |
| `TranslationSession` | class | `Translation.swiftinterface` | `TranslationSession` |
| `TranslationSession.sourceLanguage` | var | `Translation.swiftinterface` | `TranslationSession::source_language` |
| `TranslationSession.targetLanguage` | var | `Translation.swiftinterface` | `TranslationSession::target_language` |
| `TranslationSession.canRequestDownloads` | var (macOS 26.0+) | `Translation.swiftinterface` | `TranslationSession::can_request_downloads` via `trl_session_can_request_downloads` (macOS 26+ only) |
| `TranslationSession.isReady` | var (async, macOS 26.0+) | `Translation.swiftinterface` | `TranslationSession::is_ready` via `trl_session_is_ready` (macOS 26+ only) |
| `TranslationSession.Request` | struct | `Translation.swiftinterface` | `TranslationRequest` |
| `TranslationSession.Request.sourceText` | var | `Translation.swiftinterface` | `TranslationRequest::source_text`, `set_source_text` |
| `TranslationSession.Request.clientIdentifier` | var | `Translation.swiftinterface` | `TranslationRequest::client_identifier`, `set_client_identifier` |
| `TranslationSession.Request.init(sourceText:clientIdentifier:)` | func | `Translation.swiftinterface` | `TranslationRequest::new`, `with_client_identifier` |
| `TranslationSession.Response` | struct | `Translation.swiftinterface` | `TranslationResponse` |
| `TranslationSession.Response.sourceLanguage` | var | `Translation.swiftinterface` | `TranslationResponse::source_language`, `source_language_object` |
| `TranslationSession.Response.targetLanguage` | var | `Translation.swiftinterface` | `TranslationResponse::target_language`, `target_language_object` |
| `TranslationSession.Response.sourceText` | var | `Translation.swiftinterface` | `TranslationResponse::source_text` |
| `TranslationSession.Response.targetText` | var | `Translation.swiftinterface` | `TranslationResponse::target_text` |
| `TranslationSession.Response.clientIdentifier` | var | `Translation.swiftinterface` | `TranslationResponse::client_identifier`, `with_client_identifier` |
| `TranslationSession.Response.init(sourceLanguage:targetLanguage:sourceText:targetText:clientIdentifier:)` | func | `Translation.swiftinterface` | `TranslationResponse::new`, `with_client_identifier` |
| `TranslationSession.BatchResponse` | struct | `Translation.swiftinterface` | `TranslationBatchResponse` |
| `TranslationSession.BatchResponse.Element` | typealias | `Translation.swiftinterface` | `TranslationResponse` |
| `TranslationSession.BatchResponse.makeAsyncIterator()` | func | `Translation.swiftinterface` | `TranslationSession::translate_batch_streaming` via `trl_session_translate_batch_stream_json` |
| `TranslationSession.BatchResponse.AsyncIterator` | struct | `Translation.swiftinterface` | `TranslationBatchResponse` iterator |
| `TranslationSession.BatchResponse.AsyncIterator.next()` | func (async throws) | `Translation.swiftinterface` | `TranslationBatchResponse::try_next` via `trl_batch_response_next_json` |
| `TranslationSession.BatchResponse.AsyncIterator.Element` | typealias | `Translation.swiftinterface` | `TranslationResponse` |
| `TranslationSession.BatchResponse.AsyncIterator.__AsyncIteratorProtocol_Failure` | typealias | `Translation.swiftinterface` | `TranslationError` |
| `TranslationSession.BatchResponse.__AsyncSequence_Failure` | typealias | `Translation.swiftinterface` | `TranslationError` |
| `TranslationSession.Configuration` | struct | `Translation.swiftinterface` | `TranslationConfiguration` |
| `TranslationSession.Configuration.source` | var | `Translation.swiftinterface` | `TranslationConfiguration::source`, `set_source`, `with_source` |
| `TranslationSession.Configuration.target` | var | `Translation.swiftinterface` | `TranslationConfiguration::target`, `set_target`, `with_target` |
| `TranslationSession.Configuration.version` | var | `Translation.swiftinterface` | `TranslationConfiguration::version` |
| `TranslationSession.Configuration.invalidate()` | func | `Translation.swiftinterface` | `TranslationConfiguration::invalidate` |
| `TranslationSession.Configuration.init(source:target:)` | func | `Translation.swiftinterface` | `TranslationConfiguration::new`, `with_source`, `with_target` |
| `TranslationSession.Configuration.==` | func | `Translation.swiftinterface` | `TranslationConfiguration` derives `PartialEq` / `Eq` |
| `TranslationSession.translate(_:)` | func (async throws) | `Translation.swiftinterface` | `TranslationSession::translate` via `trl_session_translate_text_json` |
| `TranslationSession.translate(batch:)` | func | `Translation.swiftinterface` | `TranslationSession::translate_batch_streaming` via `trl_session_translate_batch_stream_json` |
| `TranslationSession.translations(from:)` | func (async throws) | `Translation.swiftinterface` | `TranslationSession::translate_batch` via `trl_session_translate_batch_json` |
| `TranslationSession.prepareTranslation()` | func (async throws) | `Translation.swiftinterface` | `TranslationSession::prepare_translation` via `trl_session_prepare_translation` |
| `TranslationSession.cancel()` | func (macOS 26.0+) | `Translation.swiftinterface` | `TranslationSession::cancel` via `trl_session_cancel` (macOS 26+ only) |
| `TranslationSession.init(installedSource:target:)` | func (convenience, macOS 26.0+) | `Translation.swiftinterface` | `TranslationSession::new`, `from_language_pair` (macOS 26+ only) |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ |  |  | Full public macOS surface is reachable from the crate's public API. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| _None_ |  |  | No deprecated or unavailable-on-macOS symbols were present. All symbols are either core macOS 15.0+ or new in macOS 26.0+. |
