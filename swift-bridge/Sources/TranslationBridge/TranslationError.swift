import Foundation
import Translation

struct TRLErrorPayload: Codable {
    var description: String
    var failureReason: String?
}

func trlEncodedError(_ error: Error) -> String {
    let payload: TRLErrorPayload
    if let bridgeError = error as? TRLBridgeError {
        payload = TRLErrorPayload(description: bridgeError.description, failureReason: nil)
    } else {
        let localizedError = error as? LocalizedError
        let description: String
        let failureReason: String?
        if #available(macOS 15.0, *), let translationError = error as? TranslationError {
            description = translationError.errorDescription ?? error.localizedDescription
            failureReason = translationError.failureReason ?? localizedError?.failureReason
        } else {
            description = error.localizedDescription
            failureReason = localizedError?.failureReason
        }
        payload = TRLErrorPayload(description: description, failureReason: failureReason)
    }

    do {
        return try trlEncodeJSON(payload)
    } catch {
        return payload.description
    }
}

@inline(__always)
func trlWriteError(
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) -> Int32 {
    outErrorMessage?.pointee = trlCString(trlEncodedError(error))
    return trlStatus(from: error)
}

func trlStatus(from error: Error) -> Int32 {
    if let bridgeError = error as? TRLBridgeError {
        return bridgeError.statusCode
    }

    if #available(macOS 15.0, *) {
        if TranslationError.unsupportedSourceLanguage ~= error {
            return TRL_UNSUPPORTED_SOURCE_LANGUAGE
        }
        if TranslationError.unsupportedTargetLanguage ~= error {
            return TRL_UNSUPPORTED_TARGET_LANGUAGE
        }
        if TranslationError.unsupportedLanguagePairing ~= error {
            return TRL_UNSUPPORTED_LANGUAGE_PAIRING
        }
        if TranslationError.unableToIdentifyLanguage ~= error {
            return TRL_UNABLE_TO_IDENTIFY_LANGUAGE
        }
        if TranslationError.nothingToTranslate ~= error {
            return TRL_NOTHING_TO_TRANSLATE
        }
        #if TRANSLATION_HAS_MACOS26_SDK
        if #available(macOS 26.0, *) {
            if TranslationError.alreadyCancelled ~= error {
                return TRL_ALREADY_CANCELLED
            }
            if TranslationError.notInstalled ~= error {
                return TRL_NOT_INSTALLED
            }
        }
        #endif
        if TranslationError.internalError ~= error {
            return TRL_FRAMEWORK_ERROR
        }
    }

    return TRL_FRAMEWORK_ERROR
}
