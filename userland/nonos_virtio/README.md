# nonos_virtio

The virtio 1.0 ("modern") PCI transport, shared by the four virtio driver
capsules: `capsule_driver_virtio_blk`, `capsule_driver_virtio_net`,
`capsule_driver_virtio_gpu` and `capsule_driver_virtio_rng`. It is a `no_std`
library with no dependencies; it is not a capsule and holds no capability.

## Why it exists

A modern-only virtio function has no legacy I/O window. Its registers are four
structures (common, notify, ISR and device configuration) that vendor
capabilities in config space place inside a memory BAR. QEMU builds such
functions once a device sits behind its IOMMU (`iommu_platform=on` refuses
legacy and transitional mode). A driver that knew only the legacy layout took
the first MMIO BAR it found, which on such a function is the MSI-X table BAR,
and the broker refuses to map that.

## What is here

| Module | What it does |
|---|---|
| `select` | `wants_probe` and `choose`: a transitional function with its legacy I/O BAR keeps the legacy path without a config-space read; a modern id, or a function with no I/O BAR, has its capabilities read and is driven modern when a usable common configuration is among them |
| `pci` | the BAR list in transport terms, modern and legacy ids, a config-space snapshot read through the broker |
| `caps` | the vendor capability walk over that snapshot: the first usable structure of each type, lengths and BARs checked, MSI-X table pages kept out |
| `map` | `map_window`: map the regions a driver needs (`Need` says whether ISR and device config are wanted) through the broker, checking the length the broker actually gave |
| `common` | the common configuration: reset and status, feature negotiation (VERSION_1, ACCESS_PLATFORM when offered), queue programming with an optional MSI-X vector and its read-back |
| `notify` | the doorbell address for a queue, bounded by the mapped notify region |
| `mmio` | bounds-checked volatile accessors over a mapped region |
| `broker` | the `Broker` trait: the three kernel calls the transport needs, which each driver implements on `nonos_libc` |

The `Broker` trait is `unsafe`: an implementation may return `Ok` from
`mmio_map` only for a mapping that stays valid until it is unmapped, because the
register windows dereference it without checking again.

## How it is built and checked

Each virtio driver depends on it by path and links it into its capsule ELF.
Because the broker calls come in through a trait, the whole crate builds for
the host. `userland/virtio_transport_proofs` includes its modules one by one
with `#[path]` and runs them against synthetic config spaces and an in-memory
broker; modules name each other by full `crate::` path so that works.
`virtio_blk_proofs`, `virtio_net_proofs` and `virtio_gpu_proofs` depend on it
for their drivers' modern paths.

## What it does not do

It has no virtqueue layout or descriptor handling (each driver keeps its own),
binds no interrupt, and claims nothing: the driver claims the device and owns
every grant.

See [drivers](../../docs/handbook/drivers.md).
