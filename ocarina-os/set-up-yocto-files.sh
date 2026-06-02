echo "Cloning Poky/BitBake"

git clone -b scarthgap https://git.yoctoproject.org/poky

echo "Cloning meta-raspberrypi"

git clone -b scarthgap https://github.com/agherzan/meta-raspberrypi

echo "Cloning meta-openembedded"

git clone -b scarthgap https://github.com/openembedded/meta-openembedded.git

source poky/oe-init-build-env ~/my-build

sudo chown -R build:build ~/my-build
sudo chown -R build:build ~
sudo chown -R build:build /


echo 'MACHINE = "raspberrypi3-64"
LICENSE_FLAGS_ACCEPTED = "synaptics-killswitch"
EXTRA_IMAGE_FEATURES = "ssh-server-dropbear allow-empty-password empty-root-password allow-root-login"
IMAGE_INSTALL += " net-tools"' >> ~/my-build/conf/local.conf

echo 'BBLAYERS ?= " \
  /home/build/work/poky/meta \
  /home/build/work/poky/meta-poky \
  /home/build/work/poky/meta-yocto-bsp \
  /home/build/work/meta-raspberrypi \
  /home/build/work/meta-openembedded/meta-oe \
  /home/build/work/meta-openembedded/meta-python \
"' > ~/my-build/conf/bblayers.conf

bitbake rpi-test-image
