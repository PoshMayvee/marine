# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.24.0] - 2022-12-06

### Added
- [**breaking**] prohibit going out of service_dir in app-service (#244)
- *(fluence-app-service)* make base path field optional in ConfigContext interface (#202)

### Fixed
- *(runtime)* detect mapped/preopened dirs conflicts before wasmer-wasi crashes (#223)
- [**breaking**] bump minor versions where it was required in #189 (#212)
- fix tests after renaming (#174)

### Other
- *(deps)* update all non-major rust dependencies (#211)
- *(build)* fix clippy warnings (#213)
- Update Rust crate semver to v1 (#198)
- Update all non-major Rust dependencies (#204)
- Update Rust crate serde_with to v2 (#203)
- Update Rust crate cmd_lib to v1 (#194)
- Update Rust crate pretty_assertions to v1 (#196)
- Update all non-major Rust dependencies (#189)
- Rework module searching on filesystem (#184)
- bump crate versions that used marine-rs-sdk-main 0.6.15 (#185)
- Support marine-rs-sdk 0.7.0  (#180)
- Add tests for wasm memory leaks when passing/returning records (#182)
- Add record destruction test (#181)
- Migrate  marine tests to github-actions (#178)
- Fix value after table problem in TomlMarineNamedModuleConfig(#175)
- improve "interface" command output readability (#169)
- Rename `FaaS` to `Marine`, `Runtime` to `Core` (#172)
