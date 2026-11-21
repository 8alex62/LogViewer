# Changelog

All notable changes to this project are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Repository layout from the specs: Rust workspace in `core/` with the Clean Architecture crates (`logviewer-domain`, `-application`, `-infrastructure`, `-server`, `-wasm`), first `proto/` contract, `docs/`, `samples/`.
- CI for the Core: fmt, clippy and tests on Linux and Windows, WASM build check, `buf` lint.
- Code coverage of the Core with `cargo-llvm-cov`, imported by a SonarCloud analysis run from the CI.
- Dependency checks in CI with `cargo-deny` (advisories, licenses, duplicate crates, sources) and weekly Dependabot updates for Cargo and GitHub Actions.
- README with goals, tech stack, architecture, setup, run and test commands; `CONTRIBUTING.md` with the branch model, commit conventions and PR checklist.
