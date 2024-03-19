#!/bin/bash
set -ex
dd if=/dev/zero of=fat.img bs=1M count=2200
mformat -i fat.img -F ::
mmd -i fat.img ::/EFI
mmd -i fat.img ::/EFI/BOOT
mcopy -i fat.img target/x86_64-unknown-uefi/debug/efi-size-check.efi ::/EFI/BOOT/BOOTX64.EFI
mcopy -i fat.img esp/ABC ::
cp fat.img iso
xorriso -as mkisofs -e fat.img -no-emul-boot -o cdimage.iso iso
