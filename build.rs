use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=lib.c");

    // Get the output directory from the environment variable OUT_DIR
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");

    // Compile lib.c into lib.o in the output directory
    let compile_lib = Command::new("gcc")
        .arg("-c")
        .arg("src/hello.c")
        .arg("-o")
        .arg(&format!("{}/hello.o", out_dir))
        .status()
        .expect("Failed to compile hello.c into hello.o");

    if !compile_lib.success() {
        panic!("Compilation of hello.c into hello.o failed");
    }

    // Create lib.a from lib.o in the output directory
    let create_lib = Command::new("ar")
        .arg("rcs")
        .arg(&format!("{}/libhello.a", out_dir))
        .arg(&format!("{}/hello.o", out_dir))
        .status()
        .expect("Failed to create libhello.a from hello.o");

    if !create_lib.success() {
        panic!("Creation of libhello.a from hello.o failed");
    }

    // No need to remove lib.o explicitly since it's in the output directory

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello");
}
