use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use glob::Pattern;
use crate::file_ops;

pub struct Config {
    pub source_dir: PathBuf,
    pub output_file: PathBuf,
    pub exclude_suffixes: Vec<String>,
    pub line_numbering: bool,
    pub gitignore_patterns: Vec<Pattern>,
    pub always_exclude: Vec<PathBuf>,
}
impl Config {
    pub fn new(args: &[String]) -> io::Result<Config> {
        if args.len() < 3 {
            let usage = format!(
                "Usage: {} <source_dir> <output_file> [exclude_suffixes...] [--line-numbers]\n\
                \n\
                <source_dir>      : The directory from which files will be read.\n\
                <output_file>     : The file to which concatenated output will be written.\n\
                [exclude_suffixes]: Optional. Files ending with these suffixes will be excluded from concatenation.\n\
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

        Ok(Config {
            source_dir: source_dir.clone(),
            output_file: PathBuf::from(&args[2]),
            exclude_suffixes: args[3..].iter().filter(|&arg| arg != "--line-numbers").cloned().collect(),
            line_numbering: args.contains(&"--line-numbers".to_string()),
            gitignore_patterns,
            always_exclude,
        })
    }

    pub fn default_excludes(&self) -> Vec<PathBuf> {
        vec![
            self.source_dir.join(".git"),
            self.source_dir.join("target"),
            self.source_dir.join(".idea"),
        ]
    }
}
