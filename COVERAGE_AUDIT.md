# translation-rs coverage audit (vs MacOSX26.5.sdk)

SDK_PUBLIC_SYMBOLS: 75
VERIFIED: 75
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Audit source: `MacOSX26.5.sdk/System/Library/Frameworks/Translation.framework/Versions/A/Modules/Translation.swiftmodule/arm64e-apple-macos.swiftinterface`

The `Header` column uses the macOS swiftinterface slice because `Translation.framework` is Swift-only.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `LanguageAvailability` | class | `Translation.swiftinterface` | `LanguageAvailability` |
| `LanguageAvailability.Status` | enum | `Translation.swiftinterface` | `LanguageAvailabilityStatus` |
| `LanguageAvailability.Status.==` | func | `Translation.swiftinterface` | `LanguageAvailabilityStatus` derives `PartialEq` / `Eq` |
| `LanguageAvailability.Status.hash(into:)` | func | `Translation.swiftinterface` | `LanguageAvailabilityStatus` derives `Hash` |
| `LanguageAvailability.Status.hashValue` | var | `Translation.swiftinterface` | `LanguageAvailabilityStatus` derives `Hash` |
| `LanguageAvailability.supportedLanguages` | var | `Translation.swiftinterface` | `LanguageAvailability::supported_languages`, `supported_language_objects` |
| `LanguageAvailability.status(from:to:)` | func | `Translation.swiftinterface` | `LanguageAvailability::status_for_languages`, `status_for_language_pair`, `status_for_pair` |
| `LanguageAvailability.status(for:to:)` | func | `Translation.swiftinterface` | `LanguageAvailability::status_for_text_in_language`, `status_for_text` |
| `LanguageAvailability.init()` | func | `Translation.swiftinterface` | `LanguageAvailability::new` |
| `LanguageAvailability.preferredStrategy` | var | `Translation.swiftinterface` | `LanguageAvailability::preferred_strategy`, `LanguageAvailability::with_preferred_strategy` |
| `AttributeScopes.TranslationAttributes` | struct | `Translation.swiftinterface` | `TranslationAttributes` |
| `AttributeScopes.TranslationAttributes.DecodingConfiguration` | typealias | `Translation.swiftinterface` | `TranslationAttributesDecodingConfiguration` |
| `AttributeScopes.TranslationAttributes.EncodingConfiguration` | typealias | `Translation.swiftinterface` | `TranslationAttributesEncodingConfiguration` |
| `AttributeScopes.TranslationAttributes.skipsTranslation` | var | `Translation.swiftinterface` | `TranslationAttributes::skips_translation`, `TranslationAttributedString` helpers |
| `AttributeScopes.TranslationAttributes.SkipTranslationAttribute` | enum | `Translation.swiftinterface` | `SkipTranslationAttribute` |
| `AttributeScopes.TranslationAttributes.SkipTranslationAttribute.Value` | typealias | `Translation.swiftinterface` | `SkipTranslationAttributeValue` |
| `AttributeScopes.TranslationAttributes.SkipTranslationAttribute.name` | var | `Translation.swiftinterface` | `SkipTranslationAttribute::NAME` |
| `AttributeScopes.translation` | var | `Translation.swiftinterface` | `TranslationAttributes::translation` |
| `TranslationError` | struct | `Translation.swiftinterface` | `TranslationError` |
| `TranslationError.unsupportedSourceLanguage` | var | `Translation.swiftinterface` | `TranslationError::UnsupportedSourceLanguage` |
| `TranslationError.unsupportedTargetLanguage` | var | `Translation.swiftinterface` | `TranslationError::UnsupportedTargetLanguage` |
| `TranslationError.unsupportedLanguagePairing` | var | `Translation.swiftinterface` | `TranslationError::UnsupportedLanguagePairing` |
| `TranslationError.unableToIdentifyLanguage` | var | `Translation.swiftinterface` | `TranslationError::UnableToIdentifyLanguage` |
| `TranslationError.nothingToTranslate` | var | `Translation.swiftinterface` | `TranslationError::NothingToTranslate` |
| `TranslationError.alreadyCancelled` | var | `Translation.swiftinterface` | `TranslationError::AlreadyCancelled` |
| `TranslationError.notInstalled` | var | `Translation.swiftinterface` | `TranslationError::NotInstalled` |
| `TranslationError.internalError` | var | `Translation.swiftinterface` | `TranslationError::Framework` |
| `TranslationError.~=` | func | `Translation.swiftinterface` | Rust pattern matching on `TranslationError` variants (`matches!`) |
| `TranslationError.errorDescription` | var | `Translation.swiftinterface` | `TranslationError::error_description` |
| `TranslationError.failureReason` | var | `Translation.swiftinterface` | `TranslationError::failure_reason` |
| `TranslationSession` | class | `Translation.swiftinterface` | `TranslationSession` |
| `TranslationSession.sourceLanguage` | var | `Translation.swiftinterface` | `TranslationSession::source_language` |
| `TranslationSession.targetLanguage` | var | `Translation.swiftinterface` | `TranslationSession::target_language` |
| `TranslationSession.canRequestDownloads` | var | `Translation.swiftinterface` | `TranslationSession::can_request_downloads` |
| `TranslationSession.isReady` | var | `Translation.swiftinterface` | `TranslationSession::is_ready` |
| `TranslationSession.preferredStrategy` | var | `Translation.swiftinterface` | `TranslationSession::preferred_strategy`, `TranslationSessionConfiguration::preferred_strategy` |
| `TranslationSession.Request` | struct | `Translation.swiftinterface` | `TranslationRequest` |
| `TranslationSession.Request.sourceText` | var | `Translation.swiftinterface` | `TranslationRequest::source_text`, `set_source_text` |
| `TranslationSession.Request.attributedSourceText` | var | `Translation.swiftinterface` | `TranslationRequest::attributed_source_text`, `set_attributed_source_text`, `with_attributed_source_text` |
| `TranslationSession.Request.clientIdentifier` | var | `Translation.swiftinterface` | `TranslationRequest::client_identifier`, `set_client_identifier`, `clear_client_identifier`, `with_client_identifier` |
| `TranslationSession.Request.init(sourceText:clientIdentifier:)` | func | `Translation.swiftinterface` | `TranslationRequest::new`, `with_client_identifier` |
| `TranslationSession.Response` | struct | `Translation.swiftinterface` | `TranslationResponse` |
| `TranslationSession.Response.sourceLanguage` | var | `Translation.swiftinterface` | `TranslationResponse::source_language`, `source_language_object` |
| `TranslationSession.Response.targetLanguage` | var | `Translation.swiftinterface` | `TranslationResponse::target_language`, `target_language_object` |
| `TranslationSession.Response.sourceText` | var | `Translation.swiftinterface` | `TranslationResponse::source_text` |
| `TranslationSession.Response.targetText` | var | `Translation.swiftinterface` | `TranslationResponse::target_text` |
| `TranslationSession.Response.attributedSourceText` | var | `Translation.swiftinterface` | `TranslationResponse::attributed_source_text`, `TranslationSession::translate_attributed` |
| `TranslationSession.Response.attributedTargetText` | var | `Translation.swiftinterface` | `TranslationResponse::attributed_target_text`, `TranslationSession::translate_attributed` |
| `TranslationSession.Response.clientIdentifier` | var | `Translation.swiftinterface` | `TranslationResponse::client_identifier`, `with_client_identifier` |
| `TranslationSession.Response.init(sourceLanguage:targetLanguage:sourceText:targetText:clientIdentifier:)` | func | `Translation.swiftinterface` | `TranslationResponse::new`, `with_client_identifier` |
| `TranslationSession.BatchResponse` | struct | `Translation.swiftinterface` | `TranslationBatchResponse` |
| `TranslationSession.BatchResponse.Element` | typealias | `Translation.swiftinterface` | `TranslationResponse`, `Iterator<Item = Result<TranslationResponse, TranslationError>>` |
| `TranslationSession.BatchResponse.makeAsyncIterator()` | func | `Translation.swiftinterface` | `TranslationSession::translate_batch_streaming` |
| `TranslationSession.BatchResponse.AsyncIterator` | struct | `Translation.swiftinterface` | `TranslationBatchResponse` |
| `TranslationSession.BatchResponse.AsyncIterator.next()` | func | `Translation.swiftinterface` | `TranslationBatchResponse::try_next`, `Iterator::next` |
| `TranslationSession.BatchResponse.AsyncIterator.Element` | typealias | `Translation.swiftinterface` | `TranslationResponse`, `Iterator<Item = Result<TranslationResponse, TranslationError>>` |
| `TranslationSession.BatchResponse.AsyncIterator.__AsyncIteratorProtocol_Failure` | typealias | `Translation.swiftinterface` | `Iterator<Item = Result<TranslationResponse, TranslationError>>` |
| `TranslationSession.BatchResponse.__AsyncSequence_Failure` | typealias | `Translation.swiftinterface` | `Iterator<Item = Result<TranslationResponse, TranslationError>>` |
| `TranslationSession.Strategy` | struct | `Translation.swiftinterface` | `TranslationStrategy`, `translation_session::Strategy` |
| `TranslationSession.Strategy.highFidelity` | var | `Translation.swiftinterface` | `TranslationStrategy::HighFidelity` |
| `TranslationSession.Strategy.lowLatency` | var | `Translation.swiftinterface` | `TranslationStrategy::LowLatency` |
| `TranslationSession.Configuration` | struct | `Translation.swiftinterface` | `TranslationConfiguration` |
| `TranslationSession.Configuration.source` | var | `Translation.swiftinterface` | `TranslationConfiguration::source`, `set_source`, `clear_source`, `with_source`, `without_source` |
| `TranslationSession.Configuration.target` | var | `Translation.swiftinterface` | `TranslationConfiguration::target`, `set_target`, `clear_target`, `with_target`, `without_target` |
| `TranslationSession.Configuration.preferredStrategy` | var | `Translation.swiftinterface` | `TranslationConfiguration::preferred_strategy`, `set_preferred_strategy`, `with_preferred_strategy` |
| `TranslationSession.Configuration.version` | var | `Translation.swiftinterface` | `TranslationConfiguration::version` |
| `TranslationSession.Configuration.invalidate()` | func | `Translation.swiftinterface` | `TranslationConfiguration::invalidate` |
| `TranslationSession.Configuration.init(source:target:)` | func | `Translation.swiftinterface` | `TranslationConfiguration::new`, `with_source`, `with_target` |
| `TranslationSession.Configuration.==` | func | `Translation.swiftinterface` | `TranslationConfiguration` derives `PartialEq` / `Eq` |
| `TranslationSession.translate(_:)` | func | `Translation.swiftinterface` | `TranslationSession::translate` |
| `TranslationSession.translate(batch:)` | func | `Translation.swiftinterface` | `TranslationSession::translate_batch_streaming` |
| `TranslationSession.translations(from:)` | func | `Translation.swiftinterface` | `TranslationSession::translate_batch` |
| `TranslationSession.prepareTranslation()` | func | `Translation.swiftinterface` | `TranslationSession::prepare_translation` |
| `TranslationSession.cancel()` | func | `Translation.swiftinterface` | `TranslationSession::cancel` |
| `TranslationSession.init(installedSource:target:)` | func | `Translation.swiftinterface` | `TranslationSession::new`, `from_language_pair`, `from_translation_configuration` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ |  |  | Full public macOS surface is reachable from the crate's public API. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| _None_ |  |  | No deprecated macOS symbols were present in the audited macOS slice. |  |
