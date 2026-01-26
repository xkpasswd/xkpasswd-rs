# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-01-26

### Changed

- **Rust ecosystem modernization**: Updated all dependencies to latest versions
  - wasm-bindgen 0.2.83 → 0.2.108
  - toml 0.5.9 → 0.8.23 (API migration)
  - clap 4.0.26 → 4.5.54
  - dirs 4 → 5, rand, getrandom, log, stderrlog
- **CI/CD modernization**: Migrated from deprecated `actions-rs` to `dtolnay/rust-toolchain@stable`
  - Added `Swatinem/rust-cache@v2` for dependency caching
  - Added Node.js caching with `npm ci` for deterministic builds
  - Pinned wasm-pack to 0.14.0 for reproducibility
  - Replaced Surge.sh with Netlify for PR preview deployments (more reliable, auto PR comments)
- **Frontend updates**: Updated npm dependencies
  - Vite 4 → 5, TypeScript 5, Preact 10.28, Tailwind 3.4
  - Fixed Preact TypeScript types for `useState` dispatch
- **Tooling**: Migrated from asdf `.tool-versions` to `mise.toml` with Node 22.22.0

### Fixed

- Fixed wasm-pack build compatibility with Rust 1.93+ and wasm-opt 125
- Fixed clippy warnings (lifetime elision, entry API, `?` operator)
- Improved dictionary loader error handling (graceful skip for invalid files)

### Added

- Crate-level documentation with working doc test example
- Code coverage measurement in CI with cargo-llvm-cov and Codecov integration
- Fast CI tool installation via cargo-binstall (wasm-pack) and taiki-e/install-action

## [1.0.1] - 2023-04-11

### Fixed

- Fix bloated bundle size of wasm files for languages other than EN
- Refactor build pipeline for CLI app & Arch Linux distribution

## [1.0.0] - 2023-03-27

### Added

- Support more languages:
  - German
  - Spanish
  - French
  - Portuguese
- Production site deployed at https://xkpasswd.github.io

## [0.1.0] - 2022-12-28

### Added

- Initial release
- XKCD-style password generator with CLI and WASM targets
- English language support
- Configurable presets: AppleID, Web16, Web32, WiFi, XKCD

[1.1.0]: https://github.com/xkpasswd/xkpasswd-rs/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/xkpasswd/xkpasswd-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/xkpasswd/xkpasswd-rs/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/xkpasswd/xkpasswd-rs/releases/tag/v0.1.0
