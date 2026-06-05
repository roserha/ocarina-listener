cross build --target aarch64-unknown-linux-gnu --release && scp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener root@$(cat ./scripts/secrets/RPIIP.txt):/usr/bin/ocarina-listener
