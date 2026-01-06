# Publishing Guide for Inventory Manager

This guide covers how to build, package, and distribute your Inventory Manager application.

## 🎯 Quick Start

### Local Development Build
```bash
cargo build --release
```

Your optimized binary will be at: `target/release/inventory_app` (or `.exe` on Windows)

### Create Installers Locally
```bash
# Install cargo-packager (one time)
cargo install cargo-packager --locked

# Create installer for your current platform
cargo packager --release
```

Installers will be in the `dist/` directory.

## ⚙️ Optimization Settings

Your `Cargo.toml` now includes optimizations that:
- ✅ Reduce binary size by ~60%
- ✅ Strip debug symbols
- ✅ Enable Link-Time Optimization (LTO)
- ✅ Remove panic unwinding code

```toml
[profile.release]
opt-level = "s"     # Optimize for size
lto = true          # Link Time Optimization
strip = true        # Strip symbols
codegen-units = 1   # Better optimization
panic = "abort"     # Smaller binary
```

## 📦 Packaging with cargo-packager

### Configuration
Your `Packager.toml` defines:
- App name, version, and metadata
- Platform-specific settings
- Dependencies for each OS
- Output directory (`dist/`)

### Platform-Specific Builds

**macOS** - Creates `.dmg`:
```bash
cargo packager --release --formats dmg
```

**Windows** - Creates `.msi` or `.exe`:
```bash
cargo packager --release --formats nsis
# or
cargo packager --release --formats wix
```

**Linux** - Creates `.deb` and `.AppImage`:
```bash
cargo packager --release --formats deb,appimage
```

## 🤖 Automated Releases with GitHub Actions

### Release Workflow

The project uses **`.github/workflows/release-packager.yml`** which:
- Uses `cargo-packager` for professional installers
- Creates proper installers (.dmg, .msi/.exe, .deb/.AppImage)
- Builds for macOS, Windows, and Linux in parallel
- Automatically creates GitHub Releases
- Better user experience than simple archives

### How to Trigger a Release

**Option 1: Use the helper script** ⭐ **Recommended**
```bash
./scripts/prepare-release.sh 0.2.0
```

The script will:
- ✅ Update version in `Cargo.toml` and `Packager.toml`
- ✅ Update `Cargo.lock`
- ✅ Build and verify compilation
- ✅ Create git commit and tag
- ✅ Push to GitHub (with confirmation)

**Option 2: Manual process**
```bash
# 1. Update versions
# Edit Cargo.toml and Packager.toml: version = "0.2.0"

# 2. Update Cargo.lock
cargo check

# 3. Commit and tag
git add Cargo.toml Packager.toml Cargo.lock
git commit -m "Release version 0.2.0"
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

### What Happens Next

GitHub Actions will automatically:
1. ✅ Build for macOS, Windows, and Linux
2. ✅ Create professional installers for each platform
3. ✅ Upload to GitHub Releases
4. ✅ Generate detailed release notes

### Monitor Build Progress
Visit: https://github.com/ojutalayomi/inventory_app/actions

## 🔐 Code Signing (Optional but Recommended)

### Why Code Sign?

Without signing, users see warnings:
- **Windows**: "Unknown publisher" warning
- **macOS**: "Cannot verify developer" / app may be blocked

### How to Sign

#### Windows
1. Buy a code signing certificate (~$200/year)
2. Install certificate
3. Add to workflow:
   ```yaml
   - name: Sign Windows binary
     run: signtool sign /f cert.pfx /p ${{ secrets.CERT_PASSWORD }} dist/*.exe
   ```

#### macOS
1. Get Apple Developer ID ($99/year)
2. Add secrets to GitHub:
   - `APPLE_CERTIFICATE`
   - `APPLE_CERTIFICATE_PASSWORD`
   - `APPLE_ID`
   - `APPLE_TEAM_ID`
3. Update `Packager.toml`:
   ```toml
   [macos]
   signing-identity = "Developer ID Application: Your Name (TEAM_ID)"
   ```

#### Linux
No signing required! Users trust package managers.

## 📊 File Sizes (Approximate)

| Platform | Without Optimization | With Optimization | Savings |
|----------|---------------------|-------------------|---------|
| macOS    | ~25 MB              | ~10 MB            | 60%     |
| Windows  | ~20 MB              | ~8 MB             | 60%     |
| Linux    | ~22 MB              | ~9 MB             | 59%     |

## 🚀 Distribution Options

### 1. GitHub Releases (Current Setup)
- Free hosting
- Unlimited downloads
- Automatic with workflow
- **Best for open source**

### 2. Your Own Website
Download from `dist/` folder and host on:
- Static site (Netlify, Vercel)
- Your domain
- CDN

### 3. Package Managers (Future)

**macOS Homebrew:**
```bash
brew tap ojutalayomi/tap
brew install inventory-app
```

**Windows Chocolatey:**
```bash
choco install inventory-app
```

**Linux:**
- Upload `.deb` to APT repository
- Submit to Snap Store or Flathub

## 🛠️ Troubleshooting

### Build Fails on Linux
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y libgtk-3-dev build-essential
```

### Build Fails on macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

### Build Fails on Windows
- Install Visual Studio Build Tools
- Or use MSVC toolchain: `rustup default stable-msvc`

### cargo-packager not found
```bash
cargo install cargo-packager --locked
```

### Binary size still large
- Make sure you're building with `--release`
- Check `Cargo.toml` has the optimization profile
- Use `strip` tool manually: `strip target/release/inventory_app`

## 📝 Versioning Strategy

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes

Example timeline:
- `v0.1.0` - Initial release ✅ (current)
- `v0.2.0` - Add export to Excel feature
- `v0.2.1` - Fix search bug
- `v1.0.0` - Stable API, production ready

## 🎉 Post-Release Checklist

After releasing:
- [ ] Test installers on each platform
- [ ] Update README with download links
- [ ] Announce on social media / Discord / etc
- [ ] Monitor GitHub Issues for bug reports
- [ ] Update documentation if needed

## 📚 Additional Resources

- [cargo-packager docs](https://docs.rs/cargo-packager)
- [Iced GUI framework](https://iced.rs)
- [Rust deployment guide](https://rust-lang.github.io/rustup/installation/index.html)
- [GitHub Actions docs](https://docs.github.com/en/actions)

## 💡 Tips

1. **Test installers before release**: Build locally with `cargo packager` and test on a fresh VM
2. **Keep changelog**: Users love knowing what changed
3. **Automate everything**: Let GitHub Actions do the work
4. **Listen to users**: Monitor Issues for feedback
5. **Update regularly**: Security patches, dependencies, features

---

**Need help?** Open an issue or check the documentation above!

