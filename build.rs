fn main() {
    println!("cargo:rustc-env=FFMPEG_DIR=/Users/jonathanburger/rust-ffmpeg-splitter/remotion");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-headerpad_max_install_names");
}
