use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use glob::Pattern;
use crate::file_ops;

pub struct Config {
    pub source_dir: PathBuf,
    pub output_file: PathBuf,
    pub exclude_set: GlobSet,
    pub line_numbering: bool,
    pub gitignore_patterns: Vec<Pattern>,
    pub always_exclude: Vec<PathBuf>,
}

impl Config {
    // Method to initialize Config with command-line arguments
    pub fn new(args: &[String]) -> io::Result<Config> {
        if args.len() < 3 {
            let usage = format!(
                "Usage: {} <source_dir> <output_file> [exclude_patterns...] [--line-numbers]\n\
                \n\
                <source_dir>      : The directory from which files will be read.\n\
                <output_file>     : The file to which concatenated output will be written.\n\
                [exclude_patterns]: Optional. Files containing these patterns will be excluded from concatenation.\n\
                --line-numbers    : Optional. Prepend each line with its line number.",
                args.get(0).unwrap_or(&"program".to_string())
            );
            return Err(Error::new(ErrorKind::InvalidInput, usage));
        }

        let source_dir = PathBuf::from(&args[1]);

        let gitignore_patterns = file_ops::read_gitignore(&source_dir)?;
        let always_exclude = vec![
            source_dir.join(".git"),
            source_dir.join("target"),
            source_dir.join(".idea"),
        ];

        let mut builder = GlobSetBuilder::new();
        for pattern in args.iter().skip(3) {
            if pattern.starts_with("--") {
                continue;  // Skip options like --line-numbers
            }
            let glob = GlobBuilder::new(pattern).build().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            builder.add(glob);
        }
        let exclude_set  = builder.build().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;



        Ok(Config {
            source_dir: source_dir.clone(),
            output_file: PathBuf::from(&args[2]),
            exclude_set,
            line_numbering: args.contains(&"--line-numbers".to_string()),
            gitignore_patterns,
            always_exclude,
        })
    }
}