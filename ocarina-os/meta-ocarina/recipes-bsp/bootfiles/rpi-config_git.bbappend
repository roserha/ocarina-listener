do_deploy:append() {
    sed -i 's/dtparam=audio=on/dtparam=audio=off/' ${DEPLOYDIR}/bootfiles/config.txt
}
