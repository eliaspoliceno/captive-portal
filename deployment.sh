#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly TARGET_HOST=root@captive-portal
readonly TARGET_PATH=/opt
readonly SOURCE_PATH=./target/release/captive-portal
readonly CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnu-gcc

cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}

