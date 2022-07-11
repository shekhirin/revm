extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/intx.cpp")
        .flag("-std=c++17")
        .include("cpp")
        .compile("intx");
} 