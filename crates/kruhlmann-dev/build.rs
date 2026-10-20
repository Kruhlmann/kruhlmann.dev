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
    let wasm_output = workspace_root.join("static/wasm/particles.wasm");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../wasm-particles/src/lib.rs");
    println!("cargo::rerun-if-changed=../wasm-particles/Cargo.toml");
    println!("cargo::rerun-if-changed=../../static/wasm/particles.wasm");

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
    // tools like cargo-llvm-cov inject rustflags (e.g. -C instrument-coverage) via
    // RUSTFLAGS/RUSTC_WRAPPER env vars that the wasm32 target cannot build with
    let poisoned_env: Vec<String> = env::vars()
        .map(|(key, _)| key)
        .filter(|key| {
            key == "RUSTFLAGS"
                || key == "CARGO_ENCODED_RUSTFLAGS"
                || key == "CARGO_BUILD_RUSTFLAGS"
                || key == "RUSTDOCFLAGS"
                || key == "RUSTC_WRAPPER"
                || key.starts_with("__CARGO_LLVM_COV")
        })
        .collect();
    let mut command = Command::new(&cargo);
    for key in poisoned_env {
        command.env_remove(key);
    }
    let status = command
        .current_dir(&workspace_root)
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
