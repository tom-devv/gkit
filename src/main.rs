use clap::Parser;
use gitkit::GKitArgs;
use std::process;

fn main() {
    let args = GKitArgs::parse();

    match gitkit::run(args) {
        Ok(_) => process::exit(1),
        Err(err) => eprintln!("Process failed with: {}", err),
    }
}
