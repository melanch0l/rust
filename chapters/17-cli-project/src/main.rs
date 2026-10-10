use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn std::error::Error>> {
    let needle = args
        .next()
        .ok_or("usage: chapter-17-cli-project <needle> <file>")?;
    let path = args
        .next()
        .ok_or("usage: chapter-17-cli-project <needle> <file>")?;
    let contents = fs::read_to_string(path)?;

    for (line_number, line) in contents.lines().enumerate() {
        if line.contains(&needle) {
            println!("{}: {line}", line_number + 1);
        }
    }
    Ok(())
}
