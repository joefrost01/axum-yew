use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {

    /*
    // Trigger rebuild if anything in the frontend source changes.
    println!("cargo:rerun-if-changed=../frontend/src");

    // Build the Yew project.
    // Here we assume your frontend is in a sibling directory "frontend"
    let status = Command::new("trunk")
        .args(&["build", "--release", "--public-url", "/"])
        .current_dir("frontend")
        .status()
        .expect("failed to build the frontend using trunk");
    if !status.success() {
        panic!("Frontend build failed");
    }

    // Define the path to your backend's static directory.
    // Adjust this path as necessary.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let static_dir = Path::new(&manifest_dir).join("static");
    let assets_dir = Path::new(&static_dir).join("assets");

    // Create the static directory if it doesn't exist.
    fs::create_dir_all(&static_dir).expect("failed to create static directory");
    fs::create_dir_all(&assets_dir).expect("failed to create static directory");

    // Copy the built assets (adjust filenames as produced by trunk)
    fs::copy("frontend/dist/index.html", static_dir.join("index.html"))
        .expect("failed to copy index.html");
    fs::copy("frontend/dist/frontend_bg.wasm", static_dir.join("frontend_bg.wasm"))
        .expect("failed to copy frontend_bg.wasm");
    fs::copy("frontend/dist/frontend.js", static_dir.join("frontend.js"))
        .expect("failed to copy frontend.js");
    fs::copy("frontend/dist/assets/logo.png", assets_dir.join("logo.png"))
        .expect("failed to copy logo.png");

    // If there are additional assets (JS glue, CSS, etc.), copy them here.

    */

}