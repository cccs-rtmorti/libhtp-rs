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
    extern crate cbindgen;
    use std::env;
    use std::path::PathBuf;

    common_actions();

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let hdr_path = if let Ok(path) = env::var("CBINDGEN_HEADERS_DIR") {
        PathBuf::from(path)
    } else {
        PathBuf::from(crate_dir.clone())
    };
    let htp_h_path = hdr_path.join("htp/htp.h");
    let version_h_path = hdr_path.join("htp/version.h");

    cbindgen::generate_with_config(
        crate_dir.clone(),
        cbindgen::Config::from_root_or_default(crate_dir),
    )
    .expect("Unable to generate bindings")
    .write_to_file(htp_h_path);

    // Write a version.h to include with the c_api
    std::fs::write(
        version_h_path,
        format!(
            "#define HTP_VERSION_STRING_FULL \"LibHTP v{}\"\n",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .expect("Could not write version.h");
}
