cross build --target aarch64-unknown-linux-gnu --release && scp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener pi@$(cat ./scripts/secrets/RPIIP.txt):~/ocarina-listener
