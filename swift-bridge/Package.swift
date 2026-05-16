// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "TranslationBridge",
    platforms: [
        .macOS(.v14)
    ],
    products: [
        .library(
            name: "TranslationBridge",
            type: .static,
            targets: ["TranslationBridge"])
    ],
    targets: [
        .target(
            name: "TranslationBridge",
            path: "Sources/TranslationBridge",
            publicHeadersPath: "include")
    ]
)
