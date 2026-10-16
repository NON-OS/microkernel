# The hardware broker: revocation, errors and tests

Part of [The hardware broker](hardware-broker.md).

## Revocation

- `MkMmioUnmap`, `MkDmaUnmap` and `MkPioRelease` take back one grant.
- `MkDeviceRelease` stops the device mastering the bus first, then tears down its MMIO, IRQ, DMA and PIO grants, then drops the claim, in `sys_device_release` (`src/syscall/microkernel/device.rs:84-113`).
- On exit `teardown` releases every grant the process holds, through `release_all_for_pid` and its IRQ, DMA and PIO twins (`src/process/exit/teardown.rs:47-51`); the MMIO release also drops the process's claims (`src/hardware/broker/mmio/release.rs:45-54`). Dropping a claim stops the device mastering the bus and detaches it from the capsule's domain, in the claim module's `release_all_for_pid` (`src/hardware/broker/claim/release.rs:51-62`), before the DMA buffers are freed.

A DMA buffer is freed in a fixed order by `teardown` in the DMA module: unmap it from the capsule, take it out of the device's domain, scrub it, and only then return the frames (`src/hardware/broker/dma/teardown.rs:26-46`). If the domain will not give the buffer up, the frames are kept out of use for good rather than handed to someone else. `scrub` writes zeros through the kernel's [directmap](../overview/glossary-claim-epoch.md#directmap) before the frames are reused (`src/hardware/broker/dma/scrub.rs:19-35`).

## Errors

| Errno | Value | When |
|---|---:|---|
| `EPERM` | -1 | No claim on the device, an IOMMU in service would not take it, a reserved line, or a mapping that would expose an MSI-X table or its pending bit array. |
| `ENOMEM` | -12 | No frames, no user address space, or no free vector. |
| `EBUSY` | -16 | The device is claimed by another process, or the line is already bound. |
| `ENODEV` | -19 | No such device, or the kernel could not program the device's MSI or MSI-X. |
| `EINVAL` | -22 | A bad length, alignment, BAR index or vector count. |
| `ERANGE` | -34 | A `DMA32` buffer whose device address does not fit 32 bits. |
| `EOPNOTSUPP` | -95 | An unknown flag. |
| `ESTALE` | -116 | The claim epoch is not the live one. |

The DMA mapping is in `errno_for` (`src/syscall/microkernel/dma.rs:100-111`), the MMIO mapping beside `MmioMapError` (`src/syscall/microkernel/mmio/errno_map.rs:24-35`) and the IRQ mapping beside `IrqBindError` (`src/syscall/microkernel/irq/errno_map.rs:24-42`). `ESTALE` and `EOPNOTSUPP` are not in the `[errors]` table of [abi/syscalls.toml](../../abi/syscalls.toml); [Errors](../abi/errors.md) has both, with the rest of the codes.

## Tests

The [proof crate](../overview/glossary-peer-list.md#proof-crate) `userland/kernel_proofs` compiles the I/O address placement (`userland/kernel_proofs/src/iova_space/mod.rs`), the confinement posture (`userland/kernel_proofs/src/confine_posture/mod.rs`), the user windows (`userland/kernel_proofs/src/device_windows/mod.rs`), the MMIO window rule (`userland/kernel_proofs/src/mmio_window/mod.rs`), the DMA pool sizing (`userland/kernel_proofs/src/dma_pool_bitmap/mod.rs`) and the PCI vendor bits (`userland/kernel_proofs/src/pci_quirk_bits/mod.rs`) from the kernel sources. It passed, 388 tests, in the flake check run on this commit.
