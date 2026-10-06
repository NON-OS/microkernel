# FAQ

Short answers to the questions people ask first about NONOS, each with a link to the full story.

## NONOS is not a Linux distribution

NONOS has its own kernel, a capability microkernel written in Rust, and no Linux kernel at all. Linux programs run on it through the [Linux personality](glossary.md#linux-personality), a [capsule](glossary.md#capsule) that answers their Linux syscalls. See [Architecture](architecture.md).

## Running Linux programs

The Linux personality runs x86_64 Linux programs in processes that hold no NONOS capabilities. A program read from the store runs only if the proof kept beside it verifies; the built-in BusyBox, which provides `sh`, is part of the personality's own signed image. With `linux = true` under `[store]` in `nonos.toml`, the default, the image's store carries Linux tools, python3, sqlite3 and john among them (`tools/nix/store.json`). A Linux call the personality does not serve gets ENOSYS, and the log names the call. A program started from the Terminal's `linux` command reaches no network. See [Linux programs](../using/linux-programs.md) and [Linux personality](../userland/linux-personality.md).

## Keeping files

By default nothing is kept: every boot is amnesic until the person chooses to install in first-boot setup. On a live stick, files saved during that boot live in a [data volume](glossary.md#data-volume) held in RAM, and they are gone at power off. After an install to a disk, kept files live in an encrypted data volume on that disk, keyed by the machine's TPM or by a passphrase. See [Files](../using/files.md) and [Install to disk](../install/install-to-disk.md).

## Hardware NONOS runs on

NONOS 0.9.2 builds images for x86_64 computers. Drivers are written for device classes and chips, and the [support matrix](../hardware/MATRIX.md) lists each class and chip with its PCI or USB id.

One machine has a maintainer report for these items: Wi-Fi on the Realtek RTL8821CE (PCI `10ec:c821`, from `userland/capsule_driver_rtl8821ce/src/constants/mod.rs`) for scan, join, DHCP, DNS and browser traffic; the local Qwen model offline; the Linux programs sh, python3, sqlite3 and john; the installer writing to an internal NVMe disk and booting from it; the I2C-HID touchpad on the Intel LPSS I2C controller (PCI `8086:31ac` to `8086:31ba`, even device ids only, from `userland/capsule_driver_i2c_pci/src/constants/device_info.rs`); the PS/2 keyboard with its layouts; Intel HD Audio (PCI class `0x0403`, from `userland/capsule_driver_hda/src/discover/candidate.rs`); the power button and the volume keys. Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

To add a machine of your own, see [Report a machine](../hardware/report.md).

## The network NONOS uses

The browser, the Terminal and the wallet connect through the [Nym mixnet](glossary.md#nym-mixnet) by default. The [Anyone network](glossary.md#anyone-network) and Direct are the other two choices, asked in first-boot setup and changed in Settings. If the chosen network is not running, connections fail: nothing falls back to Direct. Downloads for an install, a Qwen model or a Linux package, go over the Anyone network whatever the choice. The wallet never uses Direct for its chain reads, and the clock is set from a time server only under Direct. See [Privacy network](../using/privacy-network.md).

## Installing NONOS on a disk

You can install NONOS on an internal disk. Choose to install in first-boot setup, or pick `Install NØNOS` in the boot menu, and the installer writes to the disk you choose after you confirm. Until then NONOS writes nothing to any disk. See [Install to disk](../install/install-to-disk.md).

## Trying NONOS in a virtual machine

The `qemu` [build profile](glossary.md#build-profile) is the desktop for virtual machines, without the drivers only real hardware has (`tools/nix/config.nix`). From a checkout with Nix, the `Makefile` builds a development image, by default the development twin of the `qemu` profile, and boots it under QEMU with a software TPM attached:

```sh
make dev-image
make dev-boot
```

Not tested in this release.

A development image is sealed with throwaway keys and path-only attestation, and it is never a release. See [Get an image](../install/get-an-image.md) and [Make targets](../build/make-targets.md).

## What happens at shutdown

Shutting down or restarting from NONOS runs the [ZeroState](glossary.md#zerostate) wipe first. The kernel stops the other CPUs and every claimed device, then wipes device buffers, process memory, kernel stacks, filesystem caches, keys, its RAM log and its heap, and only then hands the machine to the firmware. A kernel panic halts the machine without the wipe, and so does cutting the power. See [Design principles](design-principles.md#amnesic-by-default).

## Without a TPM

The boot menu describes its Hardened entry as "Standard, and refuses to boot without Secure Boot and a TPM" (`nonos-bootloader/src/bootmenu/entries.rs`). Without a TPM the machine key cannot be derived, so an installed data volume needs a passphrase instead (`src/fs/blockfs_volume/passphrase.rs`), and the keyring cannot seal the wallet's record, so it answers ENOENT and nothing is saved (`userland/capsule_keyring/src/server/handlers/vault_seal.rs`). See [Boot modes](../install/boot-modes.md) and [Measured boot and the TPM](../security/measured-boot-and-tpm.md).

## Audits and proofs

This release claims no independent security audit. What it has is proof crates that test the shipped kernel and capsule source on the host, run by `nix flake check`; Lean 4 models in `verification/lean/`; and a list of everything NONOS trusts without proof, [verification/ASSUMPTIONS.md](../../verification/ASSUMPTIONS.md). [Design principles](design-principles.md#proofs-live-next-to-the-code) gives the results on this commit, failures included, and [Tests and proofs](../contributing/tests-and-proofs.md) says how to run them.

## The licence

NONOS is free software under the GNU Affero General Public License, version 3 ([LICENSE](../../LICENSE)). Each source file's header adds "or (at your option) any later version". Vendored code under `third_party/` keeps its own licence, as in `third_party/minimp3/LICENSE`, and device firmware under `nonos-bootloader/firmware/` keeps its vendor's terms, as in `nonos-bootloader/firmware/realtek/LICENSE`.

## Contributing

Start with [CONTRIBUTING.md](../../CONTRIBUTING.md) and [Contributing](../contributing/README.md). The build needs only Nix with flakes; [Build](../build/README.md) and [Make targets](../build/make-targets.md) give the commands, and [Review](../contributing/review.md) says what a change is held to.

## Reporting a security problem

Report it privately, not in a public issue. [SECURITY.md](../../SECURITY.md) and [Reporting a vulnerability](../security/reporting-a-vulnerability.md) say how.
