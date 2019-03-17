extern crate cc;

use std::env;


fn main() {
    /*
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", project_dir);
    println!("cargo:rustc-link-lib=static=opengl32");
    */
    cc::Build::new()
        .file("src/glad.c")
        .compile("glad");
}