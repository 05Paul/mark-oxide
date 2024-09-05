mod error;
mod bitmap;
mod utils;

use crate::utils::test_diff;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    old: String,
    new: String,
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(err) = test_diff(arguments) {
        eprintln!("{}", err)
    }
}
