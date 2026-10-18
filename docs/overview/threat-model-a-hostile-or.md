# Threat model: A hostile or faulty device

Part of [Threat model](threat-model.md).

### A hostile or faulty device

On Intel VT-d machines the [IOMMU](glossary-hardware-broker.md#iommu) confines device DMA. Every
image turns translation on at boot, and a device the boot PCI scan did not find,
such as a card added later, is denied (`bring_up` in
`src/arch/x86_64/iommu/unit/bringup/run.rs:26-36`). When a driver claims a
device that a unit in service covers, the broker puts it in the [IOMMU
domain](glossary-hardware-broker.md#iommu-domain) of that driver capsule, so it reaches only
what that capsule was granted, and refuses a claim the unit cannot confine or
whose interrupts it cannot remap (`attach` in
`src/hardware/broker/confine/attach.rs`, `verdict` in
`src/hardware/broker/confine/decide.rs`). A
released device has its bus mastering turned off before it leaves the domain
(`release` in `src/hardware/broker/claim/release.rs:24-36`). At shutdown and
restart every claimed device is stopped and its DMA buffers wiped before the
rest of memory is (`quiesce_all_devices` in
`src/security/hardening/memory_sanitization/api.rs:65-69`).

### An observer on the network

The browser, the Terminal and the wallet connect through the [Nym
mixnet](glossary-linux-personality.md#nym-mixnet) by default, and Direct is used only when the
person chose it (`pick` in `userland/nonos_route_link/src/pick.rs:125-139`).
Downloads for an install go over the [Anyone
network](glossary-amnesic-boot.md#anyone-network) whatever the default (`install_route` in
`userland/nonos_route_link/src/pick.rs:49-63`), except a model the person asks
to download direct, with `qwen get --direct` or `d` on its Marketplace card
(`DIRECT_WORD` in `userland/capsule_model_fetch/src/get/run.rs:42`). The
wallet's chain reads never go direct (`private_only` in
`userland/nonos_route_link/src/pick.rs:86-100`). No image this release builds
asks a time server: net.ntp, which would ask only under Direct, is not started
on any release image (`spawn_ntp` in
`src/userspace/init/spawn_plan/network/mod.rs:28-29`). The network drivers this
release builds (e1000, rtl8139, rtl8169, virtio-net, iwlwifi and rtl8821ce) draw
a random, locally administered station address at bring-up instead of using the
factory one (`apply` in `userland/nonos_mac/src/local.rs:46-55`). [How the Nym
and Anyone transports are built](../security/anonymity-transports.md) says what
`net.nym` and `net.anon` implement of each protocol, and what they leave out.

### Someone who takes the machine while it is off

A live boot's data volume was only in RAM, and it fades with the power: the wipe
before a restart does not reach it. On an installed disk the data volume holds
only sealed sectors. The package store beside it is not encrypted, so the files
kept there, setup's answers among them, can be read from the disk, apart from
records their capsule sealed (`append` in
`userland/capsule_vfs/src/blk/store_write.rs:39-84`). The TPM derives the volume
key from a seed it never releases and the current boot [PCRs](glossary-linux-personality.md#pcr),
and stores nothing, so the same disk in another machine, or the same machine in
another boot state, gets a different key (`TPM2_HMAC` in
`src/security/tpm/machine_key/mod.rs:17-31`).
