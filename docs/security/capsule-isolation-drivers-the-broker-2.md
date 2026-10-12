# Capsule isolation: Drivers: the broker and the IOMMU, continued

Part of [Capsule isolation](capsule-isolation.md).

## Drivers: the broker and the IOMMU, continued

- `map_for_caller` maps a BAR slice only for the capsule holding the claim at
  its current epoch, and only inside that BAR
  (`src/hardware/broker/mmio/map.rs:49-80`). `protected_regions` keeps every
  MSI-X table and pending-bit array out of the mapping
  (`src/hardware/broker/mmio/msix_exclusion.rs:39-52`).
- `validate` lets a driver change only Command register bits 1, 2 and 10, the
  MSI-X enable and function-mask bits, and a few vendor bits for audio and
  network functions; it refuses every other config-space write before it reaches
  the bus (`src/hardware/broker/pci/allowlist.rs:17-60`).
- `alloc_and_zero` zeroes every frame of a DMA grant before the device or the
  capsule sees it (`src/hardware/broker/dma/map/alloc.rs:35-39`), and `scrub`
  zeroes the frames again before they go back to the allocator
  (`src/hardware/broker/dma/scrub.rs:27-35`).
- When a process ends, its MMIO, IRQ, DMA and PIO grants are released,
  `dma_release_all_for_pid` among them (`src/process/exit/teardown.rs:48-52`).
