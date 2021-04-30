use std::path::PathBuf;

fn main() {
    let cur_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let little_engine_include_dir = cur_dir.join("3rdparty");

    let source_files = vec!["src/lib.rs"];
    cxx_build::bridges(source_files)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .include(little_engine_include_dir.clone())
        .include("src")
        .file("src/lib.cc")
        .file("3rdparty/little_engine.cpp")
        .compile("little-engine");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.cc");
    println!("cargo:rerun-if-changed=src/lib.h");
}