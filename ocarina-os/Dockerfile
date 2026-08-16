FROM ubuntu:22.04
 
ARG DEBIAN_FRONTEND=noninteractive
 
# Match these to the host user that owns the Jenkins workspace so that
# bind-mounted files are writable from inside the container.
#   docker build --build-arg BUILD_UID=$(id -u) --build-arg BUILD_GID=$(id -g) .
ARG BUILD_UID=963
ARG BUILD_GID=962
 
RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -yq \
        apt-utils bash bc build-essential chrpath cpio curl diffstat \
        doxygen desktop-file-utils dos2unix file g++-multilib gawk git \
        iproute2 iputils-ping libacl1 libc6-dev-i386 libncurses5-dev \
        libncursesw5 libncursesw5:i386 libssl-dev libssl-dev:i386 \
        libtinfo5 libtool libtool-bin locales lz4 lzop man nano net-tools \
        pigz procps python3 python3-distutils python3-git python3-jinja2 \
        python3-pexpect python3-pip python3-subunit python3-yaml \
        qemu-user qemu-user-static rsync screen socat sudo texinfo \
        u-boot-tools unzip vim wget xterm xvfb x11-utils xz-utils zip \
        zlib1g-dev:i386 zstd && \
    rm -rf /var/lib/apt/lists/* && \
    echo "dash dash/sh boolean false" | debconf-set-selections && \
    dpkg-reconfigure dash
 
# Google's repo tool
RUN curl -fsSL https://storage.googleapis.com/git-repo-downloads/repo -o /bin/repo && \
    chmod a+x /bin/repo && \
    sed -i "1s/python/python3/" /bin/repo
 
RUN groupadd build -g ${BUILD_GID} && \
    useradd -ms /bin/bash -u ${BUILD_UID} -g ${BUILD_GID} build && \
    usermod -aG sudo build && \
    echo "build ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/build && \
    chmod 0440 /etc/sudoers.d/build
 
RUN ln -sf /usr/bin/update-desktop-database /usr/local/bin/update-desktop-database
 
# Create every mount point BEFORE declaring volumes, so Docker inherits
# build:build ownership instead of creating them as root.
RUN mkdir -p /home/build/my-build \
             /home/build/data/downloads \
             /home/build/work && \
    chown -R build:build /home/build
 
VOLUME ["/home/build/my-build", "/home/build/data/downloads"]
 
RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen
ENV LANG=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8
 
USER build
WORKDIR /home/build
 
RUN git config --global user.email "build@example.com" && \
    git config --global user.name "Build" && \
    git config --global --add safe.directory '*'
