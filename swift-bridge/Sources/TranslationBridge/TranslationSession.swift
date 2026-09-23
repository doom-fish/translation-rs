import Foundation
import Translation

struct TRLTranslationRequestPayload: Codable {
    var sourceText: String
    var attributedSourceText: TRLTranslationAttributedStringPayload?
    var clientIdentifier: String?
}

#if TRANSLATION_HAS_MACOS26_SDK
@available(macOS 26.0, *)
final class TRLInstalledTranslationSession: NSObject {
    let session: TranslationSession

    init(configuration: TRLTranslationSessionConfigurationPayload) throws {
        if #available(macOS 26.4, *) {
            session = TranslationSession(
                installedSource: trlLanguage(from: configuration.source),
                target: configuration.target.map { trlLanguage(from: $0) },
                preferredStrategy: try trlStrategy(from: configuration.preferredStrategy)
            )
        } else {
            guard configuration.preferredStrategy == "highFidelity" else {
                throw TRLBridgeError.unavailableOnThisMacOS(
                    "TranslationSession preferredStrategy requires macOS 26.4+"
                )
            }
            session = TranslationSession(
                installedSource: trlLanguage(from: configuration.source),
                target: configuration.target.map { trlLanguage(from: $0) }
            )
        }
        super.init()
    }
}

@available(macOS 26.4, *)
func trlSessionRequest(
    from payload: TRLTranslationRequestPayload
) throws -> TranslationSession.Request {
    if let attributedSourceText = payload.attributedSourceText {
        let attributedSourceText = try trlAttributedString(from: attributedSourceText)
        return TranslationSession.Request(
            sourceText: attributedSourceText,
            clientIdentifier: payload.clientIdentifier
        )
    }
    return TranslationSession.Request(
        sourceText: payload.sourceText,
        clientIdentifier: payload.clientIdentifier
    )
}

@available(macOS 26.0, *)
final class TRLBatchIteratorHolder: @unchecked Sendable {
    private var iterator: TranslationSession.BatchResponse.AsyncIterator

    init(_ iterator: TranslationSession.BatchResponse.AsyncIterator) {
        self.iterator = iterator
    }

    func advance() async throws -> TRLTranslationResponsePayload? {
        guard let response = try await iterator.next() else {
            return nil
        }
        return trlTranslationResponsePayload(from: response)
    }
}

final class TRLPendingBatchResponse: @unchecked Sendable {
    let semaphore = DispatchSemaphore(value: 0)
    let result = TRLAsyncResultBox<TRLTranslationResponsePayload?>()
    var task: Task<Void, Never>?
}

@available(macOS 26.0, *)
final class TRLTranslationBatchResponseBox: NSObject {
    private let holder: TRLBatchIteratorHolder
    private let lock = NSLock()
    private var pending: TRLPendingBatchResponse?

    init(session: TranslationSession, requests: [TranslationSession.Request]) {
        holder = TRLBatchIteratorHolder(session.translate(batch: requests).makeAsyncIterator())
        super.init()
    }

    func nextResponse(timeoutSeconds: TimeInterval) throws -> TRLTranslationResponsePayload? {
        lock.lock()
        let current: TRLPendingBatchResponse
        if let pending {
            current = pending
        } else {
            current = TRLPendingBatchResponse()
            let holder = holder
            current.task = Task {
                do {
                    current.result.set(.success(try await holder.advance()))
                } catch {
                    current.result.set(.failure(error))
                }
                current.semaphore.signal()
            }
            pending = current
        }
        lock.unlock()

        try trlWait(current.semaphore, timeoutSeconds: timeoutSeconds)

        lock.lock()
        if pending === current {
            pending = nil
        }
        lock.unlock()
        guard let result = current.result.get() else {
            throw TRLBridgeError.unknown("missing batch response after the task completed")
        }
        return try result.get()
    }

    deinit {
        pending?.task?.cancel()
    }
}
#endif

