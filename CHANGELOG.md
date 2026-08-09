# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.7.0] - 2026-08-09

### Changed
* **BREAKING (packaging only, not API):** read `-[UIView safeAreaInsets]` directly via `objc2`/`objc2-ui-kit` instead of an `extern "C"` shim into a companion Swift Package. Consumers no longer add this crate as an Xcode SPM dependency — the Rust dependency is now the whole story. Public API (`IosSafeAreaPlugin`, `IosSafeAreaResource`, `IosSafeArea`) is unchanged.

### Removed
* the companion Swift Package (`Package.swift`, `Sources/`) — no longer needed

## [0.6.0] - 2026-07-28

### Changed
* upgrade to bevy `0.19`

## [0.5.2] - 2026-07-28

### Fixed
* fix iOS startup crash: read the safe area insets on `WindowCreated` instead of `Startup`, where the winit window may not be registered yet [#14](https://github.com/rustunit/bevy_ios_safearea/pull/14)

## [0.5.1] - 2026-01-16

### Changed
* use granular bevy_* parts as dependencies
 
## [0.5.0] - 2026-01-16

### Changed
* upgrade to bevy `0.18`

## [0.4.0] - 2025-10-18

### Changed
* upgrade to bevy `0.17`

## [0.3.0] - 2025-04-26

### Changed
* upgrade to bevy 0.16

## [0.2.0] - 2025-01-20

### Changed

* support `left`&`right` safearea [#9](https://github.com/rustunit/bevy_ios_safearea/pull/9)
* make `IosSafeArea` a `SystemParam` as main interface to use this crate [#10](https://github.com/rustunit/bevy_ios_safearea/pull/10)

## [0.1.6] - 2025-01-15

### Fixed

* mimic `winit` docs target, hoping it fixes docs.rs

### Added

* CI tests

## [0.1.5] - 2025-01-15

### Fixed
* setup ARM iOS as default docs target, hoping it fixes docs.rs

## [0.1.4] - 2025-01-14

### Fixed
* do not commit lockfile, hoping it fixes docs.rs

## [0.1.3] - 2025-01-14

### Fixed
* fix docs.rs [#4](https://github.com/rustunit/bevy_ios_safearea/pull/4)

## [0.1.2] - 2025-01-12

### Fixed
* fix 0.1.1 not compiling on iOS

## [0.1.1] - 2025-01-12

### Changed
* dont spam the log (use `tracing::debug`)

## [0.1.0] - 2025-01-12

Initial release
