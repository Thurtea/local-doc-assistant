use anyhow::{Context as AnyhowContext, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: PathBuf,
    pub line_number: usize,
    pub snippet: String,
    pub relevance_score: f32,
    pub file_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub path: PathBuf,
    pub snippet: String,
}

#[derive(Clone)]
pub struct DocumentIndex {
    corpus_root: PathBuf,
    supported_extensions: Vec<String>,
    term_index: HashMap<String, Vec<(PathBuf, usize)>>,
    indexed: bool,
}

impl DocumentIndex {
    pub fn new(corpus_root: PathBuf, extensions: Vec<String>) -> Self {
        Self {
            corpus_root,
            supported_extensions: extensions,
            term_index: HashMap::new(),
            indexed: false,
        }
    }

    pub fn open_or_build(_index_dir: &Path, corpus_root: &Path, extensions: Vec<String>) -> Result<Self> {
        std::fs::create_dir_all(_index_dir)
            .with_context(|| format!("Failed to create index dir {:?}", _index_dir))?;
        Ok(Self::new(corpus_root.to_path_buf(), extensions))
    }

    pub fn build_index(&mut self) -> Result<usize> {
        if self.indexed {
            return Ok(self.term_index.len());
        }

        let mut indexed_files = 0;

        for entry in walkdir::WalkDir::new(&self.corpus_root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if !self.is_indexable_file(path) {
                continue;
            }

            match std::fs::read_to_string(path) {
                Ok(content) => {
                    Self::index_file(&mut self.term_index, path, &content);
                    indexed_files += 1;
                }
                Err(_) => continue,
            }
        }

        self.indexed = true;
        Ok(indexed_files)
    }

    fn index_file(term_index: &mut HashMap<String, Vec<(PathBuf, usize)>>, path: &Path, content: &str) {
        for (line_idx, line) in content.lines().enumerate() {
            let terms = Self::extract_terms(line);
            for term in terms {
                term_index
                    .entry(term)
                    .or_default()
                    .push((path.to_path_buf(), line_idx));
            }
        }
    }

    fn extract_terms(line: &str) -> HashSet<String> {
        let mut terms = HashSet::new();
        let words: Vec<&str> = line
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|w| !w.is_empty() && w.len() > 2)
            .collect();

        for word in words {
            terms.insert(word.to_lowercase());
        }
        terms
    }

    pub fn search_relevant_code(&self, query: &str, limit: usize) -> Result<Vec<CodeSnippet>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for entry in walkdir::WalkDir::new(&self.corpus_root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if !self.is_indexable_file(path) {
                continue;
            }

            match std::fs::read_to_string(path) {
                Ok(content) => {
                    if content.to_lowercase().contains(&query_lower) {
                        let snippet = Self::extract_snippet(&content, &query_lower, 1024);
                        results.push(CodeSnippet {
                            path: path.to_path_buf(),
                            snippet,
                        });
                        if results.len() >= limit {
                            break;
                        }
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    pub fn search_with_scoring(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let _query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query
            .split_whitespace()
            .filter(|w| w.len() > 2)
            .collect();

        let mut candidates: Vec<(PathBuf, usize, f32)> = Vec::new();

        for entry in walkdir::WalkDir::new(&self.corpus_root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if !self.is_indexable_file(path) {
                continue;
            }

            match std::fs::read_to_string(path) {
                Ok(content) => {
                    for (line_idx, line) in content.lines().enumerate() {
                        let score = Self::calculate_relevance(&query_terms, line);
                        if score > 0.0 {
                            candidates.push((path.to_path_buf(), line_idx, score));
                        }
                    }
                }
                Err(_) => continue,
            }
        }

        candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

        let mut results = Vec::new();
        for (path, line_idx, score) in candidates.into_iter().take(limit) {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let lines: Vec<&str> = content.lines().collect();
                if line_idx < lines.len() {
                    let context_start = line_idx.saturating_sub(6);
                    let context_end = std::cmp::min(line_idx + 7, lines.len());
                    let snippet = lines[context_start..context_end].join("\n");

                    results.push(SearchResult {
                        path: path.clone(),
                        line_number: line_idx,
                        snippet,
                        relevance_score: score,
                        file_type: path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                    });
                }
            }
        }

        Ok(results)
    }

    fn calculate_relevance(query_terms: &[&str], line: &str) -> f32 {
        let line_lower = line.to_lowercase();
        let mut matches = 0;

        for term in query_terms {
            if line_lower.contains(term) {
                matches += 1;
            }
        }

        if matches == 0 {
            return 0.0;
        }

        (matches as f32) / (query_terms.len() as f32)
    }

    pub fn refresh(&self) -> Result<()> {
        Ok(())
    }

    fn is_indexable_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|s| s.to_str())
            .map(|ext| self.supported_extensions.contains(&ext.to_string()))
            .unwrap_or(false)
    }

    fn extract_snippet(content: &str, q: &str, max_len: usize) -> String {
        let lc = content.to_lowercase();
        if let Some(idx) = lc.find(q) {
            let start = idx.saturating_sub(max_len / 4);
            let end = std::cmp::min(start + max_len, content.len());
            let mut s = content[start..end].to_string();
            if end < content.len() {
                s.push_str("\n...");
            }
            return s;
        }
        if content.len() <= max_len {
            content.to_string()
        } else {
            let mut s = content[..max_len].to_string();
            s.push_str("\n...");
            s
        }
    }
    
    pub fn update_extensions(&mut self, extensions: Vec<String>) {
        self.supported_extensions = extensions;
        self.indexed = false;
    }
}