final class TRLTranslationSessionBox: NSObject {
    let configuration: TRLTranslationSessionConfigurationPayload
    private let installedSession: AnyObject?

    init(configuration: TRLTranslationSessionConfigurationPayload) throws {
        let canonical = try trlCanonicalizeSessionConfiguration(configuration)
        self.configuration = canonical
        #if TRANSLATION_HAS_MACOS26_SDK
        if #available(macOS 26.0, *) {
            installedSession = try TRLInstalledTranslationSession(configuration: canonical)
        } else {
            installedSession = nil
        }
        #else
        installedSession = nil
        #endif
        super.init()
    }

    #if TRANSLATION_HAS_MACOS26_SDK
    @available(macOS 26.0, *)
    func session() throws -> TranslationSession {
        guard let installedSession = installedSession as? TRLInstalledTranslationSession else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession construction requires macOS 26+"
            )
        }
        return installedSession.session
    }
    #endif
}

func trlTranslationSessionBox(_ token: UnsafeMutableRawPointer?) throws -> TRLTranslationSessionBox {
    guard let token else {
        throw TRLBridgeError.invalidArgument("missing translation session token")
    }
    return trlBorrow(token)
}

@_cdecl("trl_session_new")
public func trl_session_new(
    _ configurationJson: UnsafePointer<CChar>?,
    _ outToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let configuration = try trlDecodeJSON(
            configurationJson,
            as: TRLTranslationSessionConfigurationPayload.self
        )
        outToken.pointee = trlRetain(try TRLTranslationSessionBox(configuration: configuration))
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_release")
public func trl_session_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    trlRelease(token)
}

