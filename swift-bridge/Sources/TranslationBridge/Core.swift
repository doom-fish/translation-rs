import Dispatch
import Foundation

let TRL_OK: Int32 = 0
let TRL_INVALID_ARGUMENT: Int32 = -1
let TRL_UNAVAILABLE_ON_THIS_MACOS: Int32 = -2
let TRL_TIMED_OUT: Int32 = -3
let TRL_MAIN_RUN_LOOP_NOT_RUNNING: Int32 = -4
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
    Unmanaged<T>.fromOpaque(UnsafeRawPointer(ptr)).takeUnretainedValue()
}

@inline(__always)
func trlRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(ptr)).release()
}

enum TRLBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case unavailableOnThisMacOS(String)
    case timedOut(String)
    case mainRunLoopNotRunning(String)
    case unknown(String)

    var description: String {
        switch self {
        case let .invalidArgument(message),
            let .unavailableOnThisMacOS(message),
            let .timedOut(message),
            let .mainRunLoopNotRunning(message),
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
        case .mainRunLoopNotRunning:
            return TRL_MAIN_RUN_LOOP_NOT_RUNNING
        case .unknown:
            return TRL_UNKNOWN
        }
    }
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

let TRL_MAIN_QUEUE_GRACE_SECONDS: TimeInterval = 10

final class TRLMainQueueProbe: @unchecked Sendable {
    private let lock = NSLock()
    private var serviced = false

    init() {
        DispatchQueue.main.async { [self] in
            lock.lock()
            serviced = true
            lock.unlock()
        }
    }

    var wasServiced: Bool {
        lock.lock()
        defer { lock.unlock() }
        return serviced
    }
}

func trlWait(
    _ semaphore: DispatchSemaphore,
    timeoutSeconds: TimeInterval
) throws {
    let timedOut = TRLBridgeError.timedOut(
        "Translation.framework async call timed out after \(Int(timeoutSeconds)) seconds"
    )
    if Thread.isMainThread {
        let deadline = Date().addingTimeInterval(timeoutSeconds)
        while semaphore.wait(timeout: .now()) == .timedOut {
            if Date() >= deadline {
                throw timedOut
            }
            RunLoop.current.run(mode: .default, before: Date().addingTimeInterval(0.01))
        }
        return
    }
    let deadline = DispatchTime.now() + timeoutSeconds
    let probe = TRLMainQueueProbe()
    let grace = DispatchTime.now() + min(TRL_MAIN_QUEUE_GRACE_SECONDS, timeoutSeconds)
    if semaphore.wait(timeout: grace) == .success {
        return
    }
    if !probe.wasServiced {
        throw TRLBridgeError.mainRunLoopNotRunning(
            "Translation.framework needs the main thread to run its run loop, and the main " +
                "queue was not serviced within \(Int(TRL_MAIN_QUEUE_GRACE_SECONDS)) seconds"
        )
    }
    if semaphore.wait(timeout: deadline) == .timedOut {
        throw timedOut
    }
}

public func trl_block_on_async<T>(
    timeoutSeconds: TimeInterval = 60,
    work: @escaping @Sendable () async throws -> T
) throws -> T {
    let semaphore = DispatchSemaphore(value: 0)
    let box = TRLAsyncResultBox<T>()

    let task = Task {
        do {
            box.set(.success(try await work()))
        } catch {
            box.set(.failure(error))
        }
        semaphore.signal()
    }

    do {
        try trlWait(semaphore, timeoutSeconds: timeoutSeconds)
    } catch {
        task.cancel()
        throw error
    }

    guard let result = box.get() else {
        throw TRLBridgeError.unknown("missing async result after the task completed")
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
