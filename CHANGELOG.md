# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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
