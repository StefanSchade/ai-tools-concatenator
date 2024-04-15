use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use glob::Pattern;
use crate::config::Config;

pub fn initialize_output_file(path: &Path) -> io::Result<File> {
    File::create(path)
}

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
    file_path: &Path,
    depth: usize,
) -> io::Result<()> {
    let contents = fs::read_to_string(file_path)?;

    // Open the output file as specified in config
    let mut output = fs::File::create(&config.output_file)?;

    // Determine number of digits needed for line numbers
    let lines = contents.lines().count();
    let num_digits = (lines as f64).log10().floor() as usize + 1;

    writeln!(output, "\n// File: {} Depth: {}\n", file_path.strip_prefix(&config.source_dir).unwrap().display(), depth)?;
    for (i, line) in contents.lines().enumerate() {
        if config.line_numbering {
            let line_number = format!("{:0width$}", i + 1, width = num_digits);
            writeln!(output, "{} {}", line_number, line)?;
        } else {
            writeln!(output, "{}", line)?;
        }
    }

    Ok(())
}