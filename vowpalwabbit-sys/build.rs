use std::{path::PathBuf, env};

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
            return;
        }
        // Failed to find it, move onto the next thing
        Err(_) => ()
    }

    // Not using pkg-config. See if the root dir env var was set.
    match env::var("VW_RS_BINDING_HOME")
    {
        Ok(dir) =>
        {
            let root_dir = PathBuf::from(dir);
            // For dll link lib
            println!("cargo:rustc-link-search=native={}", root_dir.display());
            println!("cargo:rustc-link-search=native={}", root_dir.join("lib").display());
            println!("cargo:rustc-link-lib=dylib=vw_rs_bindings");

            // Exit early from build script
            return;
        }
        Err(_) => ()
    }

    println!("cargo:rustc-link-lib=dylib=vw_rs_bindings");
}
