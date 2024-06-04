use std::process;
use clap::{Args, Parser};
use crate::utils::{output_tests, test_cases};

mod error;
mod utils;

#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    #[command(flatten)]
    input: Input,
    #[command(flatten)]
    output: Output,

}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Input {
    #[arg(long)]
    input_file: Option<String>,
    #[arg(long)]
    test_version: Option<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Output {
    #[arg(long)]
    output_file: Option<String>,
    #[arg(long)]
    stdout: bool,
}

fn main() {
    let args = Arguments::parse();

    let cases = match test_cases(args.input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Could not read tests: {err}");
            process::exit(1)
        }
    };

    match output_tests(args.output, cases) {
        Ok(_) => println!("Test created"),
        Err(err) => {
            eprintln!("Could not output tests: {err}");
            process::exit(1)
        }
    }
}
