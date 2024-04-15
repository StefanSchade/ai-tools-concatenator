use std::path::{Path, PathBuf};
use glob::Pattern;

pub fn is_excluded(
    path: &Path,
    base_path: &Path,
    gitignore_patterns: &[Pattern],
    always_exclude: &[PathBuf],
    exclude_suffixes: &[String],
) -> bool {
    let relative_path = path.strip_prefix(base_path)
        .unwrap()
        .to_str()
        .unwrap_or("")
        .replace("\\", "/"); // Manually normalize path separators

    always_exclude.iter().any(|p| path.starts_with(p)) ||
        gitignore_patterns.iter().any(|p| p.matches_path(path)) ||
        gitignore_patterns.iter().any(|p| p.matches(&relative_path)) ||
        exclude_suffixes.iter().any(|suffix| path.to_str().unwrap_or("").ends_with(suffix))
}
