#!/bin/sh
#
set -e

TARGET=$1

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. $SCRIPT_DIR/lib.sh
VERSION=$(get_yaab_version ${WORK_DIR}/Cargo.toml)

if [ ! -n "${TARGET}" ]; then
    TARGET=glibc
fi

check_target ${TARGET}

echo "INFO: build yaab v${VERSION} for ${TARGET}"

if [ "${TARGET}" = "glibc" ]; then
    (cd ${WORK_DIR}; cargo build --release)
    cp ${WORK_DIR}/target/release/yaab ${ARTIFACTS_DIR}
elif [ "${TARGET}" = "musl" ]; then
    (cd ${WORK_DIR}; cargo build --target x86_64-unknown-linux-musl --release)
    cp ${WORK_DIR}/target/x86_64-unknown-linux-musl/release/yaab ${ARTIFACTS_DIR}
fi
