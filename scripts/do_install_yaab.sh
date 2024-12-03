#!/bin/sh
#

VARIANT=$1

if [ ! -n "${VARIANT}" ]; then
    VARIANT=glibc
fi

check_variant ${VARIANT}

YAAB_VERSION=$1
TEMP_WORK_DIR=$(mktemp -d --suffix=-yaab-deb)
(cd ${TEMP_WORK_DIR}; wget https://github.com/Mikrodidakt/yaab/releases/download/v${YAAB_VERSION}/yaab-x86_64-${VARIANT}-v${YAAB_VERSION}.deb)
sudo dpkg -i ${TEMP_WORK_DIR}/yaab-x86_64-${VARIANT}-v${YAAB_VERSION}.deb
