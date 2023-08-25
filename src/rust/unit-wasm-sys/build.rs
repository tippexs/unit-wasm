// buildscript for the unit-wasm-sys crate.

use std::env;
use std::path::{PathBuf};

fn main() {
    // Tell rustc where to find the libunit-wasm library.
    let dst = env::var("OUT_DIR").unwrap();
    // Sosme generics
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    // The rustc-link-search tells Cargo to pass the `-L` flag to the
    // compiler to add a directory to the library search plugin. The
    // `native` keyword means "only looking for `native libraries` in
    // this directory".

    // The rustc-link-lib tells Cargo to link the given library using
    // the compiler's `-l` flag. This is needed to start building our
    // FFIs.
    
    generate_bindings();
    
    
    println!("cargo:rustc-link-search=native={}", &dst);
}

fn generate_bindings() {
    
    let wasi_sysroot = "--sysroot=".to_owned() + &env::var("WASI_SYSROOT").expect("WASI_SYSROOT is not defined!");

    //build the libunit-wasm
    let mut cfg = cc::Build::new();
    
    // Use `cfg` CC::Builder to build the libunit-wasm.c. 
    // This step is important to have the static libs to build the Rust bindings
    
    cfg
        .file("libunit-wasm/libunit-wasm.c")
        .include("libunit-wasm/include")
        .flag(&wasi_sysroot)
        .flag("-fno-strict-aliasing")
        .warnings(false)
        .compile("libunit-wasm");


    let bindings = bindgen::Builder::default()
        // The input header file.
        .header("libunit-wasm/include/unit/unit-wasm.h")
        .allowlist_function("^luw_.*")
        .allowlist_var("^luw_.*")
        .allowlist_type("^luw_.*")
        .clang_args(vec![wasi_sysroot]) // Needed for strings.h
        .generate()
        .expect("Unable to generate bindings");

    let out_dir_env =
        env::var("OUT_DIR").expect("The required environment variable OUT_DIR was not set");
   
    let out_path = PathBuf::from(out_dir_env);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
