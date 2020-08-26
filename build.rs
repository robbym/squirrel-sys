extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

/// Returns the default C++ standard library for the current target: `libc++`
/// for OS X and `libstdc++` for anything else.
///
/// Credits: cc-rs crate
fn get_cpp_link_stdlib() -> Option<String> {
    if let Ok(stdlib) = env::var("CXXSTDLIB") {
        if stdlib.is_empty() {
            None
        } else {
            Some(stdlib)
        }
    } else {
        let target = env::var("TARGET").unwrap();
        if target.contains("msvc") {
            None
        } else if target.contains("apple")
            || target.contains("freebsd")
            || target.contains("openbsd")
        {
            Some("c++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    }
}

fn main() {
    let dst = cmake::build("squirrel");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=squirrel_static");
    if let Some(stdlib) = get_cpp_link_stdlib() {
        println!("cargo:rustc-flags=-l dylib={}", stdlib);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

