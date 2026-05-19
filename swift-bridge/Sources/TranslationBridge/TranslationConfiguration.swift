import Foundation

struct TRLTranslationSessionConfigurationPayload: Codable {
    var source: String
    var target: String?
    var preferredStrategy: String = "highFidelity"
}

func trlCanonicalizeSessionConfiguration(
    _ configuration: TRLTranslationSessionConfigurationPayload
) throws -> TRLTranslationSessionConfigurationPayload {
    guard !configuration.source.isEmpty else {
        throw TRLBridgeError.invalidArgument(
            "translation session source language must be non-empty"
        )
    }
    if let target = configuration.target, target.isEmpty {
        throw TRLBridgeError.invalidArgument(
            "translation session target language must be non-empty when provided"
        )
    }
    return TRLTranslationSessionConfigurationPayload(
        source: trlLanguageTag(from: trlLanguage(from: configuration.source)),
        target: configuration.target.map { trlLanguageTag(from: trlLanguage(from: $0)) },
        preferredStrategy: try trlCanonicalizeStrategyIdentifier(
            configuration.preferredStrategy
        )
    )
}
