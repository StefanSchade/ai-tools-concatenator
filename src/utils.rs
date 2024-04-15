use std::path::{Path, PathBuf};
use glob::Pattern;
use crate::config::Config;

pub fn is_excluded(
    path: &Path,
    config: &Config,  // Now passing the whole config
) -> bool {
    let relative_path = path.strip_prefix(&config.source_dir)
        .unwrap()
        .to_str()
        .unwrap_or("")
        .replace("\\", "/"); // Normalize path separators

    // Exclude if path is the output file path
    if path == config.output_file {
        return true;
    }

    // Check if path is in an always_exclude path
    if config.always_exclude.iter().any(|p| path.starts_with(p)) {
        return true;
    }

    // Check for gitignore patterns
    if config.gitignore_patterns.iter().any(|p| p.matches_path(path)) || config.gitignore_patterns.iter().any(|p| p.matches(&relative_path)) {
        return true;
    }

    // Check for excluded suffixes
    if config.exclude_suffixes.iter().any(|suffix| path.to_str().unwrap_or("").ends_with(suffix)) {
        return true;
    }

    // Check for "node_modules" directory in any part of the path
    if path.ancestors().any(|p| p.file_name().map_or(false, |f| f == "node_modules")) {
        return true;
    }

    false
}