fn main() {
    let hash = std::env::var("OCARINA_GIT_HASH").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=GIT_HASH={hash}");
    println!("cargo:rerun-if-env-changed=OCARINA_GIT_HASH");
}