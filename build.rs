extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("squirrel");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=squirrel_static");
    if !cfg!(windows) {
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}