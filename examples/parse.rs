use std::{env, path::PathBuf};

use opencli::OpenCliDocument;

fn main() {
    let path = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .expect("Specify path to OpenCLI document");

    let opencli = OpenCliDocument::from_path(path).unwrap();

    println!("{opencli:#?}");
}
