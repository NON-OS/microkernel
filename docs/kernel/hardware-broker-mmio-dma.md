# The hardware broker: MMIO windows and DMA buffers

Part of [The hardware broker](hardware-broker.md).

## MMIO windows

`MkMmioMap` needs `Mmio`. `map_for_caller` resolves the claim and its epoch, finds the BAR, checks that the request lies inside it, maps the pages and records the grant (`src/hardware/broker/mmio/map.rs:34-68`). The grant is entered only while the claim it was made under still holds, and within the holder's quota, or the map is undone (`record_while_held`, `src/hardware/broker/claim/still_held.rs:25-37`; `src/hardware/broker/mmio/map_record.rs`).

- The pages are user, read and write, strong uncacheable and never executable, as `map_user_mmio` sets them (`src/memory/paging/manager/api/mapping/map_user_mmio.rs`).
- One flag is known, `MK_MMIO_MAP_WC` (bit 0), for a framebuffer or VRAM. `cache_for` gives write-combining only on a prefetchable BAR, whose reads have no side effects, and only for pages wholly inside that BAR. On any other BAR it answers `EINVAL`, and for pages that reach past the BAR `EINVAL` too. Without a WC entry in every CPU's PAT the pages stay strong uncacheable. A write-combined page is PWT alone, PAT entry 1, and is not marked a device register. Any other flag bit is `ENOTSUP` (`src/hardware/broker/mmio/cache.rs`, `fn cache_for`; kernel_proofs::mmio_cache).
- Every device's MSI-X table and pending bit array are kept out of capsule memory: `protected_regions` lists them and a mapping stops short of the first one or is refused (`src/hardware/broker/mmio/msix_exclusion.rs:39-50`). The kernel programs those tables itself.
- Grants live in the window from `USER_MMIO_BASE` to `USER_MMIO_END`, `0x80_0000_0000` to `0x90_0000_0000`, with an unmapped page after each grant (`src/hardware/broker/windows.rs:31-32`). Each address space has its own window, found by the tables its holder runs in, and a range goes back to it once the grant's pages are unmapped (`src/hardware/broker/va_space.rs`, `src/hardware/broker/va_window.rs`).
- A process holds at most 64 MMIO grants, 1024 DMA grants and 128 MiB of DMA at once; one more answers `ENOMEM` (`src/hardware/broker/quota.rs`, `fn spent`). `DMA32` grants count against the same quotas as any other. A refusal for a quota prints one rate-limited `[DIAG] broker.quota` line naming MMIO or DMA, count or bytes, and the pid, so a spent quota can be told from a spent window (`src/hardware/broker/quota_log.rs`, `fn refused`).
- A known bound: an address space's window entry in `SPACES` is kept after the space dies, one small entry per driver address space ever made (`src/hardware/broker/va_space.rs`, `static SPACES`).
- `MkMmap` at a fixed address and `MkMunmap` refuse any range that touches the MMIO or DMA window, which `touches_device_window` checks (`src/hardware/broker/windows.rs:36-45`), so a device page can only be given back through the broker.

## DMA buffers

`MkDmaMap` needs `Dma`. `map_for_caller` runs one transaction: validate, allocate and zero the frames, map them into the capsule, give the device an address, record the grant while the claim still holds and within the quota, and undo every earlier step if a later one fails (`src/hardware/broker/dma/map/transaction.rs:26-73`).

| Flag | Value | Meaning |
|---|---:|---|
| `DMA_MAP_HIGH` | 1 | Frames from the display pool or high memory. |
| `DMA_MAP_DMA32` | 2 | Frames below 4 GiB, for a device with 32 bit addresses. |
| `DMA_MAP_COHERENT` | 4 | Mapped uncached, for rings both sides write. |
| `DMA_MAP_WC` | 8 | Mapped write combining where the PAT allows, else uncached. |
| `DMA_MAP_TO_DEVICE` | 16 | The device only reads it: read-only in its IOMMU domain. |
| `DMA_MAP_FROM_DEVICE` | 32 | The device only writes it: write-only in its IOMMU domain, read kept on a VT-d unit without CAP.ZLR. |

The flags are in `flags.rs`, starting at `DMA_MAP_HIGH` (`src/hardware/broker/dma/flags.rs:19-38`). `validate` refuses `HIGH` with `DMA32`, `COHERENT` with `WC`, `TO_DEVICE` with `FROM_DEVICE`, a length that is not a whole number of 4 KiB pages, and a length above the device class's ceiling (`src/hardware/broker/dma/map/validate.rs:39-67`). The direction reaches the device's mapping through `direction.rs` and `confine::map`; for a device no IOMMU confines it changes nothing.

The ceilings, in pages, are set by `dma_page_limit_for_class`: `RNG`, `INPUT` and `SERIAL` 1, `AUDIO` 16, `NETWORK` 64, the USB hosts 256, `BLOCK` 1024, `DISPLAY` 8192 and every other class 16 (`src/hardware/broker/dma/limits.rs:27-47`).

Where the frames come from is decided in `take`, in the order `sources` gives (`src/hardware/broker/dma/map/alloc.rs`, `fn take`; `src/hardware/broker/dma/sources.rs`, `fn sources`). A `DMA32` request takes the low pool, then any memory below 4 GiB, and fails with `ENOMEM` rather than use higher frames; kernel_proofs::dma_placement holds both, placed low and refused when low memory is spent. A request without flags also prefers low memory, so a device with no IOMMU in front of it gets low addresses when there are some. The low pool is one page in 128 of the usable memory below 4 GiB, never under 2048 pages nor over 8192, and half of it is kept for `DMA32` requests, as `low32_target_pages` and `low32_floor` compute (`src/hardware/broker/dma/pool/sizing.rs:17-62`). These are the [DMA pools](../overview/glossary-dma-pool.md#dma-pool). Frames outside the pools come from the kernel's [frame allocator](frame-allocator.md#allocating-and-freeing-frames), which honours the same 4 GiB limit.

The device address depends on the IOMMU. When a remapping unit confines the device, `map` in the confine module maps the buffer into the capsule's own domain and returns an I/O virtual address; otherwise it returns the physical address (`src/hardware/broker/confine/map.rs:22-50`). I/O virtual addresses start at `IOVA_BASE`, 1 MiB, stay below 4 GiB, and step over the interrupt window `0xFEE0_0000` to `0xFEF0_0000`, where a device write would be taken as an interrupt (`src/hardware/broker/confine/iova_space.rs:29-74`). A grant made without a confining domain is counted by `note_unconfined` and shows in the IOMMU posture line (`src/hardware/broker/dma/map/record.rs:53-59`). A `DMA32` request whose device address would not fit 32 bits fails as `Above4G`, which the caller sees as `ERANGE`, -34 (`src/hardware/broker/dma/map/transaction.rs:51-61`).

The user mapping of a buffer lives in the window from `USER_DMA_BASE`, `0xA0_0000_0000`, to `USER_DMA_END`, `0xB0_0000_0000` (`src/hardware/broker/windows.rs:33-34`), each address space's own, as for MMIO.
