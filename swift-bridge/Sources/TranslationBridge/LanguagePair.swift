import Foundation

struct TRLLanguagePairPayload: Codable {
    var source: String
    var target: String?
}

@_cdecl("trl_language_pair_canonicalize_json")
public func trl_language_pair_canonicalize_json(
    _ sourceLanguage: UnsafePointer<CChar>?,
    _ targetLanguage: UnsafePointer<CChar>?,
    _ outPairJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let source = try trlRequireString(sourceLanguage, field: "source language")
        guard !source.isEmpty else {
            throw TRLBridgeError.invalidArgument("source language must be non-empty")
        }
        let target = targetLanguage.map(String.init(cString:))
        if let target, target.isEmpty {
            throw TRLBridgeError.invalidArgument("target language must be non-empty when provided")
        }

        let payload = TRLLanguagePairPayload(
            source: trlLanguageTag(from: trlLanguage(from: source)),
            target: target.map { trlLanguageTag(from: trlLanguage(from: $0)) }
        )
        outPairJson.pointee = trlCString(try trlEncodeJSON(payload))
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}
