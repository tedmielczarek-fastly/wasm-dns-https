use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};

fn main() {
    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/blocklist.se");
    let input_data = fs::read_to_string(input_path).expect("Failed to read input blocklist");
    println!("cargo::rerun-if-changed=src/blocklist.se");
    let output_path = Path::new(&env::var("OUT_DIR").unwrap()).join("blocklist.rs");
    let mut outfile = BufWriter::new(File::create(&output_path).unwrap());

    let block_list_urls: Vec<&str> =
        serde_json::from_str(&input_data).expect("Failed to parse input blocklist");
    let mut s = phf_codegen::Set::new();
    for url in block_list_urls {
        s.entry(url);
    }
    write!(
        &mut outfile,
        "static BLOCKED_HOSTS: phf::Set<&'static str> = {}",
        s.build()
    )
    .expect("Failed to write generated code");
    write!(&mut outfile, ";\n").unwrap();
}
