#!/bin/bash
set -e

# Make sure we're running this in the right place!

cd /home/build/work

# Preparing repositories

[ -d poky ] || git clone -b scarthgap https://git.yoctoproject.org/poky
[ -d meta-raspberrypi ] || git clone -b scarthgap https://github.com/agherzan/meta-raspberrypi
[ -d meta-openembedded ] || git clone -b scarthgap https://github.com/openembedded/meta-openembedded.git

# Set up source

source ~/work/poky/oe-init-build-env ~/my-build

# Copying files

cp ~/work/conf-files/bblayers.conf ~/my-build/conf/bblayers.conf
cp ~/work/conf-files/local.conf ~/my-build/conf/local.conf

# Actually build image

bitbake core-image-base -c cleanall
bitbake core-image-base

# Export image

rm ~/work/core-image-base-raspberrypi*-64.rootfs-*.wic.*
cp ~/my-build/tmp/deploy/images/raspberrypi*/core-image-base-raspberrypi*-64.rootfs-*.wic.* ~/work/