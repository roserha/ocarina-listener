pipeline {
    agent any

    stages {
        stage('Build Rust Binary') {
            steps {
                sh '''
                    export PATH="$HOME/.cargo/bin:$PATH"
                    rustup default stable
                    cross build --target aarch64-unknown-linux-gnu --release
                    cp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener \
                       ./ocarina-os/meta-ocarina/recipes-ocarina/ocarina-listener/files
                '''
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -t yoctocontainer ./ocarina-os'
            }
        }

        stage('Yocto Build') {
            steps {
                sh '''
                    sudo docker run --rm \
                      --device=/dev/kvm:/dev/kvm \
                      --device=/dev/net/tun:/dev/net/tun \
                      --cap-add NET_ADMIN \
                      --hostname buildserver \
                      -v /tftpboot:/tftpboot \
                      -v $(pwd)/ocarina-os:/home/build/work \
                      -v ocarina_bitbake_cache_volume:/home/build/my-build \
                      yoctocontainer \
                      bash /home/build/work/ci-build.sh
                '''
            }
        }

        stage('Flash SD Card') {
            steps {
                sh '''
                    umount /dev/mmcblk0p* || true
                    sudo bmaptool copy core-image-base-raspberrypi3-64.rootfs-*.wic.bz2 /dev/mmcblk0
                '''
            }
        }
    }
}