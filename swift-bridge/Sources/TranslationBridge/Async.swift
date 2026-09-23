import Foundation
import Translation

public typealias TRLAsyncCallback = @convention(c) (
    UnsafeRawPointer?, Int32, UnsafePointer<CChar>?, UnsafeMutableRawPointer
) -> Void

final class TRLTaskHandle {
    let task: Task<Void, Never>

    init(_ task: Task<Void, Never>) {
        self.task = task
    }
}

@_cdecl("trl_async_task_cancel")
public func trl_async_task_cancel(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<TRLTaskHandle>.fromOpaque(UnsafeRawPointer(handle)).takeRetainedValue().task.cancel()
}

@inline(__always)
private func trlAsyncFail(
    _ error: Error,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let status = trlStatus(from: error)
    trlEncodedError(error).withCString { ptr in
        cb(nil, status, ptr, ctx)
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
    _ ctx: UnsafeMutableRawPointer,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession translation requires macOS 26+"
            )
        }
        let textString = try trlRequireString(text, field: "text")
        let retainedToken = try trlRetainTranslationSessionToken(token)
        let task = Task {
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
                    cb(UnsafeRawPointer(ptr), TRL_OK, nil, ctx)
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
        outTask?.pointee = trlRetain(TRLTaskHandle(task))
    } catch {
        outTask?.pointee = nil
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_session_translations_async")
public func trl_session_translations_async(
    _ token: UnsafeMutableRawPointer?,
    _ requestsJson: UnsafePointer<CChar>?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
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
        let retainedToken = try trlRetainTranslationSessionToken(token)
        let task = Task {
            let boxRef = Unmanaged<TRLTranslationSessionBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                #if TRANSLATION_HAS_MACOS26_SDK
                let box = boxRef.takeUnretainedValue()
                let sessionRequests: [TranslationSession.Request]
                if #available(macOS 26.4, *) {
                    sessionRequests = try requests.map { try trlSessionRequest(from: $0) }
                } else {
                    guard requests.allSatisfy({ $0.attributedSourceText == nil }) else {
                        throw TRLBridgeError.unavailableOnThisMacOS(
                            "TranslationSession.Request.attributedSourceText requires macOS 26.4+"
                        )
                    }
                    sessionRequests = requests.map {
                        TranslationSession.Request(
                            sourceText: $0.sourceText,
                            clientIdentifier: $0.clientIdentifier
                        )
                    }
                }
                let responses = try await box.session().translations(from: sessionRequests)
                let json = try trlEncodeJSON(responses.map(trlTranslationResponsePayload))
                json.withCString { ptr in
                    cb(UnsafeRawPointer(ptr), TRL_OK, nil, ctx)
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
        outTask?.pointee = trlRetain(TRLTaskHandle(task))
    } catch {
        outTask?.pointee = nil
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_session_prepare_translation_async")
public func trl_session_prepare_translation_async(
    _ token: UnsafeMutableRawPointer?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession preparation requires macOS 26+"
            )
        }
        let retainedToken = try trlRetainTranslationSessionToken(token)
        let task = Task {
            let boxRef = Unmanaged<TRLTranslationSessionBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                #if TRANSLATION_HAS_MACOS26_SDK
                let box = boxRef.takeUnretainedValue()
                try await box.session().prepareTranslation()
                cb(UnsafeRawPointer(bitPattern: 1), TRL_OK, nil, ctx)
                #else
                throw TRLBridgeError.unavailableOnThisMacOS(
                    "manual TranslationSession preparation requires the macOS 26 SDK"
                )
                #endif
            } catch {
                trlAsyncFail(error, cb, ctx)
            }
        }
        outTask?.pointee = trlRetain(TRLTaskHandle(task))
    } catch {
        outTask?.pointee = nil
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_language_availability_status_async")
public func trl_language_availability_status_async(
    _ token: UnsafeMutableRawPointer?,
    _ sourceLanguage: UnsafePointer<CChar>?,
    _ targetLanguage: UnsafePointer<CChar>?,
    _ ctx: UnsafeMutableRawPointer,
    _ cb: @escaping TRLAsyncCallback,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
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
        let task = Task {
            let boxRef = Unmanaged<TRLLanguageAvailabilityBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            let status = await availability.status(
                from: trlLanguage(from: source),
                to: target.map { trlLanguage(from: $0) }
            )
            let rawStatus = trlAvailabilityStatusRaw(status)
            cb(UnsafeRawPointer(bitPattern: Int(rawStatus) + 1), TRL_OK, nil, ctx)
        }
        outTask?.pointee = trlRetain(TRLTaskHandle(task))
    } catch {
        outTask?.pointee = nil
        trlAsyncFail(error, cb, ctx)
    }
}

@_cdecl("trl_language_availability_supported_languages_async")
public func trl_language_availability_supported_languages_async(
    _ token: UnsafeMutableRawPointer?,
    _ cb: @escaping TRLAsyncCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) {
    do {
        guard #available(macOS 15.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+"
            )
        }
        let availability = try trlLanguageAvailabilityBox(token).availability
        let retainedToken = try trlRetainLanguageAvailabilityToken(token)
        let task = Task {
            let boxRef = Unmanaged<TRLLanguageAvailabilityBox>.fromOpaque(
                UnsafeRawPointer(retainedToken)
            )
            defer { boxRef.release() }
            do {
                let languages = await availability.supportedLanguages.map(trlLanguageTag).sorted()
                let json = try trlEncodeJSON(languages)
                json.withCString { ptr in
                    cb(UnsafeRawPointer(ptr), TRL_OK, nil, ctx)
                }
            } catch {
                trlAsyncFail(error, cb, ctx)
            }
        }
        outTask?.pointee = trlRetain(TRLTaskHandle(task))
    } catch {
        outTask?.pointee = nil
        trlAsyncFail(error, cb, ctx)
    }
}
