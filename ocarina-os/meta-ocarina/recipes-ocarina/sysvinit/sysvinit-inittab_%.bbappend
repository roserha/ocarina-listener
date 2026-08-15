FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

do_install:append() {
    sed -i 's|1:2345:respawn:/sbin/getty 38400 tty1|1:2345:respawn:/sbin/getty --autologin root 38400 tty1|' ${D}${sysconfdir}/inittab
}