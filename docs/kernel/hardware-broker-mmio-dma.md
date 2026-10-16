# The hardware broker: MMIO windows and DMA buffers

Part of [The hardware broker](hardware-broker.md).

## MMIO windows

`MkMmioMap` needs `Mmio`. `map_for_caller` resolves the claim and its epoch, finds the BAR, checks that the request lies inside it, maps the pages and records the grant (`src/hardware/broker/mmio/map.rs:49-112`).

- The pages are user, read and write, strong uncacheable and never executable, as `map_user_mmio` sets them (`src/memory/paging/manager/api/mapping/map_user_mmio.rs:23-44`).
- Every device's MSI-X table and pending bit array are kept out of capsule memory: `protected_regions` lists them and a mapping stops short of the first one or is refused (`src/hardware/broker/mmio/msix_exclusion.rs:39-50`). The kernel programs those tables itself.
- Grants live in the window from `USER_MMIO_BASE` to `USER_MMIO_END`, `0x80_0000_0000` to `0x90_0000_0000`, with an unmapped page between two grants (`src/hardware/broker/windows.rs:31-32`), placed by `reserve_user_va` (`src/hardware/broker/grant.rs:140-153`).
- `MkMmap` at a fixed address and `MkMunmap` refuse any range that touches the MMIO or DMA window, which `touches_device_window` checks (`src/hardware/broker/windows.rs:36-45`), so a device page can only be given back through the broker.

## DMA buffers

`MkDmaMap` needs `Dma`. `map_for_caller` runs one transaction: validate, allocate and zero the frames, map them into the capsule, give the device an address, record the grant, and undo every earlier step if a later one fails (`src/hardware/broker/dma/map/transaction.rs:23-66`).

| Flag | Value | Meaning |
|---|---:|---|
| `DMA_MAP_HIGH` | 1 | Frames from the display pool or high memory. |
| `DMA_MAP_DMA32` | 2 | Frames below 4 GiB, for a device with 32 bit addresses. |
| `DMA_MAP_COHERENT` | 4 | Mapped uncached, for rings both sides write. |
| `DMA_MAP_WC` | 8 | Mapped write combining where the PAT allows, else uncached. |

The flags are in `flags.rs`, starting at `DMA_MAP_HIGH` (`src/hardware/broker/dma/flags.rs:19-30`). `validate` refuses `HIGH` with `DMA32`, `COHERENT` with `WC`, a length that is not a whole number of 4 KiB pages, and a length above the device class's ceiling (`src/hardware/broker/dma/map/validate.rs:31-58`).

The ceilings, in pages, are set by `dma_page_limit_for_class`: `RNG`, `INPUT` and `SERIAL` 1, `AUDIO` 16, `NETWORK` 64, the USB hosts 256, `BLOCK` 1024, `DISPLAY` 8192 and every other class 16 (`src/hardware/broker/dma/limits.rs:27-47`).

Where the frames come from is decided in `take` (`src/hardware/broker/dma/map/alloc.rs:27-58`). A `DMA32` request takes the low pool, then any memory below 4 GiB, and fails with `ENOMEM` rather than use higher frames. A request without flags also prefers low memory, so a device with no IOMMU in front of it gets low addresses when there are some. The low pool is one page in 128 of the usable memory below 4 GiB, never under 2048 pages nor over 8192, and half of it is kept for `DMA32` requests, as `low32_target_pages` and `low32_floor` compute (`src/hardware/broker/dma/pool/sizing.rs:17-62`). These are the [DMA pools](../overview/glossary-dma-pool.md#dma-pool). Frames outside the pools come from the kernel's [frame allocator](frame-allocator.md#allocating-and-freeing-frames), which honours the same 4 GiB limit.

The device address depends on the IOMMU. When a remapping unit confines the device, `map` in the confine module maps the buffer into the capsule's own domain and returns an I/O virtual address; otherwise it returns the physical address (`src/hardware/broker/confine/map.rs:22-50`). I/O virtual addresses start at `IOVA_BASE`, 1 MiB, stay below 4 GiB, and step over the interrupt window `0xFEE0_0000` to `0xFEF0_0000`, where a device write would be taken as an interrupt (`src/hardware/broker/confine/iova_space.rs:29-74`). A grant made without a confining domain is counted by `note_unconfined` and shows in the IOMMU posture line (`src/hardware/broker/dma/map/record.rs:45-51`). A `DMA32` request whose device address would not fit 32 bits fails as `Above4G`, which the caller sees as `ERANGE`, -34 (`src/hardware/broker/dma/map/transaction.rs:53-62`).

The user mapping of a buffer lives in the window from `USER_DMA_BASE`, `0xA0_0000_0000`, to `USER_DMA_END`, `0xB0_0000_0000` (`src/hardware/broker/windows.rs:33-34`).
