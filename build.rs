extern crate pkg_config;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
fn main() {
    pkg_config::Config::new().atleast_version("8.6.1").probe("libvw_c_wrapper").unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate().unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}