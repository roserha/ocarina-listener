if [[ -z $1 ]]; then
  echo "Please type in version name as M-m-p."
  exit
fi

tar -cJf OcarinaOSv$1.tar.xz ./ocarina-os/core-image-base-raspberrypi0-2w-64.rootfs-*.wic.bmap ./ocarina-os/core-image-base-raspberrypi0-2w-64.rootfs-*.wic.bz2
