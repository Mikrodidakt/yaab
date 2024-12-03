#!/bin/sh
#
#
set -e
YAAB_VERSION=$1
VARIANT=$2

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. ${SCRIPT_DIR}/lib.sh

if [ ! -n "$YAAB_VERSION" ]; then
	echo "ERROR: no version specified!"
	exit 1
fi

check_variant ${VARIANT}

wget https://github.com/Mikrodidakt/yaab/releases/download/v${YAAB_VERSION}/yaab-x86_64-${VARIANT}-v${YAAB_VERSION}.deb
sudo dpkg -i yaab-x86_64-${VARIANT}-v${YAAB_VERSION}.deb
