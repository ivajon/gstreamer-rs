// This file was generated by gir (https://github.com/gtk-rs/gir @ 8f15e55)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
extern crate pkg_config;

#[cfg(not(feature = "dox"))]
use pkg_config::{Config, Error};
#[cfg(not(feature = "dox"))]
use std::env;
#[cfg(not(feature = "dox"))]
use std::io;
#[cfg(not(feature = "dox"))]
use std::io::prelude::*;
#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

#[cfg(not(feature = "dox"))]
fn find() -> Result<(), Error> {
    let package_name = "gstreamer-rtsp-server-1.0";
    let shared_libs = ["gstrtspserver-1.0"];
    let version = if cfg!(feature = "v1_16") {
        "1.16"
    } else if cfg!(feature = "v1_14") {
        "1.14"
    } else if cfg!(feature = "v1_12") {
        "1.12"
    } else if cfg!(feature = "v1_10") {
        "1.10"
    } else if cfg!(feature = "v1_8") {
        "1.8"
    } else if cfg!(feature = "v1_6") {
        "1.6"
    } else if cfg!(feature = "v1_4") {
        "1.4"
    } else if cfg!(feature = "v1_2") {
        "1.2"
    } else {
        "1.0"
    };

    if let Ok(inc_dir) = env::var("GTK_INCLUDE_DIR") {
        println!("cargo:include={}", inc_dir);
    }
    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(());
    }

    let target = env::var("TARGET").expect("TARGET environment variable doesn't exist");
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);
    config.print_system_libs(false);
    if hardcode_shared_libs {
        config.cargo_metadata(false);
    }
    match config.probe(package_name) {
        Ok(library) => {
            if let Ok(paths) = std::env::join_paths(library.include_paths) {
                println!("cargo:include={}", paths.to_string_lossy());
            }
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!(
                        "cargo:rustc-link-search=native={}",
                        path.to_str().expect("library path doesn't exist")
                    );
                }
            }
            Ok(())
        }
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}
