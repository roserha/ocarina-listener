SUMMARY = "InitRAMFS support for Ocarina Splash Screen"
LICENSE = "CLOSED"
SRC_URI = "file://ocarina_splash"

S = "${WORKDIR}"

inherit allarch

do_install() {
    install -d ${D}/init.d
    install -m 0755 ${WORKDIR}/ocarina_splash ${D}/init.d/20-ocarina_splash
}

FILES:${PN} = "/init.d/20-ocarina_splash"
RDEPENDS:${PN} = "initramfs-framework-base"