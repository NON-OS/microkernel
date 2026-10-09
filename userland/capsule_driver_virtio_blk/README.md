# capsule_driver_virtio_blk

## Role

`capsule_driver_virtio_blk` is the virtio block-device capsule. It owns the
virtio block queue and exposes sector-oriented read/write/flush operations over
IPC. It deliberately does not own filesystems, partitions, encryption, or cache
policy.

```text
kernel block client / StoreWrite holders
    |
    | sector request IPC
    v
driver.virtio_blk0 -- virtqueue DMA --> virtio-blk device
    |
    `-- IRQ (INTx, else MSI-X) or used-ring polling
```

## Microkernel contract

The capsule uses brokered hardware authority:

- `MkDeviceList` locates the virtio block device.
- `MkDeviceClaim` owns the device claim.
- `MkPioGrant` takes a transitional disk's legacy I/O BAR, or `MkMmioMap`
  maps its register window (legacy MMIO, or the modern structures).
- `MkIrqBind` binds the INTx line, or one MSI-X vector when that fails;
  `MkIrqWait` and `MkIrqAck` wait on it in slices, with a used-ring check
  after each (`src/io/wait_slice.rs`).
- `MkDmaMap` and `MkDmaUnmap` allocate queue, request, and data buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.virtio_blk0` on
  `service:4202:driver.virtio_blk0`.

The kernel never embeds block-device policy. It validates capabilities,
mediates grants, routes IPC, and tears grants down on exit.

The driver guards the medium itself (`src/server/acl.rs`): every op but the
health check answers only the kernel's own client (sender pid 0) or a sender
the kernel says holds `StoreWrite`, asked with `MkCapCheck` on every request.
Reads are held to the same rule as writes.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CAPACITY` | sector capacity | 8-byte capacity |
| `OP_READ_BLOCKS` | read sectors into reply payload | status plus bytes |
| `OP_WRITE_BLOCKS` | write sectors from request payload | status word |
| `OP_FLUSH` | force device flush | status word |

## Authority

The manifest grants `CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`,
`Mmio`, `Irq`, `Dma` and `Pio` (`CAPSULE_REQUIRED_CAPS = 0x1F8019`). It has no filesystem,
partition, crypto, admin, or raw physical-memory authority, and `Debug` only as the
optional `0x100` that a `capsule-serial-debug` build grants.

```text
allowed:   virtio block claim, MMIO or PIO registers, IRQ, DMA queue, sector IPC
forbidden: filesystem parsing, partition ownership, writeback cache, LUKS
```

## Privacy and persistence

Sector payloads pass through broker DMA buffers for the active request. The
capsule does not persist a cache, inspect filesystem semantics, index file
contents, or keep block data after reply completion.

## Runtime lifecycle

The capsule claims the virtio block device, maps its registers, binds IRQ,
allocates queue/request/data DMA, initializes the queue, probes capacity, and
serves IPC. Teardown releases DMA, IRQ, register, and claim grants.

A transitional disk that still has its legacy I/O BAR is driven over the legacy
registers. A modern-only one (what QEMU builds once the device sits behind its
IOMMU) is driven over the virtio 1.0 structures its vendor capabilities place in
a memory BAR, through the shared transport in `userland/nonos_virtio`, with
VERSION_1 and ACCESS_PLATFORM negotiated, the ISR read to lower INTx, and with
MSI-X bound the request queue pointed at table entry 0 and read back.

## Failure model

Setup failure rolls back grants, and a failed attempt releases the claim, which
takes every grant with it. Bring-up is tried a bounded number of times with a
sleep between tries (`nonos_libc::bring_up`), each failure logged; the capsule
exits 2 when no usable disk is listed and 6 when one is present but never comes
up. Runtime request failures return block errors without retry loops in the
kernel. Capacity and request-size bounds are checked
before DMA is submitted.

## Current implemented surface

- Discovers and claims the virtio block device.
- Legacy and virtio 1.0 (modern) PCI transports, chosen per function.
- Initializes the virtqueue.
- Probes device capacity.
- Handles read, write, and flush requests over IPC.
- Waits on the interrupt in bounded slices, with a used-ring check after each.
- Answers the medium only to the kernel or a `StoreWrite` holder.
- Unwinds broker grants on setup failure and shutdown.

## Wire format

Requests use the `NBLK` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Capacity replies
return 8 bytes. Read/write requests use a 12-byte block header and data bounded
by `MAX_RW_PAYLOAD_BYTES`.

## State ownership

The capsule owns the virtqueue, request headers, data buffers, device capacity,
register grant, IRQ grant, and DMA grants. Filesystem and partition capsules own
all interpretation above sector reads and writes.

## Operating rules

- Validate sector range and payload length before submitting DMA.
- Keep reads/writes sector-oriented.
- Never cache filesystem contents here.
- Roll back DMA, IRQ, register, and claim grants on setup failure.

## Release target

The finished virtio-blk capsule is a signed block-device service with stable
capacity reporting, read/write/flush semantics, queue reset recovery,
teardown-safe DMA handling, QEMU validation, and hardware-equivalent virtio proof.
It exposes sectors only; all filesystems and storage policy remain above it.

## Release evidence

Release requires QEMU read/write/flush validation, bounds tests for request length,
teardown DMA revocation proof, and a filesystem capsule test mounted above it.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU read/write/flush validation passes.
- Request length and capacity bounds are tested.
- Teardown proof shows all DMA grants are revoked.
- VFS/filesystem test works above the block endpoint.

## Explicit non-goals today

No partition table, filesystem, encryption layer, snapshotting, writeback
cache, allocator, volume manager, or fsck policy lives in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-virtio-blk`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: virtio-blk must use broker MMIO/PIO/IRQ/DMA and must not
  import kernel memory or driver internals.
- Proofs: `userland/virtio_blk_proofs` (parsers, the modern capacity read and
  the medium rule) and `userland/virtio_transport_proofs`.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [storage](../../docs/handbook/storage.md).
- Documentation check: this README is required for the capsule to pass CI.

## Real bring-up checklist

Proved on the host, without a device: the transport choice, the capability
parse, feature negotiation, queue programming (with its MSI-X vector and the
read-back), the doorbell arithmetic (`userland/virtio_transport_proofs`), the
request parsers and the modern capacity read (`userland/virtio_blk_proofs`).
Only a boot shows the rest. Under QEMU q35, confirm both ways:

- Without an IOMMU (`-device virtio-blk-pci`, transitional 0x1001 with its
  legacy I/O BAR): `[BLK] step transport`, `regs`, `irq-bind`, `dma-*`,
  `bring-up`, `ready`; the capacity matches the image; vfs mounts; reads,
  writes and flushes complete.
- With the IOMMU lane (`-device intel-iommu` with `QEMU_IOMMU_OPTS`, and the
  device with `QEMU_IOMMU_VIRTIO`, i.e. `iommu_platform=on,disable-legacy=on`;
  see mk/10-qemu.mk) (modern-only 0x1042): `[BLK] step modern-pci`,
  `modern-regs`, `irq-bind`, `dma-*`, `bring-up`, `ready`; the capacity matches
  the image; with MSI-X bound there is no `device refused MSI-X entry 0` line
  and requests complete without the 100 ms wait slice (the interrupt reaches the
  driver through the IOMMU's interrupt path, which is the kernel's); with INTx
  the ISR read keeps the line from re-firing; the VT-d log shows no DMA fault.
- A disk that cannot come up: one `[BLK] setup stuck:` line per attempt, at
  most seven over about six seconds, one `driver.virtio_blk0: device present,
  bring-up failed` line, exit 6. No usable disk at all: exit 2 at once.
