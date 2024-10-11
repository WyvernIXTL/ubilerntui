# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.9] - 2024-10-11

### Changed

- Licenses are compressed resulting in smaller binary size.
- Git history is even more convoluted.
- Removed this garbage fucktard release gh action.

## [0.1.6] - 2024-05-28

### Added

- Added asciinema video.
- Added snap badge.

### Changed

- Fixed `license` and `license-file` coexisting.
- Fixed snap confinement.
- Fixed error in readme.


## [0.1.5] - 2024-05-25

### Added

- Added a warning in case too many or too few questions were loaded.
- Added snap package support.
- Added `--third-party-licenses`.
- Added deb packaging via [cargo-deb](https://github.com/kornelski/cargo-deb).


### Changed

- Switched to [3pl](https://github.com/ankane/cargo-3pl) license scanner.


### Removed

- Removed `cargo about` ci.


## [0.1.4]  - 2024-01-17

### Changed

- Scramble is much better implemented now.

## [0.1.3]  - 2024-01-17

### Added

- Added install instruction for scoop.
- Added CHANGELOG.md to be included in future releases.

### Fixed

- Fixed textwrap ignoring padding.
- Fixed dependabot automerge action.
- Fixed CHANGELOG.md bad headers.
- Fixed bad rng for answer list.

## [0.1.2] - 2024-01-16

### Added

- Added automatic github dependabot updates.
- Added automatic testing on pull request.
- Added install instructions for installing from github directly.
- Added some colors to cli.
- Added checksums to releases.

### Fixed

- Fixed github detecting `LICENSE-3RD-PARTY.html` as code.
- Fixed github detecting `about.hbs` as code.
- Fixed Typo in README.
- Fixed crates.io badge not linking to crates.io in README.
- Fixed cli stating 3rd party licenses can be found in LICENSES.html .
- Fixed missing attribution to language pattern authors.

### Changed

- Made github action `compile-and-release-on-version-push.yml` more streamlined.
- UI: Renamed "Fragenfortschritt" gauge to "Gesamt-Fragenfortschritt"
- 3rd party license file is generated on release now.
- Added spaces for textwrap in answer list.
- Embed only german dictionary. This saves 3MB roughly.
- Changed screenshots to reflect changes in ui.


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

[Unreleased]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.9...HEAD
[0.1.9]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.6...v0.1.9
[0.1.6]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/WyvernIXTL/ubilerntui/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/WyvernIXTL/ubilerntui/releases/tag/v0.1.0