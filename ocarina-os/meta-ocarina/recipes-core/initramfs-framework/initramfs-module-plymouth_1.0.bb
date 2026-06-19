SUMMARY = "initramfs support for plymouth"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = "file://plymouth"

S = "${WORKDIR}"

inherit allarch

do_install() {
    install -d ${D}/init.d
    install -m 0755 ${WORKDIR}/plymouth ${D}/init.d/20-plymouth
}

FILES:${PN} = "/init.d/20-plymouth"
RDEPENDS:${PN} = "initramfs-framework-base plymouth"