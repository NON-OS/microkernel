# FAQ: The network NONOS uses

Part of [FAQ](faq.md).

## The network NONOS uses

The browser, the Terminal and the wallet connect through the [Nym
mixnet](glossary-linux-personality.md#nym-mixnet) by default. The [Anyone
network](glossary-amnesic-boot.md#anyone-network) and Direct are the other two choices, asked
in first-boot setup and changed in Settings. If the chosen network is not
running, connections fail: nothing falls back to Direct. Downloads for an
install, a Qwen model or a Linux package, go over the Anyone network whatever
the choice; a model goes direct only when you ask for that one download, with
`qwen get --direct` or `d` on its Marketplace card. The wallet never uses Direct
for its chain reads, and no image in this release asks a time server. See
[Privacy network](../using/privacy-network.md).

## Installing NONOS on a disk

You can install NONOS on an internal disk. Choose to install in first-boot
setup, or pick `Install NØNOS` in the boot menu, and the installer writes to the
disk you choose after you confirm. Until then NONOS writes nothing to any disk,
except that choosing Install in setup keeps your answers in the stick's own
store. See [Install to disk](../install/install-to-disk.md).

## Trying NONOS in a virtual machine

The `qemu` [build profile](glossary-boot-root-record.md#build-profile) is the desktop for virtual
machines. It carries 11 of the 18 driver capsules and leaves out the Wi-Fi
drivers, the Intel and Realtek Ethernet drivers and the I2C drivers. From a
checkout with Nix, the `Makefile` builds a development image, by default the
development twin of the `qemu` profile, and boots it under QEMU with a software
TPM attached:

```sh
make dev-image
make dev-boot
```

Not tested in this release.

A [development image](glossary-claim-epoch.md#development-image) is sealed with throwaway
keys and path-only attestation, and it is never a release. See [Get an
image](../install/get-an-image.md) and [Make targets](../build/make-targets.md).

## What happens at shutdown

In 0.9.2 the desktop cannot shut down or restart the machine: the Power key
shows `Power off is not available from the desktop`. To turn the machine off,
force it off with its power button, as the [release notes](../release/0.9.2.md)
describe. A forced power-off skips the wipe below, as a power cut or a kernel
panic does.

The one orderly path is the installer's restart, and it runs the
[ZeroState](glossary-stark-proof.md#zerostate) wipe first. The kernel stops the other CPUs
and every claimed device, then wipes device buffers, process memory, kernel
stacks, filesystem caches, its key vault, its RAM log and its heap, and only
then hands the machine to the firmware. Kernel statics outside the heap, the
data volume key among them, are not wiped, and neither is a live stick's
in-memory volume. See [Design
principles](design-principles-amnesic-by-default.md#amnesic-by-default).

## Without a TPM

The boot menu describes its Hardened entry as "Standard, and refuses to boot
without Secure Boot and a TPM". Air-Gapped refuses to boot without one too,
because the TPM holds the [rollback floor](glossary-peer-list.md#rollback-floor); every
other entry boots with rollback protection off, so an older signed kernel would
boot. Without a TPM the [machine key](glossary-linux-personality.md#machine-key) cannot be
derived, so an installed data volume stays closed. The kernel could key a volume
with a passphrase instead, but no capsule in this release asks for one. Without
a TPM the keyring cannot seal the wallet's record, so it answers ENOENT and
nothing is saved. See [Boot modes](../install/boot-modes.md) and [Measured boot
and the TPM](../security/measured-boot-and-tpm.md).
