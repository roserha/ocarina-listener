sudo docker run \
  --device=/dev/kvm:/dev/kvm \
  --device=/dev/net/tun:/dev/net/tun \
  --cap-add NET_ADMIN \
  --hostname buildserver \
  -it \
  -v /tftpboot:/tftpboot \
  -v $(pwd):/home/build/work \
  -v ocarina_bitbake_cache_volume:/home/build/my-build \
  yoctocontainer
