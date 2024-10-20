use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("qtip").build();
    println!("{dst:?}");

    let inc_dir = dst.join("include");

    // Tell cargo to look for shared libraries in the specified directory

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .ctypes_prefix("cty")
        .use_core()
        // The input header we would like to generate
        // bindings for.
        .header(inc_dir.join("qtip.h").to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("qtip.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=qtip"); // Replace `qtip` with the actual name of your C library
    println!(
        "cargo:rustc-link-search=native={path}",
        path = out_path.join("lib").to_str().unwrap()
    ); // Replace with the actual path to your C library
}
