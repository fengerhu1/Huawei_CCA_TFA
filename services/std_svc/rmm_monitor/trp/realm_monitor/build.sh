#!/bin/sh

if [ "$1" = "qemu" ]; then
    cargo xbuild --release --target=aarch64-unknown-none-softfloat --features platform_qemu
elif [ "$1" = "fvp" ]; then
    cargo xbuild --release --target=aarch64-unknown-none-softfloat --features platform_fvp
else
    echo "Usage: $0 <qemu|fvp>"
fi