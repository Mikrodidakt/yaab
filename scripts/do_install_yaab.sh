#!/bin/sh
#

TARGET=$1

if [ ! -n "${TARGET}" ]; then
    TARGET=glibc
fi

check_target ${TARGET}

YAAB_VERSION=$1
TEMP_WORK_DIR=$(mktemp -d --suffix=-yaab-deb)
(cd ${TEMP_WORK_DIR}; wget https://github.com/Mikrodidakt/yaab/releases/download/v${YAAB_VERSION}/yaab-x86_64-${TARGET}-v${YAAB_VERSION}.deb)
sudo dpkg -i ${TEMP_WORK_DIR}/yaab-x86_64-${TARGET}-v${YAAB_VERSION}.deb
