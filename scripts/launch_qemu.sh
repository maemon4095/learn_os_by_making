#!/bin/bash -e
cd "$WORKSPACE"

PATH_TO_EFI=$1

rm -rf .cache/mnt
mkdir -p .cache/mnt/EFI/BOOT
cp ${PATH_TO_EFI} .cache/mnt/EFI/BOOT/BOOTX64.EFI

cd ".cache"

qemu-system-x86_64 \
    -m 4G \
    -bios $WORKSPACE/third_party/ovmf/RELEASEX64_OVMF.fd \
    -drive format=raw,file=fat:rw:mnt \
    -device isa-debug-exit,iobase=0xf4,iosize=0x01