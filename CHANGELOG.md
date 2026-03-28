# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-03-28
### Added
- You can now create a editor-test.yaml Github workflow using `uinit setup ci github editor-test`
- Prompts can now be skipped and always respond 'yes'. To do so, use the `--no-prompts` global flag with any command 
### Changed
- Prompts are now handled by the reporter, meaning all prompts use the same design and interaction for better UX

## [0.2.0] - 2026-03-26
### Added
- Custom import paths for remote or imported assets
### Changed
- Alias commands are now changed to Remote, because there are now aliases for dependency bundles as well
- Changed how the commands are structured to make more sense and be easier to type
### Fixed
- Alias adding, removing didn't use the same standard verbose or success messages

## [0.1.1] - 2026-03-22
### Changed
- General code and symantics cleanup. No functional changes.

## [0.1.0] - 2026-03-22
### Added
- Added verbose mode for commands `uinit <command> -v`
### Fixed
- File writing wasn't atomic, now fixed

## [0.0.3] - 2026-03-21
### Added
- Doctor command `uinit doctor` to scan for setup or template inconsistencies and autofix them
- Docs to the README for how to use aliases
- Ability to add or remove custom aliases via `uninit alias add` to your local uinit.toml to override default aliases
### Changed
- Cleaned up a bunch of log messages and user feedback

## [0.0.2] - 2026-03-19
### Added
- You can now import remote assets via an alias configured in your uinit.toml. Default aliases are baked into the tool

## [0.0.1] - 2026-03-15
### Added
- Initial commit
- package-template is now a jinja2 file instead of a json