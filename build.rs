#[cfg(feature = "cbindgen")]
extern crate cbindgen;

#[cfg(feature = "cbindgen")]
use std::env;

#[cfg(feature = "cbindgen")]
use std::path::PathBuf;

fn common_actions() {
    println!("cargo:rustc-flags=-l z");
    cdylib_link_lines::metabuild();
}

#[cfg(not(feature = "cbindgen"))]
fn main() {
    common_actions();
}

#[cfg(feature = "cbindgen")]
fn main() {
    common_actions();

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate_with_config(
        crate_dir.clone(),
        cbindgen::Config::from_root_or_default(crate_dir.clone()),
    )
    .expect("Unable to generate bindings")
    .write_to_file("htp.h");

    // Write a version.h to include with the c_api
    let mut hdr_path = PathBuf::from(crate_dir.clone());
    hdr_path.push("version.h");
    std::fs::write(
        hdr_path,
        format!(
            "#define HTP_VERSION_STRING_FULL \"LibHTP v{}\"\n",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .expect("Could not write version.h");
}
