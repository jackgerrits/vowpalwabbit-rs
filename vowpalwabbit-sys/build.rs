// Tries several things in sequence
// 1. Check ENVVAR VW_RS_BINDINGS_DIR for where to find lib vw_rs_bindings
// 2. Check pkgconfig for vw_rs_bindings package
// 3. Build package from source

fn main() {
    match std::env::var("VW_RS_BINDINGS_DIR")
    {
        Ok(path) =>
        {
            println!("cargo:rustc-link-search=native={}", path);
            println!("cargo:rustc-link-lib=dylib=vw_rs_bindings");
            println!("cargo:rustc-env=LD_LIBRARY_PATH={}", path);
            // Exit early from build script
            return ();
        }
        // Failed to find it, move onto the next thing
        Err(_) => (),
    }

    match pkg_config::probe_library("vw_rs_bindings") {
        Ok(res) => {
            for link_path in &res.link_paths {
                println!("cargo:rustc-link-search=native={}", link_path.display());
                println!("cargo:rustc-env=LD_LIBRARY_PATH={}", link_path.display());
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

    // For some reason on Windows I had to force exception handling to be turned on.
    let exception_handling_flag = if cfg!(target_os = "windows") {
        "/EHsc"
    } else {
        ""
    };
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new("binding")
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", out_path.join("lib"))
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", out_path.join("lib"))
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", out_path.join("bin"))
        .build_target("vw_rs_bindings")
        .cxxflag(exception_handling_flag)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin/Release").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib/Release").display()
    );
    println!("cargo:rustc-link-lib=dylib=vw_rs_bindings");
}
