use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|crates| crates.parent())
        .expect("server crate lives inside crates/ at the workspace root")
        .to_path_buf();

    let wasm_source = workspace_root.join("crates/wasm-particles/src/lib.rs");
    let wasm_output = workspace_root.join("static/js/particles.wasm");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../wasm-particles/src/lib.rs");
    println!("cargo::rerun-if-changed=../wasm-particles/Cargo.toml");
    println!("cargo::rerun-if-changed=../../static/js/particles.wasm");

    let stale = match (wasm_output.metadata(), wasm_source.metadata()) {
        (Ok(output), Ok(source)) => {
            source.modified().expect("source mtime") > output.modified().expect("output mtime")
        }
        _ => true,
    };
    if !stale {
        return;
    }

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(&cargo)
        .current_dir(&workspace_root)
        // tools like cargo-llvm-cov inject RUSTFLAGS (e.g. -C instrument-coverage)
        // that the wasm32 target cannot build with
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .args([
            "build",
            "-p",
            "wasm-particles",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
            "--target-dir",
            "target/wasm",
        ])
        .status()
        .expect("failed to invoke cargo for wasm-particles");

    if !status.success() {
        panic!(
            "failed to build particles.wasm; run `rustup target add wasm32-unknown-unknown` and retry"
        );
    }

    let built =
        workspace_root.join("target/wasm/wasm32-unknown-unknown/release/wasm_particles.wasm");
    std::fs::create_dir_all(wasm_output.parent().expect("static/js directory"))
        .expect("create static/js directory");
    std::fs::copy(&built, &wasm_output).expect("copy particles.wasm into static/js");
}
