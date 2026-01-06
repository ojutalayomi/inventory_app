# Release Workflow Update

## ✅ Changes Made

### 1. Updated Release Script
**File:** `scripts/prepare-release.sh`

The script now updates **both** version files:
- ✅ `Cargo.toml` - Rust package version
- ✅ `Packager.toml` - Installer version

When you run:
```bash
./scripts/prepare-release.sh 0.2.0
```

It will:
1. Update version in both `Cargo.toml` and `Packager.toml`
2. Update `Cargo.lock`
3. Build and verify the release
4. Commit all three files
5. Create and push the git tag

### 2. Removed Old Workflow
**Deleted:** `.github/workflows/release.yml`

The old workflow that created simple archives (`.zip`, `.tar.gz`) has been removed.

### 3. Updated Documentation
**File:** `PUBLISHING_GUIDE.md`

Updated to reflect that we now have a single, modern workflow that creates proper installers.

## 🚀 Current Release Process

### Single Modern Workflow
**File:** `.github/workflows/release-packager.yml`

This workflow creates **professional installers** for all platforms:

| Platform | Installer Types | User Experience |
|----------|----------------|-----------------|
| **macOS** | `.dmg` | Drag & drop to Applications |
| **Windows** | `.exe` / `.msi` | Click-through installer |
| **Linux** | `.deb` + `.AppImage` | Package manager or direct run |

### How to Create a Release

**Simple:** Just run the script!
```bash
./scripts/prepare-release.sh 0.2.0
```

The script handles everything:
- ✅ Updates versions in all files
- ✅ Commits changes
- ✅ Creates git tag
- ✅ Pushes to GitHub
- ✅ Triggers automated builds

### What Happens on GitHub

1. **Parallel Builds** - All three platforms build simultaneously
2. **cargo-packager** - Creates proper installers (not just archives)
3. **GitHub Release** - Automatically creates release with all files
4. **Release Notes** - Auto-generated with download instructions

## 📊 Comparison

### Before (Old Workflow)
```
v0.1.0/
├── inventory_app-macos.dmg      (simple archive)
├── inventory_app-windows.zip    (requires manual extract)
└── inventory_app-linux.tar.gz   (requires manual extract)
```

### After (New Workflow)
```
v0.1.0/
├── inventory_app_0.1.0_macos.dmg           (proper DMG)
├── inventory_app_0.1.0_windows_x64.msi     (Windows installer)
├── inventory_app_0.1.0_amd64.deb           (Debian package)
└── inventory_app_0.1.0_amd64.AppImage      (Universal Linux)
```

## ✨ Benefits

1. **Professional Installers** - Users get proper installers, not archives
2. **Better UX** - Click-through wizards instead of manual extraction
3. **Single Workflow** - Simpler to maintain
4. **Automated Versioning** - Script updates all files at once
5. **Cross-platform** - Builds on native runners for better compatibility

## 🔄 Migration Complete

You're now using the modern release workflow! No action needed - everything is configured and ready to go.

Next release:
```bash
./scripts/prepare-release.sh 0.2.0
```

That's it! 🎉

