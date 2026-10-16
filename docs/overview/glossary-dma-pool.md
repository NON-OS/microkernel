# Glossary: DMA pool

Part of [Glossary](glossary.md).

## DMA pool

Memory the hardware broker reserves for DMA buffers: a low pool below 4 GiB for
devices that can only name 32-bit addresses, and a high pool for display
surfaces. A run of pages goes back only to the pool it came from. Explained in
[The hardware broker](../kernel/hardware-broker.md#dma-buffers). Code:
`src/hardware/broker/dma/pool/mod.rs`.

## Driver capsule

A capsule, built from a `userland/capsule_driver_*` crate, that drives one kind
of device from ring 3. It reaches its device only through grants from the
hardware broker and serves it over IPC, to the kernel or to other capsules. The
tree has 27 driver crates; 18 of them are built in this release, and the other
nine are in no image. Explained in [Writing a
driver](../drivers/writing-a-driver.md). Code: `mk/20-build.mk`,
`userland/capsule_driver_virtio_rng/Capsule.mk`.

## Endpoint

A named IPC address with a port. A capsule's manifest declares its service
endpoint, which others send to, and its reply endpoint; the kernel's service
registry records each with the pid that serves it and the capability bits a
sender must hold. Explained in [IPC](../kernel/ipc.md#the-model). Code:
`src/services/registry/endpoint.rs`,
`src/security/capsule_manifest/schema/endpoint.rs`.

## Enrollment

Committing a set of measurements, such as the BLAKE3 hash of every capsule's
ELF, to one Merkle policy tree, then writing its root and a trailer for each
member. The seal runs one enrollment for the capsule set and one each for the
kernel and the loader, and keeps the capsule enrollment already in the tree when
no capsule source changed. Each enrollment draws a fresh pad seed, so enrolling
the same members again gives a new root. Explained in [STARK
attestation](../security/stark-attestation.md#who-makes-the-trailers). Code:
`nonos-stark-enroll/src/commands.rs`, `tools/nonos_seal/__main__.py`.

## ESP

The EFI system partition, a FAT volume. It holds the loader as
`EFI/BOOT/BOOTX64.EFI` and, under `EFI/nonos`, the signed `kernel.bin`, the
loader's trailer, the boot-root record, the kernel approval when there is one,
and `boot.cfg`. Explained in [Install to
disk](../install/install-to-disk.md#what-is-written). Code:
`tools/nonos_seal/media.py`.

## File store

The `vfs_pool` service, `capsule_vfs`. It holds every file the desktop and its
apps see in its own memory, serves holders of FileSystem, and writes a file to
the package store only when asked to keep it on a boot that keeps data.
Explained in [Files](../using/files.md). Code:
`userland/capsule_vfs/Capsule.mk`,
`userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Foreign process

<a id="guest"></a>A process that runs code the kernel has not verified, on x86_64 only; the Linux pages call it a guest. The Linux personality creates one with `MkForeignSpawn`, which needs ForeignExec, and it starts with a capability word of 0. A system call number the kernel does not know is parked for its supervising capsule to answer. Explained in [The Linux personality](../userland/linux-personality.md). Code: `src/process/foreign/spawn.rs`, `src/process/foreign/trap.rs`.

## Grant

One revocable piece of a claimed device that the hardware broker hands the
claiming process: an MMIO window, a DMA buffer, an interrupt binding or, on
x86_64, a port range. Each has a grant id, and the broker revokes it on unmap,
on device release and when the process exits. Explained in [The hardware
broker](../kernel/hardware-broker.md#revocation). Code:
`src/hardware/broker/grant/`, `src/process/exit/finalize.rs`.
