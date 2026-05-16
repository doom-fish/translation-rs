import Foundation
import Translation

@available(macOS 15.0, *)
final class TRLLanguageAvailabilityBox: NSObject {
    let availability = LanguageAvailability()
}

@available(macOS 15.0, *)
func trlLanguageAvailabilityBox(_ token: UnsafeMutableRawPointer?) throws -> TRLLanguageAvailabilityBox {
    guard let token else {
        throw TRLBridgeError.invalidArgument("missing language availability token")
    }
    return trlBorrow(token)
}

@available(macOS 15.0, *)
func trlAvailabilityStatusRaw(_ status: LanguageAvailability.Status) -> Int32 {
    switch status {
    case .installed:
        return 0
    case .supported:
        return 1
    case .unsupported:
        return 2
    @unknown default:
        return 99
    }
}

@available(macOS 15.0, *)
func trlStatusForText(
    availability: LanguageAvailability,
    text: String,
    targetLanguage: Locale.Language?
) async throws -> LanguageAvailability.Status {
    do {
        return try await availability.status(for: text, to: targetLanguage)
    } catch {
        if TranslationError.unableToIdentifyLanguage ~= error,
           let detectedLanguage = trlDetectLanguageTag(text)
        {
            return await availability.status(
                from: trlLanguage(from: detectedLanguage),
                to: targetLanguage
            )
        }
        throw error
    }
}

@_cdecl("trl_language_availability_new")
public func trl_language_availability_new() -> UnsafeMutableRawPointer? {
    if #available(macOS 15.0, *) {
        return trlRetain(TRLLanguageAvailabilityBox())
    }
    return nil
}

@_cdecl("trl_language_availability_release")
public func trl_language_availability_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    trlRelease(token)
}

@_cdecl("trl_language_availability_supported_languages_json")
public func trl_language_availability_supported_languages_json(
    _ token: UnsafeMutableRawPointer?,
    _ outLanguagesJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+"
            )
        }
        let box = try trlLanguageAvailabilityBox(token)
        let json = try trl_block_on_async {
            let languages = await box.availability.supportedLanguages
                .map(trlLanguageTag)
                .sorted()
            return try trlEncodeJSON(languages)
        }
        outLanguagesJson.pointee = trlCString(json)
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_language_availability_status_from_to")
public func trl_language_availability_status_from_to(
    _ token: UnsafeMutableRawPointer?,
    _ sourceLanguage: UnsafePointer<CChar>?,
    _ targetLanguage: UnsafePointer<CChar>?,
    _ outStatus: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+"
            )
        }
        let box = try trlLanguageAvailabilityBox(token)
        let source = try trlRequireString(sourceLanguage, field: "source language")
        let target = targetLanguage.map(String.init(cString:))
        let status = try trl_block_on_async {
            let availabilityStatus = await box.availability.status(
                from: trlLanguage(from: source),
                to: target.map { trlLanguage(from: $0) }
            )
            return trlAvailabilityStatusRaw(availabilityStatus)
        }
        outStatus.pointee = status
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}

@_cdecl("trl_language_availability_status_for_text")
public func trl_language_availability_status_for_text(
    _ token: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ targetLanguage: UnsafePointer<CChar>?,
    _ outStatus: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 15.0, *) else {
            throw TRLBridgeError.unavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+"
            )
        }
        let box = try trlLanguageAvailabilityBox(token)
        let text = try trlRequireString(text, field: "text")
        let target = targetLanguage.map(String.init(cString:))
        let status = try trl_block_on_async {
            let availabilityStatus = try await trlStatusForText(
                availability: box.availability,
                text: text,
                targetLanguage: target.map { trlLanguage(from: $0) }
            )
            return trlAvailabilityStatusRaw(availabilityStatus)
        }
        outStatus.pointee = status
        return TRL_OK
    } catch {
        return trlWriteError(outErrorMessage, error)
    }
}
