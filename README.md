# Local Doc Assistant

Local Doc Assistant is a desktop application for working with local development documentation and generating code snippets using a local language model backend. The app is built with Rust and Tauri for the desktop runtime and includes tools to integrate with WSL for driver compilation and project workflows.

## Key Features

- Local AI assistant backed by a local Ollama model (recommended: `qwen2.5-coder:7b`).
- Project indexing and document search for fast reference lookup.
- Save generated code to a staging area and copy to driver or library projects.
- Optional WSL integration for compiling and running project files from Windows.
- Packaging scripts and CI workflows for cross-platform builds.

## Requirements

- Rust and Cargo (stable). Install from https://rustup.rs/
- Node.js (18+) and npm or yarn for frontend tooling if modifying the UI.
- Tauri build prerequisites for your platform. See Tauri docs: https://tauri.app/
- Ollama for local model hosting (optional but required for AI features).

Recommended Ollama model: `qwen2.5-coder:7b`.

## Quick Start (Development)

1. Install Rust and Node.
2. From the repository root, run the development server (requires Tauri configured):

```powershell
# If your environment supports Tauri dev directly
cargo tauri dev

# Or build and run the app from the UI workspace if you edit the renderer
cd ui
# install frontend deps (npm or yarn)
# npm install
# npm run dev
```

3. The app will open as a desktop window. Use the Settings tab to configure Ollama, WSL paths, and project directories.

## Building a Release

To build release bundles use the Tauri build flow:

```powershell
cargo tauri build

# Release artifacts are created under:
# target/release/bundle/
```

Windows installers (MSI and NSIS) are produced under `target/release/bundle/msi` and `target/release/bundle/nsis` when packaging is configured.

## Configuration

- Driver and library output paths can be set from the Settings tab in the app.
- Ollama path and preferences are persisted by the app; use the UI to locate or configure the Ollama binary.

## Testing and Validation

- Use the Driver tab to run WSL validation, compile, and run files when WSL is configured.
- Use the Staging tab to review generated files before copying them into your projects.

## CI and Releases

- A GitHub Actions workflow is configured to build macOS and Linux bundles on tag pushes. Windows installers are built locally by the release process unless CI is extended to support them.

## Contributing

Contributions are welcome. Please open issues or pull requests for bug reports, improvements, and feature requests.

## License

The project does not include a license file in this repository. Add a `LICENSE` file to clearly indicate the terms if you plan to publish or distribute the software.
