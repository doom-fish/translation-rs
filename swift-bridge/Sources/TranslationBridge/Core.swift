import Foundation
import NaturalLanguage
import Translation

let TRL_OK: Int32 = 0
let TRL_INVALID_ARGUMENT: Int32 = -1
let TRL_UNAVAILABLE_ON_THIS_MACOS: Int32 = -2
let TRL_TIMED_OUT: Int32 = -3
let TRL_UNSUPPORTED_SOURCE_LANGUAGE: Int32 = -10
let TRL_UNSUPPORTED_TARGET_LANGUAGE: Int32 = -11
let TRL_UNSUPPORTED_LANGUAGE_PAIRING: Int32 = -12
let TRL_UNABLE_TO_IDENTIFY_LANGUAGE: Int32 = -13
let TRL_NOTHING_TO_TRANSLATE: Int32 = -14
let TRL_ALREADY_CANCELLED: Int32 = -15
let TRL_NOT_INSTALLED: Int32 = -16
let TRL_FRAMEWORK_ERROR: Int32 = -20
let TRL_UNKNOWN: Int32 = -99

@_cdecl("trl_string_free")
public func trl_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@inline(__always)
func trlCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func trlRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func trlBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as type: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func trlRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

enum TRLBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case unavailableOnThisMacOS(String)
    case timedOut(String)
    case unknown(String)

    var description: String {
        switch self {
        case let .invalidArgument(message),
            let .unavailableOnThisMacOS(message),
            let .timedOut(message),
            let .unknown(message):
            return message
        }
    }

    var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return TRL_INVALID_ARGUMENT
        case .unavailableOnThisMacOS:
            return TRL_UNAVAILABLE_ON_THIS_MACOS
        case .timedOut:
            return TRL_TIMED_OUT
        case .unknown:
            return TRL_UNKNOWN
        }
    }
}

func trlStatus(from error: Error) -> Int32 {
    if let bridgeError = error as? TRLBridgeError {
        return bridgeError.statusCode
    }
    if #available(macOS 15.0, *) {
        if TranslationError.unsupportedSourceLanguage ~= error {
            return TRL_UNSUPPORTED_SOURCE_LANGUAGE
        }
        if TranslationError.unsupportedTargetLanguage ~= error {
            return TRL_UNSUPPORTED_TARGET_LANGUAGE
        }
        if TranslationError.unsupportedLanguagePairing ~= error {
            return TRL_UNSUPPORTED_LANGUAGE_PAIRING
        }
        if TranslationError.unableToIdentifyLanguage ~= error {
            return TRL_UNABLE_TO_IDENTIFY_LANGUAGE
        }
        if TranslationError.nothingToTranslate ~= error {
            return TRL_NOTHING_TO_TRANSLATE
        }
        if #available(macOS 26.0, *) {
            if TranslationError.alreadyCancelled ~= error {
                return TRL_ALREADY_CANCELLED
            }
            if TranslationError.notInstalled ~= error {
                return TRL_NOT_INSTALLED
            }
        }
        if TranslationError.internalError ~= error {
            return TRL_FRAMEWORK_ERROR
        }
    }
    return TRL_FRAMEWORK_ERROR
}

final class TRLAsyncResultBox<T>: @unchecked Sendable {
    private let lock = NSLock()
    private var storedResult: Result<T, Error>?

    func set(_ result: Result<T, Error>) {
        lock.lock()
        storedResult = result
        lock.unlock()
    }

    func get() -> Result<T, Error>? {
        lock.lock()
        defer { lock.unlock() }
        return storedResult
    }
}

public func trl_block_on_async<T>(
    timeoutSeconds: TimeInterval = 60,
    pollIntervalSeconds: TimeInterval = 0.01,
    work: @escaping () async throws -> T
) throws -> T {
    let box = TRLAsyncResultBox<T>()
    Task {
        do {
            box.set(.success(try await work()))
        } catch {
            box.set(.failure(error))
        }
    }

    let deadline = Date().addingTimeInterval(timeoutSeconds)
    while box.get() == nil && Date() < deadline {
        RunLoop.current.run(mode: .default, before: Date().addingTimeInterval(pollIntervalSeconds))
    }

    guard let result = box.get() else {
        throw TRLBridgeError.timedOut(
            "Translation.framework async call timed out after \(Int(timeoutSeconds)) seconds"
        )
    }
    return try result.get()
}

func trlEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw TRLBridgeError.unknown("failed to encode JSON as UTF-8")
    }
    return string
}

func trlDecodeJSON<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let cString else {
        throw TRLBridgeError.invalidArgument("missing JSON payload")
    }
    let data = Data(String(cString: cString).utf8)
    do {
        return try JSONDecoder().decode(T.self, from: data)
    } catch {
        throw TRLBridgeError.invalidArgument("invalid JSON payload: \(error.localizedDescription)")
    }
}

func trlRequireString(_ cString: UnsafePointer<CChar>?, field: String) throws -> String {
    guard let cString else {
        throw TRLBridgeError.invalidArgument("missing \(field)")
    }
    return String(cString: cString)
}

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
