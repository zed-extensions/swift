// swift-tools-version:5.9

import PackageDescription

let package = Package(
    name: "my-package",
    products: [
        .library(name: "MyTarget", targets: ["MyTarget"]),
    ],
    targets: [
        .target(name: "MyTarget"),
        .testTarget(name: "MyTargetTests", dependencies: ["MyTarget"])
    ]
)
