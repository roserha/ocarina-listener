if [ -z "$1" ]; then
    echo "Error: No argument provided!" >&2
    exit 1
fi

export OCARINA_GIT_HASH=$(git rev-parse --short HEAD)$(git diff --quiet || echo -dirty)

cross build --target aarch64-unknown-linux-gnu --release --workspace &&
    {
        ssh root@$1 "/etc/init.d/ocarina-listener stop"
        ssh root@$1 "/etc/init.d/ocarina-splash stop"
        scp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener root@$1:/usr/bin/ocarina-listener
        scp ./target/aarch64-unknown-linux-gnu/release/ocarina-splash root@$1:/usr/bin/ocarina-splash
        scp ./target/aarch64-unknown-linux-gnu/release/ocarina-gui root@$1:/usr/bin/ocarina-gui
        ssh root@$1 "/etc/init.d/ocarina-listener start"
    }

