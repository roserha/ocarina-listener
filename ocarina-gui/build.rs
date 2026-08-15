fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
    let hash = std::env::var("OCARINA_GIT_HASH").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=GIT_HASH={hash}");
    println!("cargo:rerun-if-env-changed=OCARINA_GIT_HASH");
}