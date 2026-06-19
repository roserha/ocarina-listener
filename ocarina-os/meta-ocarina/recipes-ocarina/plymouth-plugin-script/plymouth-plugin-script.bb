SUMMARY = "Plymouth script plugin"
DESCRIPTION = "Script plugin for plymouth boot splash"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/GPL-2.0-only;md5=801f80980d171dd6425610833a22dbe6"

DEPENDS = "plymouth"

PLYMOUTH_VERSION = "24.004.60"

SRC_URI = "https://www.freedesktop.org/software/plymouth/releases/plymouth-${PLYMOUTH_VERSION}.tar.xz"
# find current sum by running wget https://www.freedesktop.org/software/plymouth/releases/plymouth-24.004.60.tar.xz && sha256sum plymouth-24.004.60.tar.xz
SRC_URI[sha256sum] = "f3f7841358c98f5e7b06a9eedbdd5e6882fd9f38bbd14a767fb083e3b55b1c34"

S = "${WORKDIR}/plymouth-${PLYMOUTH_VERSION}"

inherit meson pkgconfig

# only build the script plugin, nothing else
EXTRA_OEMESON = " \
    -Dgtk=disabled \
    -Ddrm=false \
    -Ddocs=false \
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