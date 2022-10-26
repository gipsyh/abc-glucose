use std::{env, path::PathBuf, process::Command};

fn main() {
    Command::new("make")
        .current_dir("./abc")
        .args(&["ABC_USE_NO_READLINE=1", "libabc.a", "-j"])
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
