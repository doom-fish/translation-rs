// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "TranslationBridge",
    platforms: [
        .macOS("15.0")
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
