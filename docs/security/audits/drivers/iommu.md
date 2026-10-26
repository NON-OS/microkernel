# IOMMU

Kernel DMA and interrupt isolation: VT-d and AMD-Vi, DMAR and IVRS parsing,
and the broker's confine step. Index and notation: README.md.

## kernel iommu: partial

VT-d bring-up, capsule domains and invalidation in ring 0; the AMD-Vi backend is off in the default build (microkernel-core). Paths under src/arch/x86_64/ unless they start with src/.

- **critical** `iommu/unit/bringup/domain.rs:33`: every enumerated PCI function sits in a read-write identity domain over all RAM until a capsule claims it (AMD-Vi: passthrough DTEs), so an undriven or hostile device can rewrite kernel memory and the IOMMU's own tables. -> K2
- **high**, was critical (interrupt spoofing and a kernel crash, no path to memory corruption found) `iommu/remap/enable.rs:46`: interrupt remapping is not built in microkernel-core and keeps CFI set when built (AMD DTEs leave IV clear), so a capsule's device can send any vector, NMI, INIT or SMI to any CPU; vectors 17, 21, 29 and 30 misalign the error-code frame. -> K3, K4
- **critical** `src/hardware/broker/confine/attach.rs:62`: fail open; where no unit translates a device (every AMD machine on the default build, VT-d off or failed, a device outside every scope) the claim and every DMA grant succeed with host-physical addresses. The live branch is the translates() check just above line 62. -> K1
- **medium**, was high (on the success path the window is a few ms before any capsule runs) `iommu/unit/report/init.rs:51`: firmware PMRs and translation are cleared before the kernel's tables exist and never restored when bring-up fails; AMD units firmware left on are stopped even without the AMD backend; the DMAR opt-in flag is never read. -> K1
- **medium**, was high (needs an invalidation failure a capsule cannot normally cause) `iommu/mapping/map_range.rs:70`: a map whose invalidation fails leaves its leaves live while the broker frees the frames; the AMD map has the same shape. -> K1
- **medium** `acpi/parser/other/dmar.rs:97`: a ninth segment-0 DRHD (often the INCLUDE_PCI_ALL unit) is dropped silently and more than 256 functions fails bring-up open, while the posture still reports enforcing. -> K11
- **medium** `iommu/device/map_device.rs:62`: an attach whose invalidation fails leaves the context entry live but drops the binding, so the device shares a reused domain id with another domain and can never be claimed again. -> none
- **medium** `iommu/unit/queue/error.rs:40`: an IQH of 256 or more from a broken or emulated unit indexes past the 256-entry ring and panics the kernel with locks held. -> none
- **medium** `acpi/tables/sdt/header.rs:45`: the DMAR/IVRS checksum reads header.length bytes with no bound, and DRHD register bases are not validated. -> K11
- **medium** `acpi/parser/other/dmar.rs:83`: the DMAR flags, HAW, RMRR, ATSR, SATC and the DRHD register-size field are ignored. -> K11 (RMRR K2, opt-in flag K1)
- **medium** `iommu/unit/fault/drain.rs:51`: faults are only logged, at most 8 lines a poll; a faulting device is never blocked, FECTL is unused, and the AMD event log stops for good after one overflow. -> none
- **medium** `src/hardware/broker/confine/attach.rs:90`: no ACS or isolation-group check before confining, so peer-to-peer writes reach other devices' BARs. -> K11
- **medium** `src/drivers/pci/vmd/registry.rs:56`: PCIe-to-PCI bridge requester-id aliases are not handled, so such a device keeps translating through the identity domain. -> K11
- **medium** `iommu/domain/destroy_domain.rs:46`: destroyed domains leak all their page-table frames; a capsule restart loop exhausts them. -> none
- **medium** `iommu/amd_vi/control.rs:59`: AMD-Vi never sets Control.Coherent or flushes table writes, and never checks EFR.HATS before using Mode 4. -> none (near K4)
- **medium** `iommu/unit/access.rs:28`: a fixed 4 KiB register window makes a unit with registers past 4 KiB disable VT-d entirely, which today fails open. -> K1 makes it fail closed; the window size: none
- **medium** `iommu/mapping/map_range.rs:28`: no tests for page tables, map and unmap, domain lifecycle, queue errors, fault drain or parse_dmar; no DMAR/IVRS fuzz target; no QEMU lane asserting that an unclaimed device faults. -> K1, K2, K4 tests in part; fuzz and lane: none
- low: `iommu/unit/fault/log.rs:34` fault lines print device addresses and unit bases; `iommu/unit/invalidate/iotlb.rs:32` register invalidation without the drain bits; `iommu/tables/walk/create.rs:47` walkers treat a large leaf as a table; `iommu/tables/root/set.rs:40` live tables written through aliasing &mut with plain stores; `iommu/globals/allocate_domain_id.rs:27` domain ids not bounded by CAP.ND; `iommu/unit/queue/enable.rs:58` QIE left set after an enable timeout; `iommu/unit/enable/translation.rs:35` register waits bounded by spin count; `iommu/domain/destroy_domain.rs:39` a destroy that stops part-way can never be retried; `iommu/unit/probe.rs:79` four files over 75 lines.

## Verification

The verifier confirmed both remaining criticals and re-rated the other three
as shown. Its policy became the plan: every requester id starts
blocked, a claim fails where no unit confines the device, firmware DMA
protection stays until the kernel's tables are live, and a frame returns to
the allocator only after an invalidation covering it succeeded. It notes that
docs/overview/threat-model.md:33 ("refuses a claim the unit cannot confine")
overstates today's guarantee. Units: the plan, the plan.