@_cdecl("trl_session_can_request_downloads")
public func trl_session_can_request_downloads(
    _ token: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "TranslationSession.canRequestDownloads requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        #if TRANSLATION_HAS_MACOS26_SDK
        let session = try box.session()
        outValue.pointee = session.canRequestDownloads ? 1 : 0
        return TRL_OK
        #else
        _ = box
        throw TRLBridgeError.unavailableOnThisMacOS(
            "TranslationSession.canRequestDownloads requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_is_ready")
public func trl_session_is_ready(
    _ token: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "TranslationSession.isReady requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        #if TRANSLATION_HAS_MACOS26_SDK
        outValue.pointee = try trl_block_on_async {
            let session = try box.session()
            let isReady = await session.isReady
            return isReady ? 1 : 0
        }
        return TRL_OK
        #else
        _ = box
        throw TRLBridgeError.unavailableOnThisMacOS(
            "TranslationSession.isReady requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_preferred_strategy")
public func trl_session_preferred_strategy(
    _ token: UnsafeMutableRawPointer?,
    _ outStrategy: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.4, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "TranslationSession preferredStrategy requires macOS 26.4+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        #if TRANSLATION_HAS_MACOS26_SDK
        outStrategy.pointee = try trl_block_on_async {
            let session = try box.session()
            return trlStrategyRaw(session.preferredStrategy)
        }
        return TRL_OK
        #else
        _ = box
        throw TRLBridgeError.unavailableOnThisMacOS(
            "TranslationSession preferredStrategy requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_cancel")
public func trl_session_cancel(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "TranslationSession.cancel() requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        #if TRANSLATION_HAS_MACOS26_SDK
        let session = try box.session()
        session.cancel()
        return TRL_OK
        #else
        _ = box
        throw TRLBridgeError.unavailableOnThisMacOS(
            "TranslationSession.cancel() requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_prepare_translation")
public func trl_session_prepare_translation(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession preparation requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        #if TRANSLATION_HAS_MACOS26_SDK
        try trl_block_on_async {
            try await box.session().prepareTranslation()
        }
        return TRL_OK
        #else
        _ = box
        throw TRLBridgeError.unavailableOnThisMacOS(
            "manual TranslationSession preparation requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_translate_text_json")
public func trl_session_translate_text_json(
    _ token: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ outResponseJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession translation requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        let text = try trlRequireString(text, field: "text")
        #if TRANSLATION_HAS_MACOS26_SDK
        let json = try trl_block_on_async {
            let response = try await box.session().translate(text)
            return try trlEncodeJSON(trlTranslationResponsePayload(from: response))
        }
        outResponseJson.pointee = trlCString(json)
        return TRL_OK
        #else
        _ = box
        _ = text
        throw TRLBridgeError.unavailableOnThisMacOS(
            "manual TranslationSession translation requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_translate_attributed_json")
public func trl_session_translate_attributed_json(
    _ token: UnsafeMutableRawPointer?,
    _ attributedTextJson: UnsafePointer<CChar>?,
    _ outResponseJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.4, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession attributed translation requires macOS 26.4+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        let attributedText = try trlDecodeJSON(
            attributedTextJson,
            as: TRLTranslationAttributedStringPayload.self
        )
        #if TRANSLATION_HAS_MACOS26_SDK
        let json = try trl_block_on_async {
            let response = try await box.session().translate(
                try trlAttributedString(from: attributedText)
            )
            return try trlEncodeJSON(trlTranslationResponsePayload(from: response))
        }
        outResponseJson.pointee = trlCString(json)
        return TRL_OK
        #else
        _ = box
        _ = attributedText
        throw TRLBridgeError.unavailableOnThisMacOS(
            "manual TranslationSession attributed translation requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_translate_batch_json")
public func trl_session_translate_batch_json(
    _ token: UnsafeMutableRawPointer?,
    _ requestsJson: UnsafePointer<CChar>?,
    _ outResponsesJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession translation requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        let requests = try trlDecodeJSON(requestsJson, as: [TRLTranslationRequestPayload].self)
        #if TRANSLATION_HAS_MACOS26_SDK
        let json = try trl_block_on_async {
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
            return try trlEncodeJSON(responses.map(trlTranslationResponsePayload))
        }
        outResponsesJson.pointee = trlCString(json)
        return TRL_OK
        #else
        _ = box
        _ = requests
        throw TRLBridgeError.unavailableOnThisMacOS(
            "manual TranslationSession translation requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_session_translate_batch_stream_json")
public func trl_session_translate_batch_stream_json(
    _ token: UnsafeMutableRawPointer?,
    _ requestsJson: UnsafePointer<CChar>?,
    _ outBatchToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "manual TranslationSession translation requires macOS 26+"
            )
        }
        let box = try trlTranslationSessionBox(token)
        let requests = try trlDecodeJSON(requestsJson, as: [TRLTranslationRequestPayload].self)
        #if TRANSLATION_HAS_MACOS26_SDK
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
        outBatchToken.pointee = trlRetain(
            TRLTranslationBatchResponseBox(
                session: try box.session(),
                requests: sessionRequests
            )
        )
        return TRL_OK
        #else
        _ = box
        _ = requests
        throw TRLBridgeError.unavailableOnThisMacOS(
            "manual TranslationSession translation requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_batch_response_release")
public func trl_batch_response_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    trlRelease(token)
}

@_cdecl("trl_batch_response_next_json")
public func trl_batch_response_next_json(
    _ token: UnsafeMutableRawPointer?,
    _ outResponseJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 26.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "streaming TranslationSession batch iteration requires macOS 26+"
            )
        }
        guard let token else {
            throw TRLBridgeError.invalidArgument("missing translation batch response token")
        }
        #if TRANSLATION_HAS_MACOS26_SDK
        let box: TRLTranslationBatchResponseBox = trlBorrow(token)
        let payload = try box.nextResponse(timeoutSeconds: 60)
        outResponseJson.pointee = try payload
            .map(trlEncodeJSON)
            .flatMap(trlCString)
        return TRL_OK
        #else
        _ = token
        throw TRLBridgeError.unavailableOnThisMacOS(
            "streaming TranslationSession batch iteration requires the macOS 26 SDK"
        )
        #endif
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}
