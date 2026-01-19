# Changelog
All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to
Semantic Versioning.

## [Unreleased]
## [0.1.0] - 2026-01-06
- Initial release with inventory tracking, notes, alerts, search, and audit log.
- Built-in update notifications for new versions.

## [0.1.1] - 2026-01-07
- Added dedicated macOS downloads for Intel and Apple Silicon.

## [0.2.0] - 2026-01-14
- UI was redesigned to be more modern and user-friendly.
- The app now has a dark and light theme.
- The app now has a new update checker that checks for updates and notifies the user if there is a new version available.

## [0.2.1] - 2026-01-15
- More reliable update checks and notifications.
- Fixed issue with icon display across different platforms.
- Moved calculator into a separate window.

## [0.2.2] - 2026-01-16
- Improved update notifications and in-app messaging.
- Stability and performance improvements.
- Fixed issue with update checker not working correctly.
- Fixed issue with update checker not showing release notes correctly.
- Import functionality now merges data with existing inventory.
- Added support for MacOS, Linux and Windows notifications.

## [0.2.3] - 2026-01-19
- Added a Windows subsystem flag to prevent the console window from appearing in release builds.
- Enhanced the calculator window with comprehensive keyboard input support:
    - Users can now type numbers (0-9) and decimal points directly using their keyboard, eliminating the need to click the calculator buttons.
    - Pressing `Shift` + `=` inputs the `+` (addition) operator, and `Shift` + `8` inputs the multiplication (`×`) operator.
    - Supports basic operations via keyboard: `+`, `-`, `*`, `/`, and `=`.
    - Pressing `Enter` evaluates the current input and displays the result.
    - Pressing `Esc` or `c`/`C` clears the calculator input.
- Added preferred currency selection for price display.
- Added inventory view toggle (cards/table) with CSV export.
- Added note export to TXT or Markdown.

