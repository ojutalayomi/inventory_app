# Auto-Update System Documentation

## Overview

The Inventory Manager now includes an automatic update system that checks for new releases on GitHub and notifies users when updates are available.

## Features

- **Automatic Update Check**: Checks for updates on login
- **Manual Check**: "Check for Updates" button in Settings
- **Update Notification**: Non-intrusive overlay when updates are available
- **One-Click Update**: Downloads and opens the installer automatically
- **Cross-Platform**: Supports macOS, Windows, and Linux
- **GitHub Actions CI/CD**: Automated builds for all platforms

## How It Works

### For Users

1. **On Login**: The app automatically checks GitHub for new releases
2. **In Settings**: Click "Check for Updates" to manually check
3. **When Update Available**: 
   - A notification overlay appears with release notes
   - Click "Update Now" to download the installer
   - Click "Later" to dismiss and check again later
4. **After Download**: The installer opens automatically

### For Developers

#### Creating a Release

Use the provided release script:

```bash
./scripts/prepare-release.sh 0.2.0
```

This script will:
1. Validate the version format
2. Update `Cargo.toml` and `Cargo.lock`
3. Build and test the project
4. Create a git commit and tag
5. Push to GitHub (triggers CI/CD)

#### Manual Release Process

If you prefer to do it manually:

```bash
# Update version in Cargo.toml
# version = "0.2.0"

# Update Cargo.lock
cargo check

# Commit changes
git add Cargo.toml Cargo.lock
git commit -m "Release version 0.2.0"

# Create and push tag
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin main
git push origin v0.2.0
```

#### GitHub Actions Workflow

When you push a version tag (e.g., `v0.2.0`), GitHub Actions automatically:

1. **Builds for macOS**:
   - Compiles release binary
   - Creates `.dmg` installer
   - Uploads to release

2. **Builds for Windows**:
   - Compiles release binary
   - Creates `.zip` archive
   - Uploads to release

3. **Builds for Linux**:
   - Compiles release binary
   - Creates `.tar.gz` archive
   - Uploads to release

4. **Creates GitHub Release**:
   - Attaches all platform binaries
   - Includes auto-generated changelog
   - Makes release public

## Architecture

### Components

1. **UpdateChecker** (`src/update_checker.rs`):
   - Queries GitHub Releases API
   - Compares versions using semver
   - Downloads platform-specific installers
   - Opens downloaded files

2. **Update Messages** (`src/messages.rs`):
   - `CheckForUpdates`: Trigger update check
   - `UpdateCheckComplete`: Handle check result
   - `DownloadUpdate`: Download installer
   - `InstallUpdate`: Open installer
   - `CloseUpdateNotification`: Dismiss notification

3. **Settings View** (`src/views/settings.rs`):
   - Displays current version
   - "Check for Updates" button
   - Shows update availability badge

4. **Update Notification** (`src/main.rs::view_update_notification`):
   - Overlay modal with release info
   - "Update Now" and "Later" buttons
   - Scrollable release notes

### Update Flow

```
Login
  ↓
Check GitHub API
  ↓
Compare Versions (semver)
  ↓
[If Update Available]
  ↓
Show Notification
  ↓
User Clicks "Update Now"
  ↓
Download Installer
  ↓
Open Installer
  ↓
User Completes Installation
```

## Configuration

### Repository Settings

The update checker is configured in `src/main.rs`:

```rust
update_checker: update_checker::UpdateChecker::new(
    "ojutalayomi".to_string(),
    "inventory_app".to_string(),
),
```

To use with a different repository:
- Change `ojutalayomi` to your GitHub username
- Change `inventory_app` to your repository name

### Version Format

The app uses **Semantic Versioning** (MAJOR.MINOR.PATCH):
- `0.1.0` - Initial release
- `0.2.0` - Minor update with new features
- `1.0.0` - Major release
- `1.0.1` - Patch/bugfix

## GitHub API Rate Limiting

- **Unauthenticated requests**: 60 per hour per IP
- This is sufficient for typical update checking
- Future enhancement: Add GitHub token for higher limits

## Platform-Specific Notes

### macOS
- Downloads `.dmg` file
- Opens with Finder
- User drags to Applications folder

### Windows
- Downloads `.zip` archive
- Opens with default unzip program
- User extracts and runs `.exe`

### Linux
- Downloads `.tar.gz` archive
- Opens with file manager
- User extracts and runs binary

## Troubleshooting

### Update Check Fails
- Check internet connection
- Verify GitHub repository exists and is public
- Check GitHub API rate limit

### Download Fails
- Ensure sufficient disk space
- Check Downloads folder permissions
- Verify internet connection

### Installer Doesn't Open
- Check file associations for `.dmg`/`.zip`/`.tar.gz`
- Manually navigate to Downloads folder
- Check system security settings

## Future Enhancements

- [ ] Add progress bar for downloads
- [ ] Auto-install updates (with user permission)
- [ ] Changelog viewer with markdown rendering
- [ ] Update scheduling (check daily/weekly)
- [ ] Beta release channel option
- [ ] Rollback to previous version
- [ ] Delta updates (only changed files)

## Security Considerations

- All downloads use HTTPS from GitHub
- GitHub Releases are signed by repository
- Users manually approve installation
- No auto-execution of downloaded files

## Testing

To test the update system:

1. Create a test release on GitHub:
   ```bash
   ./scripts/prepare-release.sh 0.2.0
   ```

2. Wait for CI/CD to complete (~5-10 minutes)

3. Build app with version 0.1.0
4. Run app and login
5. Check for update notification
6. Test "Update Now" functionality

## Dependencies

- `reqwest` - HTTP client for GitHub API
- `semver` - Version comparison
- `open` - Cross-platform file opener
- `serde_json` - JSON parsing

## License

Same as the main project.

