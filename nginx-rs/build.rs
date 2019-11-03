extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    let nginx_dir = env::var("NGINX_DIR").unwrap_or(String::from("../../nginx"));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .layout_tests(false)
        .whitelist_type("ngx_.*")
        .whitelist_function("ngx_.*")
        .whitelist_var("NGX_.*|ngx_.*|nginx_.*")
        .clang_arg(format!("-I{}/src/core", nginx_dir))
        .clang_arg(format!("-I{}/src/event", nginx_dir))
        .clang_arg(format!("-I{}/src/event/modules", nginx_dir))
        .clang_arg(format!("-I{}/src/os/unix", nginx_dir))
        .clang_arg(format!("-I{}/objs", nginx_dir))
        .clang_arg(format!("-I{}/src/http", nginx_dir))
        .clang_arg(format!("-I{}/src/http/modules", nginx_dir))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
