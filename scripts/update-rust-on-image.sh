cross build --target aarch64-unknown-linux-gnu --release

cp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener ./ocarina-os/meta-ocarina/recipes-ocarina/ocarina-listener/files
