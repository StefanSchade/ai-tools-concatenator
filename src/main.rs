use std::env;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};
use glob::Pattern;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <source_dir> <output_file> [exclude_suffixes...]", args[0]);
        std::process::exit(1);
    }

    let source_dir = &args[1];
    let output_file = &args[2];
    let exclude_suffixes = &args[3..];

    let mut output = File::create(output_file)?;

    let gitignore_patterns = read_gitignore_patterns(Path::new(source_dir))?;

    concatenate_dir(Path::new(source_dir), &mut output, 0, &gitignore_patterns, exclude_suffixes)?;

    Ok(())
}

fn read_gitignore_patterns(source_dir: &Path) -> io::Result<Vec<Pattern>> {
    let gitignore_path = source_dir.join(".gitignore");
    let mut patterns = Vec::new();
    if gitignore_path.exists() {
        let content = fs::read_to_string(gitignore_path)?;
        for line in content.lines() {
            if !line.trim().is_empty() && !line.starts_with('#') {
                patterns.push(Pattern::new(line.trim()).expect("Failed to parse .gitignore pattern"));
            }
        }
    }
    Ok(patterns)
}

fn concatenate_dir(path: &Path, output: &mut File, depth: usize, gitignore_patterns: &[Pattern], exclude_suffixes: &[String]) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                concatenate_dir(&path, output, depth + 1, gitignore_patterns, exclude_suffixes)?;
            } else if should_include_file(&path, gitignore_patterns, exclude_suffixes) {
                writeln!(output, "\n// File: {:?} Depth: {}\n", path.display(), depth)?;
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                writeln!(output, "{}", contents)?;
            }
        }
    }
    Ok(())
}

fn should_include_file(path: &Path, gitignore_patterns: &[Pattern], exclude_suffixes: &[String]) -> bool {
    if let Some(file_name) = path.to_str() {
        for pattern in gitignore_patterns {
            if pattern.matches(file_name) {
                return false;
            }
        }

        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            if exclude_suffixes.iter().any(|suffix| extension.ends_with(suffix)) {
                return false;
            }
        }
    }
    true
}
