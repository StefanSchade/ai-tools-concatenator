use std::{env, fs, io, path::Path};
use glob::Pattern;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <source_dir> <output_file> [exclude_suffixes...]", args[0]);
        std::process::exit(1);
    }

    let source_dir = &args[1];
    let output_file = &args[2];
    let exclude_suffixes = &args[3..];

    let mut output = fs::File::create(output_file)?;

    let gitignore_patterns = parse_gitignore(&source_dir);

    concatenate_dir(Path::new(source_dir), &mut output, 0, &gitignore_patterns, exclude_suffixes)?;

    Ok(())
}

fn parse_gitignore(source_dir: &str) -> Vec<Pattern> {
    let gitignore_path = Path::new(source_dir).join(".gitignore");
    let mut patterns = Vec::new();
    if let Ok(content) = fs::read_to_string(gitignore_path) {
        for line in content.lines() {
            if let Ok(pattern) = Pattern::new(line) {
                patterns.push(pattern);
            }
        }
    }
    patterns
}

fn concatenate_dir(
    path: &Path,
    output: &mut fs::File,
    depth: usize,
    gitignore_patterns: &[Pattern],
    exclude_suffixes: &[String],
) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_str().unwrap_or("");

            // Check against .gitignore patterns
            if gitignore_patterns.iter().any(|p| p.matches(path_str)) {
                continue;
            }

            // Exclude by suffix
            if exclude_suffixes.iter().any(|suffix| path_str.ends_with(suffix)) {
                continue;
            }

            if path.is_dir() {
                concatenate_dir(&path, output, depth + 1, gitignore_patterns, exclude_suffixes)?;
            } else {
                writeln!(output, "\n// File: {:?} Depth: {}\n", path.display(), depth)?;
                let mut file = fs::File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                writeln!(output, "{}", contents)?;
            }
        }
    }
    Ok(())
}
