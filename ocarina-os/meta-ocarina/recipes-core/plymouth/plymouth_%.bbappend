do_install:append() {
    cat > ${D}${sysconfdir}/plymouth/plymouthd.conf << 'EOF'
[Daemon]
Theme=ocarina-splash
ShowDelay=0
EOF
}