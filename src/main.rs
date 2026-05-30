use std::process;

use clap::Parser;
use gkit::GKitArgs;

fn main() {
    let args = GKitArgs::parse();

    match gkit::run(args) {
        Ok(_) => process::exit(1),
        Err(err) => eprintln!("Process failed with: {}", err),
    }
}
