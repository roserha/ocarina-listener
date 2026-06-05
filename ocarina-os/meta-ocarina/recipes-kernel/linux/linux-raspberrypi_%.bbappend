FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append = " file://googlevoicehat-soundcard.dtbo"

IMAGE_BOOT_FILES:append = " googlevoicehat-soundcard.dtbo;overlays/googlevoicehat-soundcard.dtbo"
