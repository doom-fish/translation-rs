import Foundation
import Translation

public typealias TRLAsyncCallback = @convention(c) (
    UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer
) -> Void

@inline(__always)
private func trlAsyncErrorMessage(_ error: Error) -> String {
    if let bridgeError = error as? TRLBridgeError {
        return bridgeError.description
    }
    if #available(macOS 15.0, *), let translationError = error as? TranslationError {
        let description = translationError.errorDescription ?? error.localizedDescription
        if let failureReason = translationError.failureReason, !failureReason.isEmpty {
            return "\(description): \(failureReason)"
        }
        return description
    }
    if let localizedError = error as? LocalizedError,
       let description = localizedError.errorDescription
    {
        if let failureReason = localizedError.failureReason, !failureReason.isEmpty {
            return "\(description): \(failureReason)"
        }
        return description
    }
    return error.localizedDescription
}

@inline(__always)
private func trlAsyncFail(
    _ error: Error,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    trlAsyncErrorMessage(error).withCString { ptr in
        cb(nil, ptr, ctx)
    }
}

@inline(__always)
private func trlRetainTranslationSessionToken(
    _ token: UnsafeMutableRawPointer?
) throws -> UnsafeMutableRawPointer {
    guard let token else {
        throw TRLBridgeError.invalidArgument("missing translation session token")
    }
    return Unmanaged<TRLTranslationSessionBox>.fromOpaque(UnsafeRawPointer(token))
        .retain()
        .toOpaque()
}

@available(macOS 15.0, *)
@inline(__always)
private func trlRetainLanguageAvailabilityToken(
    _ token: UnsafeMutableRawPointer?
) throws -> UnsafeMutableRawPointer {
    guard let token else {
        throw TRLBridgeError.invalidArgument("missing language availability token")
    }
    return Unmanaged<TRLLanguageAvailabilityBox>.fromOpaque(UnsafeRawPointer(token))
        .retain()
        .toOpaque()
}

@_cdecl("trl_session_translate_async")
public func trl_session_translate_async(
    _ token: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession translation requires macOS 26+"
            )
        }
        let textString = try trlRequireString(text, field: "text")
        let retainedToken = try trlRetainTranslationSessionToken(token)
        Task {
            let boxRef = Unmanaged<TRLTranslationSessionBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                #if TRANSLATION_HAS_MACOS26_SDK
                let box = boxRef.takeUnretainedValue()
                let response = try await box.session().translate(textString)
                let json = try trlEncodeJSON(trlTranslationResponsePayload(from: response))
                json.withCString { ptr in
                    cb(UnsafeRawPointer(ptr), nil, ctx)
                }
                #else
                throw TRLBridgeError.unavailableOnThisMacOS(
                    "manual TranslationSession translation requires the macOS 26 SDK"
                )
                #endif
            } catch {
                trlAsyncFail(error, cb, ctx)
            }
        }
    } catch {
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_session_translations_async")
public func trl_session_translations_async(
    _ token: UnsafeMutableRawPointer?,
    _ requestsJson: UnsafePointer<CChar>?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession translation requires macOS 26+"
            )
        }
        let requests = try trlDecodeJSON(
            requestsJson,
            as: [TRLTranslationRequestPayload].self
        )
        let requestPayloads = requests.map {
            (sourceText: $0.sourceText, clientIdentifier: $0.clientIdentifier)
        }
        let retainedToken = try trlRetainTranslationSessionToken(token)
        Task {
            let boxRef = Unmanaged<TRLTranslationSessionBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                #if TRANSLATION_HAS_MACOS26_SDK
                let box = boxRef.takeUnretainedValue()
                let sessionRequests = requestPayloads.map {
                    TranslationSession.Request(
                        sourceText: $0.sourceText,
                        clientIdentifier: $0.clientIdentifier
                    )
                }
                let responses = try await box.session().translations(from: sessionRequests)
                let json = try trlEncodeJSON(responses.map(trlTranslationResponsePayload))
                json.withCString { ptr in
                    cb(UnsafeRawPointer(ptr), nil, ctx)
                }
                #else
                throw TRLBridgeError.unavailableOnThisMacOS(
                    "manual TranslationSession translation requires the macOS 26 SDK"
                )
                #endif
            } catch {
                trlAsyncFail(error, cb, ctx)
            }
        }
    } catch {
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_session_prepare_translation_async")
public func trl_session_prepare_translation_async(
    _ token: UnsafeMutableRawPointer?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession preparation requires macOS 26+"
            )
        }
        let retainedToken = try trlRetainTranslationSessionToken(token)
        Task {
            let boxRef = Unmanaged<TRLTranslationSessionBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                #if TRANSLATION_HAS_MACOS26_SDK
                let box = boxRef.takeUnretainedValue()
                try await box.session().prepareTranslation()
                cb(UnsafeRawPointer(bitPattern: 1), nil, ctx)
                #else
                throw TRLBridgeError.unavailableOnThisMacOS(
                    "manual TranslationSession preparation requires the macOS 26 SDK"
                )
                #endif
            } catch {
                trlAsyncFail(error, cb, ctx)
            }
        }
    } catch {
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_language_availability_status_async")
public func trl_language_availability_status_async(
    _ token: UnsafeMutableRawPointer?,
    _ sourceLanguage: UnsafePointer<CChar>?,
    _ targetLanguage: UnsafePointer<CChar>?,
    _ ctx: UnsafeMutableRawPointer,
    _ cb: @escaping TRLAsyncCallback
) {
    do {
        guard #available(macOS 15.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+"
            )
        }
        let source = try trlRequireString(sourceLanguage, field: "source language")
        let target = targetLanguage.map(String.init(cString:))
        let availability = try trlLanguageAvailabilityBox(token).availability
        let retainedToken = try trlRetainLanguageAvailabilityToken(token)
        Task {
            let boxRef = Unmanaged<TRLLanguageAvailabilityBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            let status = await availability.status(
                from: trlLanguage(from: source),
                to: target.map { trlLanguage(from: $0) }
            )
            let rawStatus = trlAvailabilityStatusRaw(status)
            cb(UnsafeRawPointer(bitPattern: Int(rawStatus) + 1), nil, ctx)
        }
    } catch {
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_language_availability_supported_languages_async")
public func trl_language_availability_supported_languages_async(
    _ token: UnsafeMutableRawPointer?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    do {
        guard #available(macOS 15.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+"
            )
        }
        let availability = try trlLanguageAvailabilityBox(token).availability
        let retainedToken = try trlRetainLanguageAvailabilityToken(token)
        Task {
            let boxRef = Unmanaged<TRLLanguageAvailabilityBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                let languages = await availability.supportedLanguages.map(trlLanguageTag).sorted()
                let json = try trlEncodeJSON(languages)
                json.withCString { ptr in
                    cb(UnsafeRawPointer(ptr), nil, ctx)
                }
            } catch {
                trlAsyncFail(error, cb, ctx)
            }
        }
    } catch {
        trlAsyncFail(error, cb, ctx)
    }
}
