# IOMMU

Kernel DMA and interrupt isolation: VT-d and AMD-Vi, DMAR and IVRS parsing,
and the broker's confine step. Index and notation: README.md.

## kernel iommu: partial

VT-d bring-up, capsule domains and invalidation in ring 0; the AMD-Vi backend is off in the default build (microkernel-core). Paths under src/arch/x86_64/ unless they start with src/.

- **medium** `iommu/device/map_device.rs:62`: an attach whose invalidation fails leaves the context entry live but drops the binding, so the device shares a reused domain id with another domain and can never be claimed again. -> none
- **medium** `iommu/unit/queue/error.rs:40`: an IQH of 256 or more from a broken or emulated unit indexes past the 256-entry ring and panics the kernel with locks held. -> none
- **medium** `iommu/unit/fault/drain.rs:51`: faults are only logged, at most 8 lines a poll; a faulting device is never blocked, FECTL is unused, and the AMD event log stops for good after one overflow. -> none
- **medium** `iommu/domain/destroy_domain.rs:46`: destroyed domains leak all their page-table frames; a capsule restart loop exhausts them. -> none
- low: `iommu/unit/fault/log.rs:34` fault lines print device addresses and unit bases; `iommu/unit/invalidate/iotlb.rs:32` register invalidation without the drain bits; `iommu/tables/walk/create.rs:47` walkers treat a large leaf as a table; `iommu/tables/root/set.rs:40` live tables written through aliasing &mut with plain stores; `iommu/globals/allocate_domain_id.rs:27` domain ids not bounded by CAP.ND; `iommu/unit/queue/enable.rs:58` QIE left set after an enable timeout; `iommu/unit/enable/translation.rs:35` register waits bounded by spin count; `iommu/domain/destroy_domain.rs:39` a destroy that stops part-way can never be retried; `iommu/unit/probe.rs:79` four files over 75 lines.

## Verification

The verifier confirmed both remaining criticals and re-rated the other three
as shown. Its policy became the plan: every requester id starts
blocked, a claim fails where no unit confines the device, firmware DMA
protection stays until the kernel's tables are live, and a frame returns to
the allocator only after an invalidation covering it succeeded. It notes that
docs/overview/threat-model.md:33 ("refuses a claim the unit cannot confine")
overstates today's guarantee. Units: the plan, the plan.
