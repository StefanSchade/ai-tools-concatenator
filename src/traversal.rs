use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self};

use glob::Pattern;

use crate::file_ops;
use crate::utils;

pub fn concatenate_dir(
    base_path: &Path,
    current_path: &Path,
    output: &mut fs::File,
    depth: usize,
    gitignore_patterns: &[Pattern],
    exclude_suffixes: &[String],
    always_exclude: &[PathBuf],
) -> io::Result<()> {
    if current_path.is_dir() {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();

            if utils::is_excluded(&path, base_path, gitignore_patterns, always_exclude, exclude_suffixes) {
                continue;
            }

            if path.is_dir() {
                concatenate_dir(base_path, &path, output, depth + 1, gitignore_patterns, exclude_suffixes, always_exclude)?;
            } else {
                file_ops::read_and_write_file(&path, output, base_path, depth)?;
            }
        }
    }
    Ok(())
}
