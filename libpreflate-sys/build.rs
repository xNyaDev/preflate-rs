use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    {
        let mut cmake_lists = String::new();
        let mut cmake_lists_file = File::open("vendor/preflate/CMakeLists.txt")?;

        cmake_lists_file.read_to_string(&mut cmake_lists)?;

        if cmake_lists.starts_with("cmake_minimum_required(VERSION 3.2)") {
            // Upgrade the minimum cmake version for a successful build, replace 3.2 with 3.5
            cmake_lists = cmake_lists.replacen("3.2", "3.5", 1);

            // Add archive destination to link the library later on
            cmake_lists += "install(TARGETS preflate ARCHIVE DESTINATION lib)";
        }

        let mut cmake_lists_file = File::create("vendor/preflate/CMakeLists.txt")?;
        cmake_lists_file.write_all(cmake_lists.as_bytes())?;
    }

    let dst = cmake::Config::new("vendor/preflate").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=preflate");

    cxx_build::bridge("src/lib.rs").compile("preflate-bridge");

    Ok(())
}
