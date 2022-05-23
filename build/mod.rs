extern crate bindgen;
use std::env;
use std::path::PathBuf;
use std::path::Path;
use anyhow::Result;


const LIBV5_LIBRARY: &str = "build/libv5rt";

const LIBS: [&str; 3] = ["c","m","v5rt"];



fn main() -> Result<()>{

    // Get the directory to place data in
    let data_dir = dirs::home_dir().unwrap();
    let data_dir = data_dir.join(".v5");

    // If it doesn't exist, create it
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir)?;
    }

    // Make the path to the libv5_dir
    let libv5_dir = data_dir.join("libv5rt");

    // If the path the the libv5 dir does not exist then create it
    if !libv5_dir.exists() {
        std::fs::create_dir_all(&libv5_dir)?;
    }

    

    // If the .v5init does not exist in the data directory
    // we need to install the required libraries
    let init_file = data_dir.join(".v5init");
    if !init_file.exists() {
        // Get the LIBV5_PATH environment variable
        let libv5_path = std::env::var("LIBV5_PATH");

        // If it is none, we can not upload
        if libv5_path.is_err() {
            return Err(anyhow::anyhow!("Could not find LIBV5_PATH environment variable"));
        }

        let libv5_path = libv5_path.unwrap();

        // Make the path to the libv5_dir
        let libv5_dir = data_dir.join("libv5rt");
        
        // If it does not exist, create it
        if !libv5_dir.exists() {
            std::fs::create_dir_all(&libv5_dir)?;
        }

        // Copy all files from libv5_path to the data directory
        let mut co = fs_extra::dir::CopyOptions::new();
        co.content_only = true;
        fs_extra::dir::copy(libv5_path, libv5_dir, &co)?;

        // Create the .v5init file
        std::fs::write(&init_file, "")?;
    }

    // Get the build directory
    let build_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let build_dir = Path::new(&build_dir);
    
    // Find the absolute path to all required files and directories
    let libv5_library = build_dir.join(libv5_dir.clone()).join("sdk/vexv5");
    let libv5_include_library = build_dir.join(libv5_dir.clone()).join("sdk/vexv5/include");
    let gcc_include = build_dir.join(libv5_dir.clone()).join("sdk/vexv5/gcc/include");
    let wrapper = build_dir.join(LIBV5_LIBRARY.clone()).join("wrapper.h");
    
    // Add the libv5 path to the list to search for libraries in.
    println!("cargo:rustc-link-search={}", libv5_library.display());

    // Iterate over lists of libraries and tell cargo to link them
    for file in LIBS {
        println!("cargo:rustc-link-lib={}", file);
    }

    // We want to rerun this script if wrapper is changed
    println!("cargo:rerun-if-changed={}", wrapper.display());
    
    // Generate bindings
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", &libv5_include_library.display()))
        .clang_arg(format!("-I{}", &gcc_include.display()))
        .header(&wrapper.display().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");
    
    // Write bindings to file
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings.");

    Ok(())
}