SUMMARY = "Plymouth script plugin"
DESCRIPTION = "Script plugin for plymouth boot splash"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://COPYING;md5=a6f89e2100d9b6cdffcea4f398e37343"

DEPENDS = "plymouth"

SRC_URI = "https://www.freedesktop.org/software/plymouth/releases/plymouth-${PV}.tar.xz"
# find current sum by running wget https://www.freedesktop.org/software/plymouth/releases/plymouth-24.004.60.tar.xz && sha256sum plymouth-24.004.60.tar.xz
SRC_URI[sha256sum] = "f3f7841358c98f5e7b06a9eedbdd5e6882fd9f38bbd14a767fb083e3b55b1c34"

S = "${WORKDIR}/plymouth-${PV}"

inherit meson pkgconfig

# only build the script plugin, nothing else
EXTRA_OEMESON = " \
    -Dgtk=disabled \
    -Dlogo=disabled \
    -Ddrm=disabled \
    -Dsystemd-integration=false \
    -Dplugindir=${libdir}/plymouth \
"

do_install() {
    install -d ${D}${libdir}/plymouth
    install -m 0755 ${B}/src/plugins/renderers/script/.libs/script.so \
        ${D}${libdir}/plymouth/script.so
}

FILES:${PN} = "${libdir}/plymouth/script.so"

RDEPENDS:${PN} = "plymouth"