# Requirements

What a machine needs to boot NONOS 0.9.2 from a USB stick, and to install it on a disk.

## Processor and firmware

- An x86_64 processor. The image carries one loader, `EFI/BOOT/BOOTX64.EFI`, and an x86_64 kernel (`tools/nonos_seal/media.py`).
- UEFI firmware. The stick is a GPT disk whose only boot path is its EFI system partition, the [ESP](../overview/glossary.md#esp). There is no legacy BIOS path.
- The NX bit, and at least 36 bits of physical address. The loader stops on a processor without them, and its reason reads `This CPU has no NX bit` or `This CPU addresses under 36 bits of physical memory` (`nonos-bootloader/src/boot/security/hardware.rs`).
- A hardware random source. Every boot entry needs one for the boot's keys, and the menu lists `HARDWARE RNG` as missing when none answers (`nonos-bootloader/src/bootmenu/ready.rs`). The loader's own advice is to turn on RDRAND or the TPM in the firmware, or to give a virtual machine a virtio-rng device (`nonos-bootloader/src/display/boot/refusal/platform.rs`).

## Secure Boot and the TPM

What the firmware must have on depends on the boot entry you pick and on how the image was built.

| Entry | Secure Boot | TPM 2.0 |
|---|---|---|
| Standard, Safe Mode, Recovery, Install | not needed | optional |
| Air-Gapped | not needed | needed, for the rollback floor |
| Hardened | on, with a platform key (PK) and a signature database (db) | needed |

These are the floors of an image built with the `standard`, `qemu` or `core` [build profile](../overview/glossary.md#build-profile). An image built `hardened` or `airgapped` uses the loader's production policy: every entry then needs [Secure Boot](../overview/glossary.md#secure-boot) and a [TPM](../overview/glossary.md#tpm), because the menu can raise the build's floor and never lower it (`nonos-bootloader/src/bootmenu/ready.rs`, `tools/nix/config.nix`). The menu shows that floor as `BUILD FLOOR`, beside the Secure Boot and TPM state, before you choose.

When the NONOS db key is present, the seal signs `BOOTX64.EFI` for Secure Boot with it and with no other key; without it the loader is not signed for Secure Boot at all, and a `--release` seal of a `hardened` or `airgapped` image stops (`tools/nonos_seal/chain.py`). Firmware with Secure Boot on starts a loader only when its own db trusts the signature. Unless the NONOS db certificate is enrolled in your firmware, turn Secure Boot off and boot Standard. This release documents no procedure for that enrollment, and booting with Secure Boot on is not tested in this release.

A Standard boot runs without a TPM, with three things missing:

- No rollback floor. The menu shows `ROLLBACK` with `NO COUNTER`, the loader notes `No TPM: rollback protection is off`, and an older signed kernel would boot (`nonos-bootloader/src/boot/crypto/rollback/floor.rs`).
- No key for an installed disk's [data volume](../overview/glossary.md#data-volume). The kernel derives that key from the TPM, and without one it leaves the volume closed (`src/fs/blockfs_volume/open_machine.rs`).
- No remembered Wi-Fi networks. A remembered passphrase is sealed with ChaCha20-Poly1305 under a key the TPM derives, and is never written in the clear (`userland/nonos_wifi_client/src/saved/file.rs`, `userland/nonos_wifi_client/src/saved/key.rs`). Without a TPM, remembering a network fails with `No TPM to seal the passphrase with`.

## Memory

This release sets no minimum amount of memory: the loader's hardware check names only NX and the physical address width (`nonos-bootloader/src/boot/security/hardware.rs`). What memory decides is what runs:

- On a boot from the stick, the session's data volume is held in RAM, and only when at least 256 MiB is free beyond what the kernel keeps for the system, which is the larger of 1 GiB and a quarter of memory (`src/fs/blockfs_volume/session.rs`, `src/fs/cryptoblock/ram.rs`). Below that, that boot has no `/data`, and programs that ask for it get `too little memory free for a volume in RAM`.
- The Qwen model step in setup offers only the tiers that fit this machine's memory ([Local AI](../using/local-ai.md)).
- The QEMU boots give the virtual machine 8 GiB by default (`--mem 8G`), and the boot tool's help says Qwen, Linux programs and a second window need more than 2G (`tools/nonos_qemu/__main__.py`).

## USB stick

The sealed image `nonos.img` is 1,043,148,800 bytes, about 995 MiB:

| Part | Size |
|---|---|
| Partition table, package store, disk plan and ESP | 384 MiB |
| The Qwen3 0.6B model file, laid past the ESP | 639,446,688 bytes, rounded up to whole sectors |
| Room for the backup partition table | 1 MiB |

The sizes are `USB_MB`, `STICK_TIER` and `GPT_TAIL` in `tools/nonos_seal/media.py`, and the pinned file size in `userland/capsule_linux/src/linux/file/models/pinned_qwen3.rs`. Use a stick of 2 GB or more. Writing the image erases everything on the stick.
