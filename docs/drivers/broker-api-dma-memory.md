# Broker API: DMA memory

Part of [Broker API](broker-api.md).

## DMA memory

`mk_dma_map(device_id, epoch, length, flags, out)` returns a buffer the device
may read and write unless a direction flag says otherwise; `mk_dma_map` is the
wrapper (`userland/libc/src/broker/dma.rs:49-57`). The flags are
`MK_DMA_MAP_HIGH` (bit 0), `MK_DMA_MAP_DMA32` (bit 1, frames below 4 GiB or
fail), `MK_DMA_MAP_COHERENT` (bit 2, mapped uncached, for rings both sides
write), `MK_DMA_MAP_WC` (bit 3, write-combining where the PAT has the entry),
`MK_DMA_MAP_TO_DEVICE` (bit 4, a buffer the device only reads, such as a
transmit buffer) and `MK_DMA_MAP_FROM_DEVICE` (bit 5, one it only writes, such
as a receive buffer) (`userland/libc/src/broker/dma.rs:22-46`). The direction is
enforced by the IOMMU where one confines the device; set neither for a ring.
`validate` refuses HIGH with DMA32, COHERENT with WC, the two directions
together, and a length that is zero or not a multiple of 4096
(`src/hardware/broker/dma/map/validate.rs:39-55`).

One grant may not exceed the page ceiling of the device's class, from
`dma_page_limit_for_class` (`src/hardware/broker/dma/limits.rs:27-47`):

| Class | Pages per grant |
|---|---|
| RNG, INPUT, SERIAL | 1 |
| AUDIO | 16 |
| NETWORK | 64 |
| USB_HOST, USB_HOST_XHCI | 256 |
| BLOCK | 1024 |
| DISPLAY | 8192 |
| any other class | 16 |

`map_for_caller` validates, allocates and zeroes the frames, maps them into the
caller, maps them into the device's domain and records the grant, undoing each
step on failure (`src/hardware/broker/dma/map/transaction.rs:26-66`). The
32-byte `DmaMapOut` returns `user_va`, `device_addr`, `length` and `grant_id`
(`src/syscall/microkernel/dma.rs:43-52`). With a domain, `device_addr` is an
IOVA placed from `IOVA_BASE` (1 MiB) up, below 4 GiB, stepping over the
interrupt window from 0xFEE0_0000 to 0xFEF0_0000
(`src/hardware/broker/confine/iova_space.rs:31-37`). Without one, it is the
physical address. A DMA32 map whose address would end above 4 GiB fails with
-34, `ERRNO_RANGE` (`src/syscall/microkernel/dma.rs:109-111`).

`mk_dma_unmap(grant_id)` gives a buffer back. `teardown` unmaps it from the
driver, then takes it out of the device's domain before the frames go anywhere.
If the domain will not let go, the frames are quarantined and never reused;
otherwise `scrub` zeroes them before they are freed
(`src/hardware/broker/dma/teardown.rs:26-46`). For a write-back buffer on
x86_64, `mk_dma_sync_for_device` and `mk_dma_sync_for_cpu` flush the cache lines
without a system call (`userland/libc/src/broker/dma_sync.rs:27-48`).
