import Foundation
import NaturalLanguage

func trlDetectLanguageTag(_ text: String) -> String? {
    let recognizer = NLLanguageRecognizer()
    recognizer.processString(text)
    return recognizer.dominantLanguage.map { trlLanguageTag(from: trlLanguage(from: $0.rawValue)) }
}

@_cdecl("trl_detect_language")
public func trl_detect_language(
    _ text: UnsafePointer<CChar>?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let text = try trlRequireString(text, field: "text")
        outLanguage.pointee = trlDetectLanguageTag(text).flatMap(trlCString)
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}
