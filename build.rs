use bindgen;
use bindgen::builder;

fn main() {
    let bindings =
        builder()
            .header("./graphviz.h")
            .generate()
            .unwrap();

    bindings.write_to_file("src/lib.rs").unwrap();

    println!("cargo:rustc-flags=-L /usr/local/lib/");
    println!("cargo:rustc-link-search=/usr/local/lib/");

    println!("cargo:rustc-flags=-L /opt/local/lib/");
    println!("cargo:rustc-link-search=/opt/local/lib/");

    println!("cargo:rustc-link-lib=gvc");
    println!("cargo:rustc-link-lib=cgraph");
    println!("cargo:rustc-link-lib=pathplan");
    println!("cargo:rustc-link-lib=cdt");
}
