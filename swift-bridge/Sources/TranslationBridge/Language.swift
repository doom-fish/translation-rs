import Foundation

func trlLanguage(from identifier: String) -> Locale.Language {
    Locale.Language(identifier: identifier)
}

func trlLanguageTag(from language: Locale.Language) -> String {
    var parts: [String] = []
    if let languageCode = language.languageCode?.identifier {
        parts.append(languageCode)
    }
    if let script = language.script?.identifier {
        parts.append(script)
    }
    if let region = language.region?.identifier {
        parts.append(region)
    }
    if parts.isEmpty {
        return language.maximalIdentifier
    }
    return parts.joined(separator: "-")
}

@_cdecl("trl_language_canonicalize")
public func trl_language_canonicalize(
    _ identifier: UnsafePointer<CChar>?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let identifier = try trlRequireString(identifier, field: "language identifier")
        guard !identifier.isEmpty else {
            throw TRLBridgeError.invalidArgument("language identifier must be non-empty")
        }
        let canonical = trlLanguageTag(from: trlLanguage(from: identifier))
        outLanguage.pointee = trlCString(canonical)
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}
