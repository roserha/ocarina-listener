#!/bin/sh
alias whatsmyip='ip addr show wlan0 | grep "inet " | awk '"'"'{print $2}'"'"' | cut -d/ -f1'

ensure_wpa_conf_headers() {
    if ! grep -q 'ctrl_interface' /etc/wpa_supplicant.conf 2>/dev/null; then
        sed -i '1s/^/ctrl_interface=\/run\/wpa_supplicant\np2p_disabled=1\n/' /etc/wpa_supplicant.conf
    fi
}

changewifi() {
    if [ -f /etc/wpa_supplicant.conf ]; then
        CURRENT_SSID=$(grep 'ssid=' /etc/wpa_supplicant.conf | cut -d'"' -f2)
    fi

    printf ">> SSID [${CURRENT_SSID}]: "
    read SSID
    [ -z "$SSID" ] && SSID="$CURRENT_SSID"

    printf ">> Password (leave blank to keep existing): "
    read -s PSK
    echo ""

    if [ -z "$PSK" ] && [ "$SSID" = "$CURRENT_SSID" ]; then
        echo ">> Reconnecting with existing config..."
    else
        wpa_passphrase "$SSID" "$PSK" > /etc/wpa_supplicant.conf
        sed -i '1s/^/ctrl_interface=\/run\/wpa_supplicant\np2p_disabled=1\n/' /etc/wpa_supplicant.conf
    fi

    killall wpa_supplicant 2>/dev/null
    killall dhcpcd 2>/dev/null
    sleep 0.5
    ip link set wlan0 down
    ip link set wlan0 up
    ensure_wpa_conf_headers
    wpa_supplicant -B -i wlan0 -D nl80211 -c /etc/wpa_supplicant.conf > /dev/null 2>&1

    echo ">> Waiting for association..."
    i=0
    while [ $i -lt 10 ]; do
        sleep 1
        STATE=$(wpa_cli -i wlan0 status 2>/dev/null | grep wpa_state | cut -d= -f2)
        [ "$STATE" = "COMPLETED" ] && break
        i=$((i + 1))
    done

    if [ "$STATE" != "COMPLETED" ]; then
        echo ">> Failed to associate, check SSID/password"
        return 1
    fi

    dhcpcd wlan0 > /dev/null 2>&1 &
    echo ">> Connecting in background, run 'whatsmyip' to check status"
}