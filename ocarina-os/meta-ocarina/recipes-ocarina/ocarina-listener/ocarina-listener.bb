SUMMARY = "Ocarina note listener"
LICENSE = "CLOSED"

SRC_URI = "file://ocarina-listener \
           file://ocarina-listener.init \
           file://googlevoicehat-soundcard.dtbo"

S = "${WORKDIR}"

RDEPENDS:${PN} = "alsa-lib"

inherit update-rc.d

INITSCRIPT_NAME = "ocarina-listener"
INITSCRIPT_PARAMS = "defaults 99"

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${WORKDIR}/ocarina-listener ${D}${bindir}/ocarina-listener

    install -d ${D}${INIT_D_DIR}
    install -m 0755 ${WORKDIR}/ocarina-listener.init ${D}${INIT_D_DIR}/ocarina-listener

    install -d ${D}/boot/overlays
    install -m 0644 ${WORKDIR}/googlevoicehat-soundcard.dtbo ${D}/boot/overlays/
}

FILES:${PN} = "${bindir}/ocarina-listener ${INIT_D_DIR}/ocarina-listener /boot/overlays/googlevoicehat-soundcard.dtbo"
