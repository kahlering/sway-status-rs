use std::process::Command;
use std::env;
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/hello.c");

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{out_dir}");
    
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let alsa_dir = dir + "/src/alsa";
    println!("{}", &alsa_dir);

    let status = Command::new("make").args(["-C", &alsa_dir]).status().expect("failed to make!");
    println!("process finished with: {status}");

    assert!(status.success());

    println!("cargo:rustc-link-search={alsa_dir}");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    
    println!("cargo:rustc-link-lib=static=alsa_test");
    println!("cargo:rustc-link-lib=asound");
}