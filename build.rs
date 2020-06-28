use bindgen::builder;

use std::env;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    println!("cargo:rustc-link-lib=gvc");
    println!("cargo:rustc-link-lib=cgraph");
    println!("cargo:rustc-link-lib=pathplan");
    println!("cargo:rustc-link-lib=cdt");

    let ignored_macros = IgnoreMacros(
                vec![
                    "FP_INFINITE".into(),
                    "FP_NAN".into(),
                    "FP_NORMAL".into(),
                    "FP_SUBNORMAL".into(),
                    "FP_ZERO".into(),
                ]
                .into_iter()
                .collect(),
            );

    let bindings =
        builder()
            .header("./wrapper.h")
            .parse_callbacks(Box::new(ignored_macros))
            .generate()
            .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
