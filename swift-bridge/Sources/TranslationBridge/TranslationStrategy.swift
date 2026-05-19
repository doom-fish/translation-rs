import Foundation
import Translation

func trlCanonicalizeStrategyIdentifier(_ identifier: String) throws -> String {
    switch identifier {
    case "highFidelity", "lowLatency":
        return identifier
    default:
        throw TRLBridgeError.invalidArgument(
            "unsupported preferred strategy '\(identifier)'"
        )
    }
}

#if TRANSLATION_HAS_MACOS26_SDK
@available(macOS 26.4, *)
func trlStrategyRaw(_ strategy: TranslationSession.Strategy) -> Int32 {
    if strategy == .highFidelity {
        return 0
    }
    if strategy == .lowLatency {
        return 1
    }
    return TRL_UNKNOWN
}

@available(macOS 26.4, *)
func trlStrategy(from raw: Int32) throws -> TranslationSession.Strategy {
    switch raw {
    case 0:
        return .highFidelity
    case 1:
        return .lowLatency
    default:
        throw TRLBridgeError.invalidArgument(
            "unsupported preferred strategy raw value '\(raw)'"
        )
    }
}

@available(macOS 26.4, *)
func trlStrategy(from identifier: String) throws -> TranslationSession.Strategy {
    switch try trlCanonicalizeStrategyIdentifier(identifier) {
    case "highFidelity":
        return .highFidelity
    case "lowLatency":
        return .lowLatency
    default:
        throw TRLBridgeError.invalidArgument(
            "unsupported preferred strategy '\(identifier)'"
        )
    }
}
#endif
