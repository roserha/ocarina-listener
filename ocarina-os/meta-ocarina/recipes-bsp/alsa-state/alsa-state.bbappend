FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append = " file://asound.conf file://asound.state"

do_install:append() {
    install -m 0644 ${WORKDIR}/asound.conf ${D}${sysconfdir}/asound.conf
    install -m 0644 ${WORKDIR}/asound.state ${D}${localstatedir}/lib/alsa/asound.state
}
