# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## 0.1.2 - 2024-4-18
- Embedded-hal was raised to 1.0.0
- linux-embedded-hal raised to 0.4 for embedded-hal 1.0.0 support
- embedded-hal-mock specify embedded-hal 1.0.0 feature set

## 0.1.1 - 2023-12-11

### Changed

- MSRV was raised to 1.62.0.

### Fixed

- MAX17048 & MAX17049 `charge_rate` correctly handles negative values (such as when discharging). See: [PR #2](https://github.com/eldruin/max170xx-rs/pull/2)

## 0.1.0 - 2020-07-19

Initial release of the driver to crates.io.

[Unreleased]: https://github.com/eldruin/max170xx-rs/compare/v0.1.0...HEAD
