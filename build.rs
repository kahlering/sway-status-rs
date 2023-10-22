//use std::env;

const ALSA_SRC: &str = "src/status_modules/audio_module/alsa.c";

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", ALSA_SRC);

    cc::Build::new()
        .file(ALSA_SRC)
        .compile("alsa");

    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=asound");
}