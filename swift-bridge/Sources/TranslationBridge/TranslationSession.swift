import Foundation
import Translation

struct TRLTranslationRequestPayload: Codable {
    var sourceText: String
    var clientIdentifier: String?
}

#if TRANSLATION_HAS_MACOS26_SDK
@available(macOS 26.0, *)
final class TRLInstalledTranslationSession: NSObject {
    let session: TranslationSession

    init(configuration: TRLTranslationSessionConfigurationPayload) {
        session = TranslationSession(
            installedSource: trlLanguage(from: configuration.source),
            target: configuration.target.map { trlLanguage(from: $0) }
        )
        super.init()
    }
}

@available(macOS 26.0, *)
final class TRLTranslationBatchResponseBox: NSObject {
    private var iterator: TranslationSession.BatchResponse.AsyncIterator

    init(session: TranslationSession, requests: [TranslationSession.Request]) {
        iterator = session.translate(batch: requests).makeAsyncIterator()
        super.init()
    }

    func nextResponse() async throws -> TRLTranslationResponsePayload? {
        guard let response = try await iterator.next() else {
            return nil
        }
        return trlTranslationResponsePayload(from: response)
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
            installedSession = TRLInstalledTranslationSession(configuration: canonical)
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
            let sessionRequests = requests.map {
                TranslationSession.Request(
                    sourceText: $0.sourceText,
                    clientIdentifier: $0.clientIdentifier
                )
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
        let sessionRequests = requests.map {
            TranslationSession.Request(
                sourceText: $0.sourceText,
                clientIdentifier: $0.clientIdentifier
            )
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
        let payload = try trl_block_on_async {
            try await box.nextResponse()
        }
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
