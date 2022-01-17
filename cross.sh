#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly LINK_FLAGS='-L /usr/aarch64-linux-gnu -L /usr/lib/aarch64-linux-gnu'

RUSTFLAGS=${LINK_FLAGS} cross build --release --example lipl-gatt-bluer-cli --target aarch64-unknown-linux-gnu
