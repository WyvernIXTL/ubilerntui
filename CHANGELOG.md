# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Added

- Added automatic github dependabot updates.
- Added automatic testing on pull request.
- Added install instructions for installing from github directly.
- Added some colors to cli.

## Fixed

- Fixed github detecting `LICENSE-3RD-PARTY.html` as code.
- Fixed github detecting `about.hbs` as code.
- Fixed Typo in README.
- Fixed crates.io badge not linking to crates.io in README.
- Fixed cli stating 3rd party licenses can be found in LICENSES.html .

## Changed

- Made github action `compile-and-release-on-version-push.yml` more streamlined.
- UI: Renamed "Fragenfortschritt" gauge to "Gesamt-Fragenfortschritt"
- 3rd party license file is generated on release now.
- Added spaces for textwrap in answer list.


## [0.1.1] - 2024-01-14

### Added

- Added some badges to README.md .
- Added license information to README.md .
- Added automatic release on push of version tag.
- Added [cargo about] generated 3rd party licensing information.
- Added link to release page from readme.

### Fixed

- Fixed some typos.

### Changed

- Clarified README.md regarding what exam is supported.
- Changed file extension of LICENSE to LICENSE.txt .


## [0.1.0] - 2024-01-13

### Added

- TUI user interface for learning with selector.
- Cli for reading in a pdf file (`UBI Fragenkatalog`) and parsing it.
- README.md
- CHANGELOG.md


[cargo about]: https://github.com/EmbarkStudios/cargo-about

[Unreleased]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/WyvernIXTL/ubilerntui/releases/tag/v0.1.0