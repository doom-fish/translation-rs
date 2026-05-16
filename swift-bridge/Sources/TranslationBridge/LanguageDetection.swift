import Foundation
import NaturalLanguage

func trlDetectLanguageTag(_ text: String) -> String? {
    let recognizer = NLLanguageRecognizer()
    recognizer.processString(text)
    return recognizer.dominantLanguage?.rawValue
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
    } catch let error as TRLBridgeError {
        outErrorMessage?.pointee = trlCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = trlCString(error.localizedDescription)
        return TRL_UNKNOWN
    }
}
