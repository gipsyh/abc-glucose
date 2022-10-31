use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo:include=abc/src");
    println!("cargo:rustc-link-search=native=abc");
    println!("cargo:rustc-link-lib=static=abc");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    if Path::new(".git").is_dir() {
        Command::new("git")
            .args(["submodule", "update", "--init"])
            .status()
            .expect("Failed to update submodules.");
    } else {
        assert!(Path::new("abc").is_dir(), "abc source not included");
    }

    Command::new("make")
        .current_dir("./abc")
        .args(["ABC_USE_NO_READLINE=1", "libabc.a", "-j"])
        .status()
        .expect("Failed to build abc using make");

    let bindings = bindgen::Builder::default()
        .header("abc/src/sat/glucose2/AbcGlucose2.h")
        .clang_arg("-Iabc/src")
        .clang_arg("-DLIN64")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
