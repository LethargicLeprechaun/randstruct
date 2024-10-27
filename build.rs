extern crate bindgen;

use std::env;

fn main() {
    #[cfg(not(test))]
    bindgen::Builder::default()
        .header(
            env::var("SEED_HEADER_FILE").expect("Environment variable SEED_HEADER_FILE is not set"),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_cstr(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Unable to write bindings file");
}
