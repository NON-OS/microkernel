# FAQ

Short answers to the questions people ask first about NONOS, each with a link to the full story.

## NONOS is not a Linux distribution

NONOS has its own kernel, a capability microkernel written in Rust, and no Linux kernel at all. Linux programs run on it through the [Linux personality](glossary.md#linux-personality), a [capsule](glossary.md#capsule) that answers their Linux syscalls. See [Architecture](architecture.md).

## Running Linux programs

The Linux personality runs x86_64 Linux programs in processes that hold no NONOS capabilities. A program read from the store runs only if the proof kept beside it verifies; the built-in BusyBox, which provides `sh`, is part of the personality's own signed image. With `linux = true` under `[store]` in `nonos.toml`, the default, the image's store carries Linux tools, python3, sqlite3 and john among them. A Linux call the personality does not serve gets ENOSYS, and the log names the call. A program started from the Terminal's `linux` command reaches no network. See [Linux programs](../using/linux-programs.md) and [Linux personality](../userland/linux-personality.md).

The Terminal also runs seven programs from crates.io, `tokei` and `csview` among them, each built as its own signed NONOS capsule that needs no Linux personality. See [Command-line tools](../using/command-line-tools.md).

## Keeping files

By default nothing is kept: every boot is [amnesic](glossary.md#amnesic-boot) until the person chooses to install. Files saved during a boot live in memory, in the [file store](glossary.md#file-store), and are gone at power off. On a live stick the [data volume](glossary.md#data-volume) that holds Qwen models is in RAM as well. After an install to a disk, a file a program asks to keep goes to the [package store](glossary.md#store) on that disk, which is not encrypted, and Qwen models go to an encrypted data volume keyed by the machine's [TPM](glossary.md#tpm). Files made in Files or Editor are never kept, and a Linux package is held in memory until restart. The kernel can also key a volume with a passphrase, but nothing in this release asks for one. See [Files](../using/files.md) and [Install to disk](../install/install-to-disk.md).

## Hardware NONOS runs on

NONOS 0.9.2 builds images for x86_64 computers, and it runs on every core of the processor: the kernel starts every CPU the firmware enables, up to 256, and schedules processes on all of them. Drivers are written for device classes and chips, and the [support matrix](../hardware/MATRIX.md) lists each class and chip with its PCI or USB id.

On Intel VT-d machines the [IOMMU](glossary.md#iommu) confines device DMA: each PCI device a driver claims reaches only the buffers granted to that driver. AMD-Vi is not driven and interrupt remapping is off in this release, so on an AMD machine device DMA is unrestricted, and the boot log says so. See [IOMMU](../kernel/iommu.md).

One machine has a maintainer report for these items: Wi-Fi on the Realtek RTL8821CE (PCI `10ec:c821`) for scan, join, DHCP, DNS and browser traffic; the local Qwen model offline; the Linux programs sh, python3, sqlite3 and john; the installer writing to an internal NVMe disk and booting from it; the I2C-HID touchpad on the Intel LPSS I2C controller (PCI `8086:31ac` to `8086:31ba`, even device ids only); the PS/2 keyboard with its layouts; Intel HD Audio (PCI class `0x0403`, or `0x0401` on an Intel controller that is not an SST engine); the power button and the volume keys. Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

To add a machine of your own, see [Report a machine](../hardware/report.md).

## The network NONOS uses

The browser, the Terminal and the wallet connect through the [Nym mixnet](glossary.md#nym-mixnet) by default. The [Anyone network](glossary.md#anyone-network) and Direct are the other two choices, asked in first-boot setup and changed in Settings. If the chosen network is not running, connections fail: nothing falls back to Direct. Downloads for an install, a Qwen model or a Linux package, go over the Anyone network whatever the choice; a model goes direct only when you ask for that one download, with `qwen get --direct` or `d` on its Marketplace card. The wallet never uses Direct for its chain reads, and no image in this release asks a time server. See [Privacy network](../using/privacy-network.md).

## Installing NONOS on a disk

You can install NONOS on an internal disk. Choose to install in first-boot setup, or pick `Install NØNOS` in the boot menu, and the installer writes to the disk you choose after you confirm. Until then NONOS writes nothing to any disk, except that choosing Install in setup keeps your answers in the stick's own store. See [Install to disk](../install/install-to-disk.md).

## Trying NONOS in a virtual machine

The `qemu` [build profile](glossary.md#build-profile) is the desktop for virtual machines. It carries 11 of the 18 driver capsules and leaves out the Wi-Fi drivers, the Intel and Realtek Ethernet drivers and the I2C drivers. From a checkout with Nix, the `Makefile` builds a development image, by default the development twin of the `qemu` profile, and boots it under QEMU with a software TPM attached:

```sh
make dev-image
make dev-boot
```

Not tested in this release.

A [development image](glossary.md#development-image) is sealed with throwaway keys and path-only attestation, and it is never a release. See [Get an image](../install/get-an-image.md) and [Make targets](../build/make-targets.md).

## What happens at shutdown

In 0.9.2 the desktop cannot shut down or restart the machine: the Power key shows `Power off is not available from the desktop`. To turn the machine off, force it off with its power button, as the [release notes](../release/0.9.2.md) describe. A forced power-off skips the wipe below, as a power cut or a kernel panic does.

The one orderly path is the installer's restart, and it runs the [ZeroState](glossary.md#zerostate) wipe first. The kernel stops the other CPUs and every claimed device, then wipes device buffers, process memory, kernel stacks, filesystem caches, its key vault, its RAM log and its heap, and only then hands the machine to the firmware. Kernel statics outside the heap, the data volume key among them, are not wiped, and neither is a live stick's in-memory volume. See [Design principles](design-principles.md#amnesic-by-default).

## Without a TPM

The boot menu describes its Hardened entry as "Standard, and refuses to boot without Secure Boot and a TPM". Air-Gapped refuses to boot without one too, because the TPM holds the [rollback floor](glossary.md#rollback-floor); every other entry boots with rollback protection off, so an older signed kernel would boot. Without a TPM the [machine key](glossary.md#machine-key) cannot be derived, so an installed data volume stays closed. The kernel could key a volume with a passphrase instead, but no capsule in this release asks for one. Without a TPM the keyring cannot seal the wallet's record, so it answers ENOENT and nothing is saved. See [Boot modes](../install/boot-modes.md) and [Measured boot and the TPM](../security/measured-boot-and-tpm.md).

## Audits and proofs

This release claims no independent security audit. What it has is [proof crates](glossary.md#proof-crate) that test the shipped kernel and capsule source on the host, run by `nix flake check`; Lean 4 models in `verification/lean/`; and a list of everything NONOS trusts without proof, [verification/ASSUMPTIONS.md](../../verification/ASSUMPTIONS.md). [Design principles](design-principles.md#proofs-live-next-to-the-code) gives the results on this commit, failures included, and [Tests and proofs](../contributing/tests-and-proofs.md) says how to run them.

## The licence

NONOS is free software under the GNU Affero General Public License, version 3 ([LICENSE](../../LICENSE)). Each source file's header adds "or (at your option) any later version". Vendored code under `third_party/` keeps its own licence, as in `third_party/minimp3/LICENSE`, and device firmware under `nonos-bootloader/firmware/` keeps its vendor's terms, as in `nonos-bootloader/firmware/realtek/LICENSE`.

## Contributing

Start with [CONTRIBUTING.md](../../CONTRIBUTING.md) and [Contributing](../contributing/README.md). The build needs Nix with flakes turned on, plus `git` and GNU `make`; every compiler and tool comes from the flake ([Toolchain](../build/toolchain.md)). [Build](../build/README.md) and [Make targets](../build/make-targets.md) give the commands, and [Review](../contributing/review.md) says what a change is held to. To write your first program, [Writing an app](../userland/writing-an-app.md) builds a windowed app step by step.

## Reporting a security problem

Report it privately, not in a public issue. [SECURITY.md](../../SECURITY.md) and [Reporting a vulnerability](../security/reporting-a-vulnerability.md) say how.

## Where this comes from

The source behind the answers above, at the commit in the footer.

- Running Linux programs
  - The store's Linux tools: `linux` in `tools/nix/store.json:100`.
- Hardware NONOS runs on
  - The RTL8821CE id: `PCI_DEVICE_RTL8821CE` in `userland/capsule_driver_rtl8821ce/src/constants/mod.rs:26`.
  - The Intel LPSS I2C ids, even ones only: `device_info` in `userland/capsule_driver_i2c_pci/src/constants/device_info.rs:29-38`.
  - The HD Audio classes and the SST exception: `is_candidate` in `userland/capsule_driver_hda/src/discover/candidate.rs:24-34`.
- Trying NONOS in a virtual machine
  - What each profile carries: `profiles` in `tools/nix/config.nix:69-117`.
- What happens at shutdown
  - The Power key notice: `POWER_OFF_UNAVAILABLE` in `userland/capsule_desktop_shell/src/state/system_key.rs:42-47`.
- Without a TPM
  - The Hardened entry's text: `Hardened` in `nonos-bootloader/src/bootmenu/entries.rs:41-43`.
  - The modes that need a TPM: `requires_tpm` in `nonos-bootloader/src/menu/types/mode.rs:57-58`.
  - The volume key comes from the TPM: `derive_for_kernel` in `src/fs/blockfs_volume/open_machine.rs:19-35`.
  - The passphrase call, which no capsule calls: `mk_data_volume_passphrase` in `userland/libc/src/data.rs:53`.
  - The keyring answers `ENOENT` without a TPM: `userland/capsule_keyring/src/server/handlers/vault_seal.rs:57-61`.

## See also

- [Overview](README.md)
- [Mission](mission.md)
- [Architecture](architecture.md)
- [Threat model](threat-model.md)
- [Glossary](glossary.md)
- [Install](../install/README.md)
