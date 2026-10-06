# Requirements

What a machine needs to boot NONOS 0.9.2 from a USB stick, and to install it on a disk.

## Processor and firmware

- An x86_64 processor. The image carries one loader, `EFI/BOOT/BOOTX64.EFI`, and an x86_64 kernel (`tools/nonos_seal/media.py`).
- UEFI firmware. The stick is a GPT disk whose only boot path is its EFI system partition, the [ESP](../overview/glossary.md#esp). There is no legacy BIOS path.
- The NX bit, and at least 36 bits of physical address. The loader stops on a processor without them, and its reason reads `This CPU has no NX bit` or `This CPU addresses under 36 bits of physical memory` (`nonos-bootloader/src/boot/security/hardware.rs`).
- A hardware random source. Every boot entry needs one for the boot's keys, and the menu lists `HARDWARE RNG` as missing when none answers (`nonos-bootloader/src/bootmenu/ready.rs`). The loader's own advice is to turn on RDRAND or the TPM in the firmware, or to give a virtual machine a virtio-rng device (`nonos-bootloader/src/display/boot/refusal/platform.rs`).
