#!/bin/sh
### BEGIN INIT INFO
# Provides:          wifi-connect
# Required-Start:    $local_fs
# Required-Stop:
# Default-Start:     S
# Default-Stop:
# Short-Description: Connect to saved WiFi network
### END INIT INFO

wifi_connect() {
    if ! grep -q 'ctrl_interface' /etc/wpa_supplicant.conf 2>/dev/null; then
        sed -i '1s/^/ctrl_interface=\/run\/wpa_supplicant\np2p_disabled=1\n/' /etc/wpa_supplicant.conf
    fi
    wpa_supplicant -B -i wlan0 -D nl80211 -c /etc/wpa_supplicant.conf > /dev/null 2>&1

    # wait for association before asking for a lease
    i=0
    while [ $i -lt 10 ]; do
        sleep 1
        STATE=$(wpa_cli -i wlan0 status 2>/dev/null | grep wpa_state | cut -d= -f2)
        [ "$STATE" = "COMPLETED" ] && break
        i=$((i + 1))
    done

    if [ "$STATE" != "COMPLETED" ]; then
        echo ">> WiFi association timed out, giving up"
        killall wpa_supplicant 2>/dev/null
        return 1
    fi

    dhcpcd wlan0 > /dev/null 2>&1 &
    DHCPCD_PID=$!

    # wait up to 10s for an IP
    i=0
    while [ $i -lt 10 ]; do
        sleep 1
        IP=$(ip addr show wlan0 | grep "inet " | awk '{print $2}' | cut -d/ -f1)
        if [ -n "$IP" ]; then
            echo ">> WiFi connected: $IP"
            return 0
        fi
        i=$((i + 1))
    done

    # no IP after 10s, kill everything
    echo ">> WiFi connection timed out, giving up"
    kill $DHCPCD_PID 2>/dev/null
    killall wpa_supplicant 2>/dev/null
    return 1
}

if [ -f "/etc/wpa_supplicant.conf" ]; then
    wifi_connect &
fi