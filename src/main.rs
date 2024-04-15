mod config;
mod file_ops;
mod utils;
mod traversal;

use config::Config;
use traversal::concatenate_dir;
use file_ops::{initialize_output_file, read_gitignore};

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(config: &Config) -> io::Result<()> {
    let mut output = file_ops::initialize_output_file(&config.output_file)?;
    let gitignore_patterns = file_ops::read_gitignore(&config.source_dir)?;
    let always_exclude = config.default_excludes();

    concatenate_dir(config, &config.source_dir, 0)

}
