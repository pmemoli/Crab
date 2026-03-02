use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Need a source file to compile.",
        ));
    }

    let source_file = &args[1];

    // Runs preprocessor
    let preprocessor_file_name = "temp.i";
    let status = Command::new("gcc")
        .arg("-E") // Run only preprocessor
        .arg("-P") // No linemarkers
        .arg(source_file)
        .arg("-o")
        .arg(preprocess_file_name);

    // Runs compiler (TODO)
    let assembly_file = "temp.s";
    fs::remove_file("./{}", preprocess_file_name);

    // Runs linker
    let output_file = source_file.strip_suffix(".c").unwrap_or(source_file);
    let status = Command::new("gcc")
        .arg(assembly_file);
        .arg("-o")
        .arg(preprocess_file_name);
    
    fs::remove_file("./{}", assembly_file);

    Ok(())
}
