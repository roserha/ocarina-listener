#!/bin/sh
### BEGIN INIT INFO
# Provides:          wifi-connect
# Required-Start:    $local_fs
# Required-Stop:
# Default-Start:     S
# Default-Stop:
# Short-Description: Connect to saved WiFi network
### END INIT INFO

if [ -f "/etc/wpa_supplicant.conf" ]; then
  wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant.conf >/dev/null 2>&1
  dhcpcd wlan0 >/dev/null 2>&1
fi
