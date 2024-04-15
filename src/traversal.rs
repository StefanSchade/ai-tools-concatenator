use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

use glob::Pattern;

use crate::config::Config;
use crate::file_ops::read_and_write_file;
use crate::utils;

pub fn concatenate_dir(
    config: &Config,
    current_path: &Path,
    depth: usize,
) -> io::Result<()> {
    if current_path.is_dir() {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();

            if utils::is_excluded(&path, &config.source_dir, &config.gitignore_patterns, &config.always_exclude, &config.exclude_suffixes) {
                continue;
            }

            if path.is_dir() {
                concatenate_dir(config, &path, depth + 1)?;
            } else {
                read_and_write_file(config, &path, depth)?;
            }
        }
    }
    Ok(())
}