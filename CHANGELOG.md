# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Auto-resize bottom info and buddy image.

### Fixed
- Content truncation of learning interface when displaying long paragraphs.
- Bottom info layout overflow.

## [0.1.0] - 2026-06-21

### Added
- TUI (Terminal User Interface): Full-screen interactive terminal interface with real-time rendering.
- Learn data reader: Reads and parses learning data from `.yaml`(recommend), `.json`, `.toml`, and `.ron`, allowing you to author learning content in your preferred format.
- Inline command system: Controls the TUI using vim-like keybindings.
- Simple configuration system: Customizable TUI behavior(e.g. user_name) and appearance(e.g. themes) to match your preferences.
- Cute buddy on right-bottom: A cute ASCII art character customized via the user_name appears on the right-bottom corner.
- Simple Development tool: Auto build release packages.

# Thanks

I appreciate freedom software lover and every contributor who has helped improve this project, whether through dependencies or tools.

[Unreleased]: https://codeberg.org/hello_wxs/yezi/compare/v0.1.0...dev
[0.1.0]: https://codeberg.org/hello_wxs/yezi/releases/tag/v0.1.0
[@hello_wxs]: https://codeberg.org/hello_wxs
