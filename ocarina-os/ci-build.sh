#!/bin/bash
set -e

# Make sure we're running this in the right place!

cd /home/build/work

# Preparing repositories
echo "Preparing repositories"

[ -d poky ] || git clone -b scarthgap https://git.yoctoproject.org/poky
[ -d meta-raspberrypi ] || git clone -b scarthgap https://github.com/agherzan/meta-raspberrypi
[ -d meta-openembedded ] || git clone -b scarthgap https://github.com/openembedded/meta-openembedded.git

# Set up source

echo "Setting up BB source"
source ~/work/poky/oe-init-build-env ~/my-build

# Copying files

echo "Copying BB configuration files"
cp ~/work/conf-files/bblayers.conf ~/my-build/conf/bblayers.conf
cp ~/work/conf-files/local.conf ~/my-build/conf/local.conf

# disable every single post install intercept since qemu can't handle it for now i guess

echo "Disabling poky post-installaton intercept scripts"
for script in /home/build/work/poky/scripts/postinst-intercepts/*; do
    echo '#!/bin/sh' > "$script"
    echo 'exit 0' >> "$script"
    chmod +x "$script"
done

# Actually build image

echo "Cleaning bitbake image"
bitbake core-image-base -c cleanall
echo "Building image"
bitbake core-image-base

# Export image

echo "Exporting images"
rm ~/work/core-image-base-raspberrypi*-64.rootfs-*.wic.* || true
cp ~/my-build/tmp/deploy/images/raspberrypi*/core-image-base-raspberrypi*-64.rootfs-*.wic.* ~/work/

# Also package it for archival purposes!

echo "Tarring image"
rm ~/work/*.tar.xz || true
VERSION=$(grep DISTRO_VERSION /home/build/work/meta-ocarina/conf/distro/ocarinaos.conf | cut -d'"' -f2 | tr '.' '-')
tar -cJf /home/build/work/OcarinaOSv${VERSION}.tar.xz \
    /home/build/work/core-image-base-raspberrypi3-64.rootfs-*.wic.bmap \
    /home/build/work/core-image-base-raspberrypi3-64.rootfs-*.wic.bz2