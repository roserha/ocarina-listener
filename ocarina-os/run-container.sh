sudo docker run --rm -it \
  --device=/dev/kvm:/dev/kvm \
  --device=/dev/net/tun:/dev/net/tun \
  --cap-add NET_ADMIN \
  --hostname buildserver \
  --user 963:962 \
  -e OCARINA_GIT_HASH="$OCARINA_GIT_HASH" \
  -v /tftpboot:/tftpboot \
  -v "$(pwd)/ocarina-os:/home/build/work" \
  -v ocarina_bitbake_cache_volume:/home/build/my-build \
  -v ocarina_downloads_volume:/home/build/data/downloads \
  yoctocontainer \
  bash