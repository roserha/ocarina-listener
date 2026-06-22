SUMMARY = "Ocarina note listener"
LICENSE = "CLOSED"

SRC_URI = "file://ocarina-listener \
           file://ocarina-gui \
           file://ocarina-splash \
           file://ocarina-listener.init"

S = "${WORKDIR}"

RDEPENDS:${PN} = "alsa-lib"

inherit update-rc.d

INITSCRIPT_NAME = "ocarina-listener"
INITSCRIPT_PARAMS = "defaults 99"

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${WORKDIR}/ocarina-listener ${D}${bindir}/ocarina-listener
    install -m 0755 ${WORKDIR}/ocarina-splash ${D}${bindir}/ocarina-splash
    install -m 0755 ${WORKDIR}/ocarina-gui ${D}${bindir}/ocarina-gui

    install -d ${D}${INIT_D_DIR}
    install -m 0755 ${WORKDIR}/ocarina-listener.init ${D}${INIT_D_DIR}/ocarina-listener
}

FILES:${PN} = "${bindir}/ocarina-listener \
               ${bindir}/ocarina-gui \
               ${bindir}/ocarina-splash \
               ${INIT_D_DIR}/ocarina-listener"
