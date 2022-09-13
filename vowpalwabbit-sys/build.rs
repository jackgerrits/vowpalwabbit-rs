fn main() {
    match pkg_config::probe_library("vw_rs_bindings") {
        Ok(res) => {
            for link_path in &res.link_paths {
                println!("cargo:rustc-link-search=native={}", link_path.display());
            }

            for lib in &res.libs {
                println!("cargo:rustc-link-lib=dylib={}", lib);
            }

            // Exit early from build script
            return ();
        }
        // Failed to find it, move onto the next thing
        Err(_) => {
            ();
        }
    }

    // If pkg_config didn't find it. Just rely on standard system search paths.
    println!("cargo:rustc-link-lib=dylib=vw_rs_bindings");

    // Support some sort of VW_RS_ROOT envvar so that a Windows user can specify the include dir
    // println!(r"cargo:rustc-link-search=native=C:\Program Files (x86)\vowpalwabbit-rs-bindings\lib");
}
