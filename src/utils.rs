use std::fs;
use std::path::Path;

use crate::config::Config;

pub fn is_excluded(
    path: &Path,
    config: &Config,  // Now passing the whole config
) -> bool {

    let path_str = path.to_str().unwrap_or("");

    let relative_path = path.strip_prefix(&config.source_dir)
        .unwrap()
        .to_str()
        .unwrap_or("")
        .replace("\\", "/"); // Normalize path separators

    // Resolve both paths to their absolute canonical forms to avoid mismatches
    let canonical_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let canonical_output = fs::canonicalize(&config.output_file).unwrap_or_else(|_| config.output_file.clone());

    // Check if the path is the output file path
    if canonical_path == canonical_output {
        println!("Excluding the output file");
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
    if config.exclude_set.is_match(path_str) {
        return true;
    }

    // Check for "node_modules" directory in any part of the path
    if path.ancestors().any(|p| p.file_name().map_or(false, |f| f == "node_modules")) {
        return true;
    }

    false
}