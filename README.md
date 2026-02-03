# Local Doc Assistant

A cross-platform desktop application that turns any codebase into an AI-powered documentation assistant. Index your project, choose your LLM, and get instant answers about your code.

## Features

- **Universal Indexing**: Point it at any directory and file types
- **Local LLM**: Uses Ollama - your data never leaves your machine
- **Fast Search**: Built-in full-text search with relevance scoring
- **Beautiful UI**: Modern desktop app with syntax highlighting
- **Configurable**: Choose your own model and file types

## Quick Start

1. **Install Ollama**: [ollama.ai](https://ollama.ai)
2. **Pull a model**: `ollama pull qwen2.5-coder:7b`
3. **Download** the latest release for your platform
4. **Run** the installer and follow the setup wizard

## Supported Languages

Works with any text-based files! Common presets include:
- Rust, Python, JavaScript/TypeScript, C/C++, Java, Go
- Markdown, HTML, CSS, YAML, JSON
- And any custom file extensions you specify

## How It Works

1. **Setup Wizard**: On first launch, configure your project settings
   - Name your project
   - Select the directory to index
   - Choose file types (or use presets)
   - Pick your Ollama model
   
2. **Indexing**: The app builds a searchable index of your code
   - Fast full-text search
   - Relevance scoring
   - Smart context extraction

3. **Query**: Ask natural language questions
   - Powered by your chosen Ollama model
   - Context-aware responses
   - Code examples included

## Building from Source

### Prerequisites
- Rust (latest stable)
- Node.js (for Tauri)
- Ollama (for LLM support)

### Build Steps
```bash
git clone https://github.com/YOUR_USERNAME/local-doc-assistant.git
cd local-doc-assistant
cargo tauri build
```

The compiled binary will be in `src-tauri/target/release/`

## Configuration

Configuration is stored in:
- **Windows**: `%APPDATA%\local-doc-assistant\config.json`
- **macOS**: `~/Library/Application Support/local-doc-assistant/config.json`
- **Linux**: `~/.config/local-doc-assistant/config.json`

You can reset configuration by deleting this file and restarting the app.

## Recommended Models

For best results with code documentation:
- **Small/Fast**: `qwen2.5-coder:3b` (good for quick queries)
- **Balanced**: `qwen2.5-coder:7b` (recommended default)
- **Large/Detailed**: `qwen2.5-coder:14b` (most comprehensive)

Pull models with: `ollama pull <model-name>`

## Use Cases

- **Code Exploration**: Understand unfamiliar codebases
- **API Documentation**: Generate explanations for your APIs
- **Legacy Code**: Document undocumented projects
- **Learning**: Study open-source projects with AI assistance
- **Custom Knowledge Base**: Index any text-based documentation

## Architecture

- **Frontend**: HTML/CSS/JavaScript with Tauri
- **Backend**: Rust with async/await
- **Indexing**: Tantivy full-text search
- **LLM**: Ollama (local inference)

## Privacy

- All data stays on your machine
- No cloud dependencies
- No telemetry or tracking
- Offline-capable (except for Ollama downloads)

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Troubleshooting

### Ollama Connection Issues
- Ensure Ollama is running: `ollama serve`
- Verify models are available: `ollama list`
- Check port 11434 is not blocked

### Indexing Problems
- Ensure selected directory is readable
- Check file permissions
- Try smaller directory first

### Build Issues
- Update Rust: `rustup update`
- Clear cargo cache: `cargo clean`
- Check Tauri prerequisites: [Tauri Setup](https://tauri.app/v1/guides/getting-started/prerequisites)

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

- Built with [Tauri](https://tauri.app)
- Powered by [Ollama](https://ollama.ai)
- Search by [Tantivy](https://github.com/quickwit-oss/tantivy)

## Roadmap

- [ ] Vector database support for large codebases
- [ ] Multiple project management
- [ ] Export conversations
- [ ] Custom prompt templates
- [ ] Collaborative annotations

---

**Version**: 2.0.0  
**Status**: Stable  
**Platform**: Windows, macOS, Linux
# local-doc-assistant

## Screenshots

Gallery of the latest UI screenshots (captured 2026-02-03):

![Main window 1](docs/screenshots/Screenshot%202026-02-03%20132328.png)

![Main window 2](docs/screenshots/Screenshot%202026-02-03%20132339.png)

![Setup wizard](docs/screenshots/Screenshot%202026-02-03%20132346.png)

![Ollama setup](docs/screenshots/Screenshot%202026-02-03%20132351.png)
