use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_dir> <output_file>", args[0]);
        std::process::exit(1);
    }

    let source_dir = &args[1];
    let output_file = &args[2];

    let mut output = File::create(output_file)?;

    // Start the recursive concatenation process
    concatenate_dir(Path::new(source_dir), &mut output, 0)?;

    Ok(())
}

fn concatenate_dir(path: &Path, output: &mut File, depth: usize) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                concatenate_dir(&path, output, depth + 1)?;
            } else {
                // Write the file path and content to the output file
                writeln!(output, "\n// File: {:?} Depth: {}\n", path.display(), depth)?;
                let mut file = File::open(path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                writeln!(output, "{}", contents)?;
            }
        }
    }
    Ok(())
}
