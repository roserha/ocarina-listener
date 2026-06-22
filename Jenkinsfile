pipeline {
    agent any
    options {
        ansiColor('xterm')
    }
    stages {
        stage('Set Build Name') {
            steps {
                buildName("ocarina_os_${env.BRANCH_NAME}_${env.BUILD_NUMBER}")
            }
        }
        stage('Build Rust Binaries') {
            steps {
                sh '''
                    export PATH="$HOME/.cargo/bin:$PATH"
                    rustup default stable
                    cross build --target aarch64-unknown-linux-gnu --release
                    cp ./target/aarch64-unknown-linux-gnu/release/ocarina-listener \
                       ./ocarina-os/meta-ocarina/recipes-ocarina/ocarina-listener/files
                    RUSTFLAGS="-C target-feature=+crt-static" cross build --target aarch64-unknown-linux-gnu --release --bin ocarina-splash
                    cp ./target/aarch64-unknown-linux-gnu/release/ocarina-splash \
                       ./ocarina-os/meta-ocarina/recipes-ocarina/ocarina-listener/files
                    cp ./target/aarch64-unknown-linux-gnu/release/ocarina-splash \
                       ./ocarina-os/meta-ocarina/recipes-core/initramfs-framework/files
                    cd ./ocarina-gui
                    cross build --target aarch64-unknown-linux-gnu --release
                    cp ./target/aarch64-unknown-linux-gnu/release/ocarina-gui \
                       ./ocarina-os/meta-ocarina/recipes-ocarina/ocarina-listener/files
                    cd ..
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
                sh 'sudo chmod -R 777 ./ocarina-os'
                sh '''
                    sudo docker run --rm \
                      --device=/dev/kvm:/dev/kvm \
                      --device=/dev/net/tun:/dev/net/tun \
                      --cap-add NET_ADMIN \
                      --hostname buildserver \
                      -v /tftpboot:/tftpboot \
                      -v $(pwd)/ocarina-os:/home/build/work \
                      -v ocarina_bitbake_cache_volume:/home/build/my-build \
                      -v ocarina_downloads_volume:/home/build/data/downloads \
                      yoctocontainer \
                      bash /home/build/work/ci-build.sh
                '''
            }
        }

        stage('Flash SD Card') {
            when {
                expression { sh(script: 'test -b /dev/mmcblk0', returnStatus: true) == 0 }
            }
            options {
                catchError(buildResult: 'UNSTABLE', stageResult: 'UNSTABLE')
            }
            steps {
                sh '''
                    sudo umount /dev/mmcblk0p* || true
                    IMAGE=$(ls ./ocarina-os/core-image-base-raspberrypi3-64.rootfs-*.wic.bz2)
                    BMAP=$(ls ./ocarina-os/core-image-base-raspberrypi3-64.rootfs-*.wic.bmap 2>/dev/null || true)
                    if [ -n "$BMAP" ]; then
                        sudo bmaptool copy --bmap $BMAP $IMAGE /dev/mmcblk0
                    else
                        sudo bmaptool copy $IMAGE /dev/mmcblk0
                    fi
                '''
            }
        }

        stage('Archive Build') {
            steps {
                archiveArtifacts artifacts: 'ocarina-os/OcarinaOS*.tar.xz', fingerprint: true
            }
        }
    }
    post {
        always {
            sh '''
                sudo docker ps -a --filter "ancestor=yoctocontainer" --format "{{.ID}}" | xargs -r sudo docker rm -f || true
                sudo docker run --rm \
                -v ocarina_bitbake_cache_volume:/home/build/my-build \
                yoctocontainer \
                bash -c "rm -f /home/build/my-build/bitbake.lock /home/build/my-build/bitbake.sock" || true
                if [ -f /dev/mmcblk0 ] && [ ! -b /dev/mmcblk0 ]; then
                    sudo rm -f /dev/mmcblk0
                    sleep 1
                fi
            '''
        }
        success {
            echo "OcarinaOS build complete!"
        }
        failure {
            echo "Build failed. sstate cache preserved in ocarina_bitbake_cache_volume"
        }
    }
}