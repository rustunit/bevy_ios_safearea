// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "bevy_ios_safearea",
    platforms: [.iOS("15.0")],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "bevy_ios_safearea",
            targets: ["bevy_ios_safearea"]),
    ],
    dependencies: [],
    targets: [.target(name: "bevy_ios_safearea")]
)
