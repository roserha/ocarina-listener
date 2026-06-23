if [ -z "$1" ]; then
    echo "Error: No argument provided!" >&2
    exit 1
fi

cross build --target aarch64-unknown-linux-gnu --release --workspace \
 && {
    ssh root@$1 "/etc/init.d/ocarina-listener stop"
    scp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener root@$1:/usr/bin/ocarina-listener
    scp ./target/aarch64-unknown-linux-gnu/release/ocarina-gui root@$1:/usr/bin/ocarina-gui
    ssh root@$1 "/etc/init.d/ocarina-listener start"
 }