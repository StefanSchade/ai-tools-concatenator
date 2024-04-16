use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use glob::Pattern;
use crate::config::Config;

pub fn read_gitignore(source_dir: &Path) -> io::Result<Vec<Pattern>> {
    let gitignore_path = source_dir.join(".gitignore");
    let mut patterns = Vec::new();
    if let Ok(content) = fs::read_to_string(&gitignore_path) {
        for line in content.lines() {
            if let Ok(pattern) = Pattern::new(line) {
                patterns.push(pattern);
            }
        }
    }
    Ok(patterns)
}

pub fn read_and_write_file(
    config: &Config,
    file_handle: &mut File,
    file_path: &Path,
    depth: usize,
) -> io::Result<()> {
    let contents = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_e) => {
            eprintln!("Skipping file with non-UTF-8 contents: {}", file_path.display());
            return Ok(());  // Return Ok to continue processing other files
        }
    };

    let lines = contents.lines().count();
    let num_digits = (lines as f64).log10().floor() as usize + 1;

    writeln!(file_handle, "\n// File: {} Depth: {}\n", file_path.strip_prefix(&config.source_dir).unwrap().display(), depth)?;
    for (i, line) in contents.lines().enumerate() {
        if config.line_numbering {
            let line_number = format!("{:0width$}", i + 1, width = num_digits);
            writeln!(file_handle, "{} {}", line_number, line)?;
        } else {
            writeln!(file_handle, "{}", line)?;
        }
    }

    Ok(())
}