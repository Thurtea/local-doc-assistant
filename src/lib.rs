#![allow(dead_code, unused_imports, deprecated, clippy::all)]

pub mod ollama_client;
pub mod context_manager;
pub mod prompt_builder;
pub mod document_index;  // CHANGED from mud_index
pub mod user_config;     // NEW
pub mod driver_analyzer;
pub mod rag_validator;
pub mod model_tester;

// Re-export selected types for convenience
pub use ollama_client::{OllamaClient, OllamaOptions};
pub use context_manager::ContextManager;
pub use prompt_builder::PromptBuilder;
pub use document_index::DocumentIndex;  // CHANGED
pub use user_config::{UserProjectConfig, load_user_config, save_user_config};  // NEW
pub use rag_validator::RAGValidator;
pub use model_tester::ModelTester;
