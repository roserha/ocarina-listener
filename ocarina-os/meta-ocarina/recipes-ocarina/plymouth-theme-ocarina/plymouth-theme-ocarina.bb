SUMMARY = "OcarinaOS Plymouth splash screen theme"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://COPYING;md5=94d55d512a9ba36caa9b7df079bae19f"

FILESEXTRAPATHS:prepend := "${THISDIR}/files:"
SRC_URI = "file://ocarina-splash.plymouth \
           file://ocarina-splash.script \
           file://OcarinaOS.png"

inherit allarch

do_install() {
    install -d ${D}${datadir}/plymouth/themes/ocarina-splash
    install -m 0644 ${WORKDIR}/ocarina-splash.plymouth \
        ${D}${datadir}/plymouth/themes/ocarina-splash/
    install -m 0644 ${WORKDIR}/ocarina-splash.script \
        ${D}${datadir}/plymouth/themes/ocarina-splash/
    install -m 0644 ${WORKDIR}/OcarinaOS.png \
        ${D}${datadir}/plymouth/themes/ocarina-splash/
}

FILES:${PN} = "${datadir}/plymouth/themes/ocarina-splash \
               ${datadir}/plymouth/themes/ocarina-splash/ocarina-splash.plymouth \
               ${datadir}/plymouth/themes/ocarina-splash/ocarina-splash.script \
               ${datadir}/plymouth/themes/ocarina-splash/OcarinaOS.png"

RDEPENDS:${PN} = "plymouth plymouth-plugin-script"