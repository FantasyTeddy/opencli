# opencli

[![Crates.io](https://img.shields.io/crates/v/opencli.svg)](https://crates.io/crates/opencli)
[![Documentation](https://docs.rs/opencli/badge.svg)](https://docs.rs/opencli/)

This crate provides structures and support for serializing and deserializing [OpenCLI](https://opencli.org/) specifications.

# Examples

```rust
use opencli::OpenCliDocument;

fn main() {
    let opencli = OpenCliDocument::from_path("path/to/opencli.yaml").unwrap();

    println!("{opencli:#?}");
}
```