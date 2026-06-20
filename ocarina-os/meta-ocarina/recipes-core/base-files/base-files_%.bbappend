FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append = " file://issue \
                   file://load-modules \
                   file://aliases.sh \
                   file://motd.sh \
                   file://wifi-connect.sh"

do_install:append() {
    # create all the directories we need first so nothing breaks
    install -d ${D}${sysconfdir}/init.d
    install -d ${D}${sysconfdir}/rcS.d
    install -d ${D}${sysconfdir}/profile.d
    install -d ${D}${sysconfdir}/modules-load.d

    # add sysfs to fstab so it gets mounted on boot
    echo "sysfs                /sys                 sysfs      defaults              0  0" >> ${D}${sysconfdir}/fstab

    # install the pretty login screen and os info files
    install -m 0644 ${WORKDIR}/issue ${D}${sysconfdir}/issue
    install -m 0644 ${WORKDIR}/issue ${D}${sysconfdir}/issue.net
    
    # dynamically generate os-release to fill in distro version
    echo "ID=ocarinaos" > ${D}${sysconfdir}/os-release
    echo "NAME=\"OcarinaOS\"" >> ${D}${sysconfdir}/os-release
    echo "VERSION=\"${DISTRO_VERSION}\"" >> ${D}${sysconfdir}/os-release
    echo "PRETTY_NAME=\"OcarinaOS ${DISTRO_VERSION}\"" >> ${D}${sysconfdir}/os-release
    echo "HOME_URL=\"https://github.com/roserha/ocarina-listener\"" >> ${D}${sysconfdir}/os-release

    # set the hostname to ocarinaos
    echo "ocarinaos" > ${D}${sysconfdir}/hostname

    # add aliases we want to have
    install -m 0755 ${WORKDIR}/aliases.sh ${D}${sysconfdir}/profile.d/aliases.sh

    # install and register the modules loader so the mic and i2c communication works on boot
    install -m 0755 ${WORKDIR}/load-modules ${D}${sysconfdir}/init.d/load-modules
    ln -s ../init.d/load-modules ${D}${sysconfdir}/rcS.d/S06load-modules

    # install and register the hostname setter so it actually applies on boot
    cat > ${D}${sysconfdir}/init.d/set-hostname << 'HEOF'
#!/bin/sh
hostname -F /etc/hostname
HEOF
    chmod 0755 ${D}${sysconfdir}/init.d/set-hostname
    ln -s ../init.d/set-hostname ${D}${sysconfdir}/rcS.d/S05set-hostname

    # keep modules-load.d around just in case systemd ever gets used
    cat > ${D}${sysconfdir}/modules-load.d/i2s-audio.conf << 'EOF'
snd-soc-bcm2835-i2s
snd-soc-googlevoicehat-codec
EOF

    # install the motd script that shows system info after login
    install -m 0755 ${WORKDIR}/motd.sh ${D}${sysconfdir}/profile.d/motd.sh

    # install wifi-connect script to auto login to wifi if existent
    install -m 0755 ${WORKDIR}/wifi-connect.sh ${D}${sysconfdir}/init.d/wifi-connect
    ln -s ../init.d/wifi-connect ${D}${sysconfdir}/rcS.d/S07wifi-connect

    # fast dhcpcd config
cat > ${D}${sysconfdir}/dhcpcd.conf << 'EOF'
timeout 2
noipv4ll
noipv6rs
noipv6
EOF
}

FILES:${PN}:append = " ${sysconfdir}/profile.d/motd.sh \
                       ${sysconfdir}/profile.d/aliases.sh \
                       ${sysconfdir}/modules-load.d/i2s-audio.conf \
                       ${sysconfdir}/init.d/set-hostname \
                       ${sysconfdir}/rcS.d/S05set-hostname \
                       ${sysconfdir}/init.d/load-modules \
                       ${sysconfdir}/rcS.d/S06load-modules \
                       ${sysconfdir}/init.d/plymouth-start \
                       ${sysconfdir}/dhcpcd.conf \
                       ${sysconfdir}/init.d/wifi-connect"
