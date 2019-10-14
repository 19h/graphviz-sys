use bindgen::builder;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=gvc");
    println!("cargo:rustc-link-lib=cgraph");
    println!("cargo:rustc-link-lib=pathplan");
    println!("cargo:rustc-link-lib=cdt");

    let bindings =
        builder()
            .header("./wrapper.h")
            .generate()
            .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
