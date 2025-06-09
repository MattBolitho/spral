fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Skip building the bindings if we are on docs.rs, otherwise we will get build failures.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    // Generate bindings for the public includes.
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_file_glob = format!("{project_dir}/../../../include/*.h");
    let mut bindings_builder = bindgen::Builder::default();
    for header in glob::glob(&include_file_glob)? {
        match header {
            Ok(path) => {
                println!("cargo:rerun-if-changed={}", path.display());
                bindings_builder = bindings_builder.header(path.to_string_lossy().to_string());
            }
            Err(e) => eprintln!("Error reading header file: {}", e),
        }
    }

    let bindings = bindings_builder.generate().expect("Failed to generate SPRAL bindings.");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write SPRAL bindings.");

    Ok(())
}
