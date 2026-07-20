SUMMARY = "Ocarina note listener"
LICENSE = "CLOSED"

SRC_URI = "file://ocarina-listener \
           file://ocarina-gui \
           file://ocarina-gui-launcher \
           file://ocarina-splash \
           file://ocarina-listener.init\
           file://sounds\
           file://ocarina-splash.init"

S = "${WORKDIR}"

RDEPENDS:${PN} = "alsa-lib fontconfig"

inherit update-rc.d

INITSCRIPT_NAME = "ocarina-listener"
INITSCRIPT_PARAMS = "defaults 99"

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${WORKDIR}/ocarina-listener ${D}${bindir}/ocarina-listener
    install -m 0755 ${WORKDIR}/ocarina-splash ${D}${bindir}/ocarina-splash
    install -m 0755 ${WORKDIR}/ocarina-gui ${D}${bindir}/ocarina-gui
    install -m 0755 ${WORKDIR}/ocarina-gui-launcher ${D}${bindir}/ocarina-gui-launcher

    install -d ${D}${INIT_D_DIR}
    install -m 0755 ${WORKDIR}/ocarina-listener.init ${D}${INIT_D_DIR}/ocarina-listener

    install -d ${D}${datadir}/ocarina/sounds
    cp -r ${WORKDIR}/sounds/*.wav ${D}${datadir}/ocarina/sounds

    install -m 0755 ${WORKDIR}/ocarina-splash.init ${D}${sysconfdir}/init.d/ocarina-splash
    install -d ${D}${sysconfdir}/rcS.d
    ln -sf ../init.d/ocarina-splash ${D}${sysconfdir}/rcS.d/S02ocarina-splash
}

FILES:${PN} = "${bindir}/ocarina-listener \
               ${bindir}/ocarina-gui \
               ${bindir}/ocarina-gui-launcher \
               ${bindir}/ocarina-splash \
               ${INIT_D_DIR}/ocarina-listener \
               ${datadir}/ocarina/sounds/*.wav \
               ${sysconfdir}/init.d/ocarina-splash \
               ${sysconfdir}/rcS.d/S02ocarina-splash""


