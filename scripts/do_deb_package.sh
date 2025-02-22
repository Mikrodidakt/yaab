#!/bin/sh
#
set -e

VARIANT=$1

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. ${SCRIPT_DIR}/lib.sh
VERSION=$(get_yaab_version ${WORK_DIR}/Cargo.toml)
TEMP_WORK_DIR=$(mktemp -d --suffix=-yaab-deb)

if [ ! -n "${VARIANT}" ]; then
    VARIANT=glibc
fi

check_variant ${VARIANT}

mkdir -p ${TEMP_WORK_DIR}/yaab
TEMP_WORK_DIR=${TEMP_WORK_DIR}/yaab
mkdir -p ${TEMP_WORK_DIR}/usr/bin
mkdir -p ${TEMP_WORK_DIR}/etc/yaab
cp ${ARTIFACTS_DIR}/yaab ${TEMP_WORK_DIR}/usr/bin/
cp ${SCRIPT_DIR}/yaab.bashrc ${TEMP_WORK_DIR}/etc/yaab/yaab.bashrc

mkdir -p ${TEMP_WORK_DIR}/DEBIAN
touch ${TEMP_WORK_DIR}/DEBIAN/control
cat <<EOT >> ${TEMP_WORK_DIR}/DEBIAN/control
Package: yaab
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: all
Maintainer: Mans <mans.zigher@mikro.io>
Description: Build engine for Android
EOT

dpkg-deb --root-owner-group --build ${TEMP_WORK_DIR}

cp ${TEMP_WORK_DIR}/../yaab.deb ${ARTIFACTS_DIR}/yaab-x86_64-${VARIANT}-v${VERSION}.deb
(cd ${ARTIFACTS_DIR}; ln -sf yaab-x86_64-${VARIANT}-v${VERSION}.deb yaab.deb && ln -sf yaab-x86_64-${VARIANT}-v${VERSION}.deb yaab-x86_64-${VARIANT}.deb)

