use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;


pub struct Config {
    pub source_dir: PathBuf,
    pub output_file: PathBuf,
    pub exclude_suffixes: Vec<String>,
}


impl Config {
    pub fn parse_arguments(args: &[String]) -> io::Result<Config> {
        if args.len() < 3 {
            let err = Error::new(ErrorKind::InvalidInput, "Insufficient arguments provided");
            return Err(err);
        }

        Ok(Config {
            source_dir: PathBuf::from(&args[1]),
            output_file: PathBuf::from(&args[2]),
            exclude_suffixes: args[3..].iter().cloned().collect(),
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
