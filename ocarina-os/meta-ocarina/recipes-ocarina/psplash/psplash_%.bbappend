FILESEXTRAPATHS:prepend := "${THISDIR}/files:"
SRC_URI:append = " file://OcarinaOS.png"

do_compile:append() {
    ${S}/make_header.sh ${WORKDIR}/OcarinaOS.png > ${S}/psplash-poky-img.h
}