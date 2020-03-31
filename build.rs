#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-flags=-l z");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-flags=-l z");
}
