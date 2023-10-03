use std::env;

const ALSA_SRC: &str = "src/status_modules/audio_module/alsa.c";

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", ALSA_SRC);

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{out_dir}");
    
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let alsa_dir = dir + "/src/status_modules/audio_module";
    println!("{}", &alsa_dir);


    cc::Build::new()
        .file(ALSA_SRC)
        .compile("alsa");

    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=asound");
}