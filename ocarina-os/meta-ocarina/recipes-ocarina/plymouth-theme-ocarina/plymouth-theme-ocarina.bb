SUMMARY = "OcarinaOS Plymouth splash screen theme"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = "file://ocarina-splash"

S = "${WORKDIR}"

inherit allarch

do_install() {
    install -d ${D}${datadir}/plymouth/themes/ocarina-splash
    install -m 0644 ${WORKDIR}/ocarina-splash/ocarina-splash.plymouth \
        ${D}${datadir}/plymouth/themes/ocarina-splash/
    install -m 0644 ${WORKDIR}/ocarina-splash/ocarina-splash.script \
        ${D}${datadir}/plymouth/themes/ocarina-splash/
    install -m 0644 ${WORKDIR}/ocarina-splash/OcarinaOS.png \
        ${D}${datadir}/plymouth/themes/ocarina-splash/
}

FILES:${PN} = "${datadir}/plymouth/themes/ocarina-splash"

RDEPENDS:${PN} = "plymouth plymouth-plugin-script"