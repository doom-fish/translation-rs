import Foundation
import Translation

struct TRLTranslationSessionConfigurationPayload: Codable {
    var source: String
    var target: String
}

struct TRLTranslationRequestPayload: Codable {
    var sourceText: String
    var clientIdentifier: String?
}

struct TRLTranslationResponsePayload: Codable {
    var sourceLanguage: String
    var targetLanguage: String
    var sourceText: String
    var targetText: String
    var clientIdentifier: String?
}

final class TRLTranslationSessionBox: NSObject {
    let configuration: TRLTranslationSessionConfigurationPayload

    init(configuration: TRLTranslationSessionConfigurationPayload) {
        self.configuration = configuration
        super.init()
    }

    @available(macOS 26.0, *)
    func makeSession() -> TranslationSession {
        TranslationSession(
            installedSource: trlLanguage(from: configuration.source),
            target: trlLanguage(from: configuration.target)
        )
    }
}

func trlTranslationSessionBox(_ token: UnsafeMutableRawPointer?) throws -> TRLTranslationSessionBox {
    guard let token else {
        throw TRLBridgeError.invalidArgument("missing translation session token")
    }
    return trlBorrow(token)
}

@available(macOS 15.0, *)
func trlTranslationResponsePayload(from response: TranslationSession.Response) -> TRLTranslationResponsePayload {
    TRLTranslationResponsePayload(
        sourceLanguage: trlLanguageTag(from: response.sourceLanguage),
        targetLanguage: trlLanguageTag(from: response.targetLanguage),
        sourceText: response.sourceText,
        targetText: response.targetText,
        clientIdentifier: response.clientIdentifier
    )
}

@_cdecl("trl_session_new")
public func trl_session_new(
    _ configurationJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let configuration = try trlDecodeJSON(
            configurationJson,
            as: TRLTranslationSessionConfigurationPayload.self
        )
        if configuration.source.isEmpty || configuration.target.isEmpty {
            throw TRLBridgeError.invalidArgument(
                "translation session source and target languages must be non-empty"
            )
        }
        return trlRetain(TRLTranslationSessionBox(configuration: configuration))
    } catch let error as TRLBridgeError {
        outErrorMessage?.pointee = trlCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = trlCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("trl_session_release")
public func trl_session_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    trlRelease(token)
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
        try trl_block_on_async {
            try await box.makeSession().prepareTranslation()
        }
        return TRL_OK
    } catch let error as TRLBridgeError {
        outErrorMessage?.pointee = trlCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = trlCString(error.localizedDescription)
        return trlStatus(from: error)
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
        let json = try trl_block_on_async {
            let response = try await box.makeSession().translate(text)
            return try trlEncodeJSON(trlTranslationResponsePayload(from: response))
        }
        outResponseJson.pointee = trlCString(json)
        return TRL_OK
    } catch let error as TRLBridgeError {
        outErrorMessage?.pointee = trlCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = trlCString(error.localizedDescription)
        return trlStatus(from: error)
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
        let json = try trl_block_on_async {
            let sessionRequests = requests.map {
                TranslationSession.Request(
                    sourceText: $0.sourceText,
                    clientIdentifier: $0.clientIdentifier
                )
            }
            let responses = try await box.makeSession().translations(from: sessionRequests)
            return try trlEncodeJSON(responses.map(trlTranslationResponsePayload))
        }
        outResponsesJson.pointee = trlCString(json)
        return TRL_OK
    } catch let error as TRLBridgeError {
        outErrorMessage?.pointee = trlCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = trlCString(error.localizedDescription)
        return trlStatus(from: error)
    }
}
