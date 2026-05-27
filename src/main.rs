use std::path::PathBuf;

use gkit::{
    git::kit::GRepo,
    metrics::{GitMetric, cadence::CadenceMetric},
};

fn main() {
    let repo_path = PathBuf::from("../ghexample");
    let repo = GRepo::open(&repo_path).unwrap();

    CadenceMetric::new().calculate(&repo);
    println!("Done!");
}
