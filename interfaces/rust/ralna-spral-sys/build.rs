fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Skip building the bindings if we are on docs.rs, otherwise we will get build failures.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    // Generate Rust bindings for the public includes.
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_file_glob = format!("{project_dir}/../../../include/*.h");
    let mut bindings_builder = bindgen::Builder::default().derive_default(true);
    for header in glob::glob(&include_file_glob)? {
        match header {
            Ok(path) => {
                println!("cargo:rerun-if-changed={}", path.display());
                bindings_builder = bindings_builder.header(path.to_string_lossy().to_string());
            }
            Err(e) => eprintln!("Error reading header file: {}", e),
        }
    }
    let bindings = bindings_builder
        .generate()
        .expect("Failed to generate SPRAL bindings.");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    // SPRAL allows users to configure the libraries used for its dependencies. These environment
    // variables can be used to tell rustc which libraries to link against accordingly.
    const SPRAL_LIBBLAS_ENV_VAR: &str = "SPARAL_LIBBLAS";
    const SPARAL_OPENMP_ENV_VAR: &str = "SPARAL_OPENMP";
    const SPARAL_LIBLAPACK_ENV_VAR: &str = "SPARAL_LIBLAPACK";
    const SPARAL_LIBHWLOC_ENV_VAR: &str = "SPARAL_LIBHWLOC";
    const SPARAL_LIBMETIS_ENV_VAR: &str = "SPARAL_LIBMETIS";
    println!("cargo:rerun-if-env-changed={}", SPRAL_LIBBLAS_ENV_VAR);
    println!("cargo:rerun-if-env-changed={}", SPARAL_OPENMP_ENV_VAR);
    println!("cargo:rerun-if-env-changed={}", SPARAL_LIBLAPACK_ENV_VAR);
    println!("cargo:rerun-if-env-changed={}", SPARAL_LIBHWLOC_ENV_VAR);
    println!("cargo:rerun-if-env-changed={}", SPARAL_LIBMETIS_ENV_VAR);

    // Link against the dependencies (with defaults if nothing was explicitly requested).
    let blas_library = std::env::var(SPRAL_LIBBLAS_ENV_VAR).unwrap_or("blas".into());
    let openmp_library = std::env::var(SPARAL_OPENMP_ENV_VAR).unwrap_or("gomp".into());
    let lapack_library = std::env::var(SPARAL_LIBLAPACK_ENV_VAR).unwrap_or("lapack".into());
    let hwloc_library = std::env::var(SPARAL_LIBHWLOC_ENV_VAR).unwrap_or("hwloc".into());
    let metis_library = std::env::var(SPARAL_LIBMETIS_ENV_VAR).unwrap_or("metis".into());
    println!("cargo:rustc-link-lib=spral");
    println!("cargo:rustc-link-lib={}", openmp_library);
    println!("cargo:rustc-link-lib={}", blas_library);
    println!("cargo:rustc-link-lib={}", lapack_library);
    println!("cargo:rustc-link-lib={}", hwloc_library);
    println!("cargo:rustc-link-lib={}", metis_library);

    Ok(())
}
