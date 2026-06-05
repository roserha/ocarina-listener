#!/bin/sh
alias whatsmyip='ip addr show wlan0 | grep "inet " | awk '"'"'{print $2}'"'"' | cut -d/ -f1'

changewifi() {
  printf ">> SSID: "
  read SSID
  printf ">>  Password: "
  read -s PSK
  echo ""
  wpa_passphrase "$SSID" "$PSK" >/etc/wpa_supplicant.conf
  echo ">> Connecting..."
  wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant.conf >/dev/null 2>&1
  dhcpcd wlan0 >/dev/null 2>&1
  sleep 3
}
