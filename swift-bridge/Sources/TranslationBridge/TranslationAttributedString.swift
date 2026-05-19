import Foundation
import Translation

struct TRLTranslationAttributedRunPayload: Codable {
    var start: Int
    var end: Int
    var value: Bool
}

struct TRLTranslationAttributedStringPayload: Codable {
    var text: String
    var skipTranslationRuns: [TRLTranslationAttributedRunPayload]
}

#if TRANSLATION_HAS_MACOS26_SDK
@available(macOS 26.4, *)
private func trlStringIndex(in text: String, offset: Int) throws -> String.Index {
    guard offset >= 0,
          let index = text.index(text.startIndex, offsetBy: offset, limitedBy: text.endIndex)
    else {
        throw TRLBridgeError.invalidArgument(
            "attributed text offset \(offset) is outside '\(text)'"
        )
    }
    return index
}

@available(macOS 26.4, *)
func trlAttributedString(from payload: TRLTranslationAttributedStringPayload) throws -> AttributedString {
    var attributed = AttributedString(payload.text)
    for run in payload.skipTranslationRuns.sorted(by: { $0.start < $1.start }) {
        guard run.start <= run.end else {
            throw TRLBridgeError.invalidArgument(
                "attributed text range \(run.start)..\(run.end) is invalid"
            )
        }
        let lower = try trlStringIndex(in: payload.text, offset: run.start)
        let upper = try trlStringIndex(in: payload.text, offset: run.end)
        guard let range = Range(lower..<upper, in: attributed) else {
            throw TRLBridgeError.invalidArgument(
                "attributed text range \(run.start)..\(run.end) could not be mapped"
            )
        }
        attributed[range].translation.skipsTranslation = run.value
    }
    return attributed
}

@available(macOS 26.4, *)
func trlTranslationAttributedStringPayload(
    from attributed: AttributedString
) -> TRLTranslationAttributedStringPayload {
    var offset = 0
    var runs: [TRLTranslationAttributedRunPayload] = []
    for run in attributed.runs {
        let segment = String(attributed[run.range].characters)
        let segmentLength = segment.count
        if let value = run.translation.skipsTranslation {
            runs.append(
                TRLTranslationAttributedRunPayload(
                    start: offset,
                    end: offset + segmentLength,
                    value: value
                )
            )
        }
        offset += segmentLength
    }
    return TRLTranslationAttributedStringPayload(
        text: String(attributed.characters),
        skipTranslationRuns: runs
    )
}
#endif
