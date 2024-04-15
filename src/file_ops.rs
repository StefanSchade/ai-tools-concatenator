use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use glob::Pattern;

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
    file_path: &Path,
    output: &mut fs::File,
    base_path: &Path,
    depth: usize,
) -> io::Result<()> {
    // Attempt to read the file as UTF-8, skip if it fails
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            writeln!(output, "\n// File: {} Depth: {}\n", file_path.strip_prefix(base_path).unwrap().display(), depth)?;
            writeln!(output, "{}", contents)?;
        },
        Err(e) if e.kind() == io::ErrorKind::InvalidData => {
            println!("Skipping file with invalid UTF-8 contents: {}", file_path.display());
        },
        Err(e) => return Err(e),
    }

    Ok(())
}