import Foundation
import Translation

struct TRLTranslationResponsePayload: Codable {
    var sourceLanguage: String
    var targetLanguage: String
    var sourceText: String
    var targetText: String
    var attributedSourceText: TRLTranslationAttributedStringPayload?
    var attributedTargetText: TRLTranslationAttributedStringPayload?
    var clientIdentifier: String?
}

@available(macOS 15.0, *)
func trlTranslationResponsePayload(
    from response: TranslationSession.Response
) -> TRLTranslationResponsePayload {
    TRLTranslationResponsePayload(
        sourceLanguage: trlLanguageTag(from: response.sourceLanguage),
        targetLanguage: trlLanguageTag(from: response.targetLanguage),
        sourceText: response.sourceText,
        targetText: response.targetText,
        attributedSourceText: {
            if #available(macOS 26.4, *) {
                return response.attributedSourceText.map(trlTranslationAttributedStringPayload)
            }
            return nil
        }(),
        attributedTargetText: {
            if #available(macOS 26.4, *) {
                return response.attributedTargetText.map(trlTranslationAttributedStringPayload)
            }
            return nil
        }(),
        clientIdentifier: response.clientIdentifier
    )
}
