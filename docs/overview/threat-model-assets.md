# Threat model: Assets

Part of [Threat model](threat-model.md).

## Assets

| Asset | Where it lives | What guards it |
|---|---|---|
| Qwen models on an installed disk | the [data volume](glossary-claim-epoch.md#data-volume) | every sector sealed, under a TPM-derived key |
| Files a program keeps, setup's answers and kept settings | the [package store](glossary-linux-personality.md#store) on the NONOS disk | nothing: the store is not encrypted, and only a record its capsule sealed, such as the saved Wi-Fi list or the wallet's key, is protected |
| What a live boot holds | RAM | power off; the [ZeroState](glossary-stark-proof.md#zerostate) wipe runs in 0.9.2 only at the installer's restart, and leaves the in-memory data volume and its key |
| The wallet's key | the keyring capsule | the keyring's vault gate |
| Where this machine is on the network | its IP address and its cards' station addresses | the chosen anonymity network for the system's own connections, and a fresh station address at each bring-up |
| What runs | the kernel image and every capsule | signatures, attestation and the [capability word](glossary-boot-root-record.md#capability-word) |
| The [device secret](glossary-claim-epoch.md#device-secret) | derived from the [TPM](glossary-stark-proof.md#tpm) | the `DeviceSecret` bit, held by one capsule |

The sections below give the code behind each guard.

## Adversaries

### A hostile app or capsule

It runs with the bits its [manifest](glossary-linux-personality.md#manifest) grants and no others,
unless a capsule holding Admin grants it more, and only bits that capsule holds
itself (`sys_cap_grant` in
`src/syscall/microkernel/capability/handlers.rs:33-52`). Every NONOS syscall it
makes passes the capability check (`dispatch` in
`src/syscall/contract/dispatch.rs:25-40`). Reaching any of the 11 services that
carry traffic off the machine needs Network (`NETWORK_SERVICES` in
`src/services/registry/policy.rs:19-45`). It cannot read the file store without
FileSystem (`CAP_FILE_SYSTEM` in
`userland/capsule_vfs/src/server/fs_gate.rs:35-40`), and with FileSystem it can
open any file there, another capsule's included, since `open` checks the file's
mode and not who made it
(`userland/capsule_vfs/src/store/fdtable/open.rs:25-50`). It cannot read raw
sectors from the NVMe, AHCI or virtio-blk driver without StoreWrite (`permits`
in `userland/capsule_driver_nvme/src/server/medium.rs:25-30`). It cannot have
the keyring open the wallet's sealed record unless the service registry names it
as the wallet (`allowed` in
`userland/capsule_keyring/src/server/vault_gate/mod.rs:23-30`). A capsule that
holds both Crypto and FileSystem needs no keyring: it can ask the kernel for the
vault root under the published label, as the keyring does with `machine_key` and
`ROOT_LABEL` (`userland/capsule_keyring/src/vault/root.rs:29`), and read the
sealed blob under `/data`. The Linux personality, Settings and the Terminal are
among the capsules that hold both. A flood of messages from one sender fills at
most half of any one [inbox](glossary-hardware-broker.md#inbox) (`SHARE_BYTES_MAX` in
`src/ipc/nonos_inbox/budget.rs:17-43`). The `DeviceSecret` bit, which receives
the TPM-derived device secret, is held by one capsule, `app.prove`
(`DeviceSecret` in `src/capabilities/types/defs.rs:80-82`, `CAPSULE_HANDLE` in
`userland/capsule_prove/Capsule.mk:17-25`).

`userland/capsule_attack` tries escapes of this kind with raw syscalls, a socket
without Network and an MMIO window without Mmio among them, and prints the
kernel's answer to each. No Cargo feature, kernel mirror or make include names
it yet, so no image carries it (`userland/capsule_attack/Capsule.mk`).

### A hostile Linux program

A Linux program holds no NONOS capabilities. The kernel parks each of its Linux
syscalls for the [Linux personality](glossary-linux-personality.md#linux-personality) to answer
(`redirect` in `src/process/foreign/trap.rs:26-54`), and refuses every NONOS
syscall it tries except `MkTtyQuery`, which only describes its own streams
(`MkTtyQuery` in `src/syscall/contract/cap_table/mk_surface.rs:58`). The
personality's run and terminal roles hold no Network, and every network service
requires it (`RUN` and `TERMINAL` in
`src/userspace/capsule_linux/roles.rs:46-67`). The personality is itself a
capsule, so a program that subverts it gets the personality's bits and no more:
CoreExec, IPC, Memory, Crypto, FileSystem, Debug, two display bits, StoreWrite,
ForeignExec and LocalSign, and Network as well in the install role
(`CAPSULE_REQUIRED_CAPS` and `CAPSULE_OPTIONAL_CAPS` in
`userland/capsule_linux/Capsule.mk:45-48`).
