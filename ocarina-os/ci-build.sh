#!/bin/bash
set -e

# Preparing repositories

echo "Cloning Poky/BitBake"

[ -d poky ] git clone -b scarthgap https://git.yoctoproject.org/poky

echo "Cloning meta-raspberrypi"

[ -d meta-raspberrypi ] git clone -b scarthgap https://github.com/agherzan/meta-raspberrypi

echo "Cloning meta-openembedded"

[ -d meta-openembedded ] git clone -b scarthgap https://github.com/openembedded/meta-openembedded.git

# Set up source

source ~/work/poky/oe-init-build-env ~/my-build

# Copying files

sudo cp ~/work/conf-files/bblayers.conf ~/my-build/conf/bblayers.conf
sudo cp ~/work/conf-files/local.conf ~/my-build/conf/local.conf

# Actually build image

bitbake core-image-base -c cleanall
bitbake core-image-base

# Export image

sudo rm ~/work/core-image-base-raspberrypi*-64.rootfs-*.wic.*
sudo cp ~/my-build/tmp/deploy/images/raspberrypi*/core-image-base-raspberrypi*-64.rootfs-*.wic.* ~/work/