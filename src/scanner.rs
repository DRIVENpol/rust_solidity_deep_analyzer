use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileScanner {
    root_path: PathBuf,
    exclusions: Vec<String>,
}

impl FileScanner {
    pub fn new(path: PathBuf) -> Self {
        let exclusions = Self::load_exclusions();
        Self { root_path: path, exclusions }
    }

    fn load_exclusions() -> Vec<String> {
        // Look for .analyzerignore in the current working directory (project root)
        let ignore_file = PathBuf::from(".analyzerignore");

        if !ignore_file.exists() {
            return Vec::new();
        }

        match fs::read_to_string(&ignore_file) {
            Ok(content) => {
                content
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty() && !line.starts_with('#'))
                    .map(|line| line.to_string())
                    .collect()
            }
            Err(_) => Vec::new(),
        }
    }

    fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in &self.exclusions {
            if Self::matches_pattern(&path_str, pattern) {
                return true;
            }
        }

        false
    }

    fn matches_pattern(path: &str, pattern: &str) -> bool {
        // Handle wildcards
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();

            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];

                // Check if pattern is like "lib/*" (directory prefix)
                if suffix.is_empty() {
                    return path.contains(prefix);
                }

                // Check if pattern is like "*Test.sol" (suffix match)
                if prefix.is_empty() {
                    return path.ends_with(suffix);
                }

                // Check if pattern is like "Test*.sol" (prefix and suffix)
                return path.contains(prefix) && path.ends_with(suffix);
            }
        }

        // Exact match or contains match
        path.contains(pattern) || path.ends_with(pattern)
    }

    pub fn scan_contracts(&self) -> Result<Vec<PathBuf>> {
        let mut sol_files = Vec::new();

        for entry in WalkDir::new(&self.root_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "sol") {
                // Check if this file should be excluded
                if !self.should_exclude(path) {
                    sol_files.push(path.to_path_buf());
                }
            }
        }

        Ok(sol_files)
    }
}
