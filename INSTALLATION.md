# Installation

This document explains how to install the Local Doc Assistant on Windows, Linux and macOS.

Windows
--
- Download the MSI installer from the GitHub release page and run the installer.
- The NSIS `.exe` installer is also provided for custom installs.

Linux
--
- Recommended: download the AppImage (or deb/rpm) artifact from the GitHub release for your distribution.
- Make the AppImage executable and run it:

```bash
chmod +x Local\ Doc\ Assistant-*.AppImage
./Local\ Doc\ Assistant-*.AppImage
```

- Alternatively build locally in WSL or on a Linux host:

```bash
# Install dependencies (rust, node, tauri prerequisites)
cargo build --release
npx tauri build
```

macOS
--
- Download the `.dmg` or `.pkg` from the GitHub release and open it; drag the app to `Applications` (for a dmg) or run the installer for `.pkg`.
- Building macOS bundles requires a macOS runner or local Mac with Xcode and Tauri prerequisites:

```bash
# On macOS
cargo build --release
npx tauri build
```

Building on CI
--
- This repository includes a GitHub Actions workflow `.github/workflows/build-release-bundles.yml` that will build Linux and macOS bundles when a tag matching `v*` is pushed. The workflow will attach produced artifacts to the corresponding GitHub release.

Troubleshooting
--
- If an installer doesn't launch, check system logs and requirements for Tauri on your platform.
- For macOS Gatekeeper issues, right-click the app and choose "Open", or remove quarantine with `xattr -r -d com.apple.quarantine /path/to/app`.
