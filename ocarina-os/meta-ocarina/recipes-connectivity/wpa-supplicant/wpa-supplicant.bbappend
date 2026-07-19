FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append = "file://wpa_supplicant.conf"

do_install:append() {
    install -d ${D}${sysconfdir}
    install -m 0600 ${WORKDIR}/wpa_supplicant.conf ${D}${sysconfdir}/wpa_supplicant.conf
}

FILES:{$PN}:append = "${sysconfdir}/wpa_supplicant.conf"
