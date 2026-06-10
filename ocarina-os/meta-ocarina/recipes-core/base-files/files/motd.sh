#!/bin/sh
IP=$(ip addr show eth0 2>/dev/null | grep "inet " | awk '{print $2}' | cut -d/ -f1)
[ -z "$IP" ] && IP=$(ip addr show wlan0 2>/dev/null | grep "inet " | awk '{print $2}' | cut -d/ -f1)
UPTIME=$(uptime | awk '{print $3, $4}' | sed 's/,//')
DISK=$(df -h / | awk 'NR==2 {print $4}')
MEM=$(free | awk 'NR==2 {printf "%.0fMB/%.0fMB", $3/1024, $2/1024}')
TEMP=$(vcgencmd measure_temp 2>/dev/null | cut -d= -f2)
VERSION=$(. /etc/os-release && echo $VERSION)
STATUS=$(pgrep ocarina-listener >/dev/null && echo "Running 🟢" || echo "Stopped 🔴")

# attempt to connect to saved wifi
if [ -f "/etc/wpa_supplicant.conf" ]; then
  echo ">> Connecting to previously saved network '$(cat /etc/wpa_supplicant.conf | grep "ssid" | cut -d '"' -f2)'..."
  while [ -z "$IP" ] && [ $i -lt 10 ]; do
    IP=$(ip addr show wlan0 2>/dev/null | grep "inet " | awk '{print $2}' | cut -d/ -f1)
    sleep 1
    i=$((i + 1))
  done
fi

# if no ip detected, offer to connect to wifi
if [ -z "$IP" ]; then
  echo ">> No network connection detected."
  printf ">> Would you like to connect to WiFi? [y/N] "
  read REPLY
  if [ "$REPLY" = "y" ] || [ "$REPLY" = "Y" ]; then
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
    IP=$(ip addr show wlan0 2>/dev/null | grep "inet " | awk '{print $2}' | cut -d/ -f1)
    echo "" && echo ""
  fi
fi

cat <<'LOGO'
 .88888.                             oo                    .88888.  .d88888b  
d8'   `8b                                                 d8'   `8b 88.    "' 
88     88 .d8888b. .d8888b. 88d888b. dP 88d888b. .d8888b. 88     88 `Y88888b. 
88     88 88'  `"" 88'  `88 88'  `88 88 88'  `88 88'  `88 88     88       `8b 
Y8.   .8P 88.  ... 88.  .88 88       88 88    88 88.  .88 Y8.   .8P d8'   .8P 
 `8888P'  `88888P' `88888P8 dP       dP dP    dP `88888P8  `8888P'   Y88888P  
==============================================================================
LOGO
echo ""
echo "  OcarinaOS $VERSION - (c) Rosalyn May Rodrigues"
echo "  Welcome, $USER!~ 🎵"
echo ""
echo "  IP Address : ${IP:-not connected}"
echo "  Uptime     : ${UPTIME}"
echo "  Disk Free  : ${DISK}"
echo "  Memory     : ${MEM}"
echo "  CPU Temp   : ${TEMP:-unavailable}"
echo "  Listener   : ${STATUS}"
echo ""
echo "=============================================================================="
echo ""
