SUMMARY = "Plymouth script plugin"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/GPL-2.0-only;md5=801f80980d171dd6425610833a22dbe6"

DEPENDS = "plymouth"

PLYMOUTH_VERSION = "24.004.60"
SRC_URI = "https://www.freedesktop.org/software/plymouth/releases/plymouth-${PLYMOUTH_VERSION}.tar.xz"
SRC_URI[sha256sum] = "f3f7841358c98f5e7b06a9eedbdd5e6882fd9f38bbd14a767fb083e3b55b1c34"

S = "${WORKDIR}/plymouth-${PLYMOUTH_VERSION}"

inherit meson pkgconfig

EXTRA_OEMESON = " \
    -Dgtk=disabled \
    -Ddrm=false \
    -Ddocs=false \
    -Dsystemd-integration=false \
"

do_install() {
    install -d ${D}${libdir}/plymouth
    find ${B} -name "script.so" -exec install -m 0755 {} ${D}${libdir}/plymouth/ \;
}

FILES:${PN} = "${libdir}/plymouth/script.so"
RDEPENDS:${PN} = "plymouth"