do_install:append() {
    cat > ${D}${sysconfdir}/dhcpcd.conf << 'EOF'
timeout 5
noipv4ll
noipv6rs
noipv6
EOF
}