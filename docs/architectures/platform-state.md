# Platform state: CPU architectures, many cores, GPU

Where NONOS stands on x86_64 SMP, on aarch64, and on a real GPU path, read from the code on `main` at 1536ccef3 (24 October 2026), and the order the work goes in.

## How each claim is marked

| Mark | Meaning |
|---|---|
| **booted** | seen in a QEMU serial log; the page names the log |
| **host-tested** | a proof crate or host test exercises it; nothing has booted it |
| **design** | the code exists and reads as described, but no boot and no test exercises it |
| **absent** | no code |
| **hardware-verified** | seen on a named real machine |

Nothing on this page is booted yet. The SMP soak runs in [the plan](#plan) are what move x86_64 lines to booted. The one hardware report NONOS has is the Gemini Lake laptop in [x86_64](x86_64.md#on-real-hardware). It is single-socket and says nothing about core count, NUMA or the GPU.

## x86_64

### SMP bring-up

- **design.** APs start only in a kernel built with `nonos-smp` (`src/kernel_core/init/start_secondary.rs:21-27`). The flake profiles add it (`tools/nix/config.nix:144`), but the Cargo default features do not, and neither do most `mk/` profiles, `nonos-mk-desktop-gui-prod` among them. `nonos-mk-smp-prod` is the make target that has it (`mk/20-build.mk:1247-1250`).
- **design.** CPUs come from the MADT. Types 0 (LAPIC), 9 (x2APIC) and 10 (x2APIC NMI) are parsed (`src/arch/x86_64/acpi/parser/madt/parse.rs:44-56`), and online-capable entries are honoured on MADT revision 5 and later (`src/arch/x86_64/acpi/hw/madt_cpu.rs:47-52`).
- **design.** `MAX_CPUS` is 256 (`src/smp/constants.rs:17`). Entries past it are counted `over_limit` and logged, not started.
- **design.** APs start one at a time. Each gets INIT, then STARTUP twice, then 1000 ms to enter and 1000 ms to come online (`src/smp/init/ap_unit.rs:38-41`). At 64 CPUs a healthy boot pays 63 serial handshakes. A boot where every AP is dead pays up to 63 × 2 s.
- **design.** One trampoline context slot at `0x8000` is rewritten for each AP (`src/smp/trampoline/install.rs:66-92`). An AP that is late and was not parked (the parking INIT's result is dropped, `src/smp/init/ap_unit.rs:97`) can read the next AP's stack and CPU number before its `boot_claim` CAS (`src/smp/ap/entry.rs:34`).
- **design.** A failed stack allocation or trampoline write for one AP is swallowed by `unwrap_or(false)` and not logged (`src/smp/init/ap_start.rs:59`).
- **design.** AP kernel stacks are 64 KiB, back to back, with no guard page between them (`src/smp/init/stack.rs:22-32`).
- **booted (QEMU q35 TCG `-cpu max`, forced with `nonos-smp-park-proof` and `nonos-smp-stuck-proof`).** An AP that misses its online deadline is asked to park, parks at the next checkpoint of `bring_up`, where it holds no lock, and only then is reset by INIT and set `Offline` (`src/smp/init/ap_unit.rs`, `src/smp/ap/park.rs`). At `-smp 4` and `-smp 64` CPU 1 printed `[SMP] cpu 1 abandoned at bring-up: missed its online deadline; parked and reset by INIT`, the rest came online, and no later round targeted it. An AP that does not park within 100 ms stays `Starting` and a shootdown target: at `-smp 4` CPU 1 printed `stuck at bring-up` and the next kernel shootdown waited for it and stopped the machine at its 2 s deadline, as designed. Before this, such an AP ran kernel code unflushed. [Shootdown](#tlb-shootdown) has the targeting.
- **booted, QEMU q35 TCG `-cpu max`, 24 October 2026.** `microkernel-capsules` with `nonos-smp`, at 1536ccef3 and again with the shootdown and park changes: `-smp 1` reports `UP`; `-smp 2`, 4, 16 and 64 report `[SMP-PROOF] cpu_count=N PASS` with every AP online (`answered=63 online=64`). No run has a FATAL, PANIC, #GP or #PF line, and the first userland spawn comes at the same uptime with and without the changes (5004 to 5012 at up to 4 CPUs, 6007 to 6010 at 16, 14031 to 14032 at 64). The boot matrix also has `q35-smp2`, `q35-smp4`, `q35-smp8`, `i440fx-smp4` and `q35-iommu-smp4` cells (`scripts/bootmatrix/cells.py:50-63`), held to the same line; that workflow runs daily and is not a merge gate.

### Per-CPU data

- **design.** `PERCPU_DATA` is a static array of 256 blocks of 4 KiB, indexed by logical CPU number. GS_BASE points at the block in kernel mode (`src/smp/percpu/operations.rs:22-25`, `src/arch/percpu_base.rs:32-42`). Fixed offsets are asserted at build time (`src/smp/percpu/layout.rs:33-42`).
- **design.** `cpu_id()` on x86_64 does not read GS. It runs CPUID for the APIC id, then scans the CPU table linearly (`src/arch/x86_64/abi.rs:64-79`, `src/smp/cpu.rs:25-27`). The reason is documented: several vectors still enter from user mode without `swapgs` (`src/smp/cpu_id.rs:24-43`). The cost is a CPUID, a VM exit under KVM, plus O(n) work on the syscall, tick and switch paths, and it grows with core count. A GS-relative reader exists (`src/arch/percpu_id.rs`) but is not compiled (`src/arch/mod.rs:41`).
- **design.** Every per-CPU table is indexed by logical number, not APIC id, so sparse APIC ids are fine. The exception is the no-MADT fallback, which assumes dense ids `0..n` (`src/smp/topology/detection/probe_x86_64/mod.rs:77-79`).
- **design.** `CPU_INFO` is never written (`src/smp/topology/detection/state.rs:28`). As a result `cpu_to_numa_node` always answers 0 and `cpus_share_cache` always answers false.

### APIC and x2APIC

- **design.** x2APIC is entered only when firmware already set EXTD, or when the CPU has x2APIC and the MADT names an id above `0xFE` (`src/sys/apic/local/x2apic.rs:46-60`). APs copy the boot CPU's mode. With x2APIC the ICR is MSR `0x830`. With xAPIC it is MMIO, with a 100 ms idle wait, and destinations above `0xFE` are refused (`src/arch/x86_64/interrupt/apic/plan.rs:85-91`). Every IPI uses physical destination mode.
- **design.** Device interrupt destinations are held to `0xFE` or below (`src/arch/x86_64/interrupt/apic/irq_dest.rs`). Interrupts on a machine with CPUs above `0xFE` all land on CPUs with low ids. That is correct, but it is not spread.

### TLB shootdown

- **design.** One round runs at a time under `SHOOTDOWN_LOCK`, with interrupts masked. The request is a single global VA, page count and ack counter. Targets are every online or starting CPU for a kernel mapping, or the online CPUs whose `active_asid` matches for a user mapping; a starting target gets no vector and serves its own mark. Each target gets a pending flag and vector `0x40` (`src/memory/paging/manager/shootdown/broadcast.rs:26-70`, `select.rs:25-65`, `send.rs:32-50`).
- **design.** The handler runs `invlpg` per page, or the full flush when the page count is 0, then acks (`handle.rs:60-75`). The full flush (`src/arch/paging/tlb.rs`, `invalidate_all`) clears CR4.PGE and sets it back with interrupts masked, then rewrites CR3, so GLOBAL kernel entries go too. Pending rounds are also served from the NMI path, from `lock_responsive` and from usercopy every 64 KiB.
- **design.** At 50 ms the CPUs still pending get an NMI. At 2000 ms the machine stops (`wait.rs:34-64`). A failed `send_ipi` is dropped and recovered only by that 50 ms NMI (`send.rs:48`).
- **design.** PCID and INVPCID are detected but never enabled. `enable_pcid` is reached only from `process::address_space::init`, which nothing calls (`src/process/address_space/api.rs:42-47`). Every CR3 write therefore flushes every non-GLOBAL entry, which is every user entry, and that is the only reason the `active_asid` filter is sound; CR4.PGE (below) keeps only kernel entries across it. A CPU that switched away from an address space holds no tagged entries for it. Turning PCID on without changing target selection would make shootdown unsound.
- **booted.** Kernel PTEs carry GLOBAL (`src/memory/paging/manager/api/mapping/map_kernel_page.rs:23`), and no user PTE does: the only GLOBAL mappings are that, the IOAPIC and xAPIC windows and the kernel's layout sections (by audit, not enforced by the encoder). The kernel now sets CR4.PGE itself on every CPU where CPUID leaf 1 offers it, in the same bring-up that sets SMEP, SMAP and UMIP (`src/memory/mmu/mmu/protect/cr4/pge.rs`), reports it as a read-back in `ProtectionFlags::pge_enabled`, and an AP whose `pge=` differs from the boot CPU's fails `matches_boot` (`protect/per_cpu.rs`). Booted under OVMF at `-smp 1`, 2, 4, 16 and 64: every `[CPU-PROT]` line reads `pge=1`, every AP online, no FATAL. `nonos-smp-pge-proof`, a boot proof in no profile, sets PGE on the boot CPU alone before the kernel looks, as firmware may (`[PGE-PROOF] boot cpu pge forced before handoff, was=0 now=1`); at `-smp 4` and 64 every CPU still ends `pge=1`. `shootdown_proofs` takes PGE per CPU (off, boot CPU only, every CPU): a full flush that is a bare CR3 write leaves the stale entry wherever PGE is set (mutant, caught for both), and the toggling flush holds for all three. Not booted: a full-flush round (`pages=0`). The trace boots sent only 1- and 16-page rounds, so the toggle runs in the host model, not yet in a booted shootdown.
- **host-tested and booted.** `userland/shootdown_proofs` searches every interleaving of one round with the CPUs it must reach and the APs that come up, park or are abandoned during it: every configuration up to 4 CPUs, 5 CPUs for the kernel's rules, the park path up to 3. Seven mutants are each caught. With `nonos-shootdown-trace`, each AP's bring-up at `-smp 64` sent 8 kernel rounds, each reaching every other online CPU but the abandoned CPU 1 (target set `…FD`), and each returned with every acknowledgement. No boot in this set made a user-address-space round, so the asid filter is not booted, and no host test covers it.

### Scheduler

- **design.** There is one machine-wide run queue, `PID_RUN_QUEUE`, a `Mutex<VecDeque<u32>>` (`src/process/scheduler/dispatch/run_queue.rs:39-95`). It has five strict priority bands, with round robin by pid inside a band. There are no per-CPU queues, no balancing, no affinity and no aging. A band holding more ready work than there are CPUs starves the bands below it. The tick preempts only user mode. [Scheduler and SMP](../kernel/scheduler-and-smp.md) has the detail.
- **host-tested.** `band_choice` alone is compiled into `kernel_proofs` (`userland/kernel_proofs/src/sched_pick/mod.rs`). No fairness property is stated or tested anywhere.
- At 64 CPUs every pick, wake and sleep takes the same lock. It will be the first scaling wall after `cpu_id()`.

### NUMA

- **design.** SRAT types 0, 1, 2, 3 and 5, SLIT and HMAT are parsed (`src/arch/x86_64/acpi/parser/other/srat.rs`, `slit_matrix.rs`, `hmat_load.rs`). The only consumer is the device-domain lookup in `src/hardware/broker/acpi/numa_store.rs`.
- **absent.** Node-aware memory allocation and node-aware scheduling. `CpuDescriptor.numa_node` is always 0 (`src/smp/types.rs:54`).

### CPU protection features

| Feature | Detected | Boot CPU | APs | Mark |
|---|---|---|---|---|
| NX | yes | EFER.NXE; boot stops without it | trampoline sets it | design |
| SMEP, SMAP, UMIP | `src/memory/mmu/mmu/protect/cpuid.rs:36-45` | CR4, `protect/cr4.rs:32-51` | set again; an AP that differs runs no user code (`src/smp/ap/user_setup.rs:41-58`) | design |
| WP | n/a | CR0 | n/a | design |
| PCID, INVPCID | yes | never enabled (see shootdown) | n/a | design |
| FSGSBASE | yes | never enabled | n/a | design |
| CET shadow stack, IBT | yes | no CR4.CET, no `S_CET` or `U_CET` write | n/a | design |
| PKU | yes | never enabled | n/a | design |
| PKS | absent | | | absent |

Usercopy does not run inside a STAC and CLAC window. It walks the user page table in software and copies through the direct map (`src/usercopy/direct.rs:25-44`), so SMAP is never relaxed for a copy. `with_user_access` exists, but it is gated on CPUID rather than on CR4 (`src/arch/user_access/scope.rs:23-52`).

## aarch64

### What boots in QEMU virt

- **booted, 24 October 2026, QEMU 11.1 `virt` with GICv3 and `-cpu max`, 1 CPU.** The aarch64 CI boot job runs only the two trap cells, `trap-sp0` and `trap-kernel-abort` (`.github/workflows/ci-boot-aarch64.yml:47-52`). The `core` cell named in the job's header is not in its matrix, and neither are `-smp` or `iommu=smmuv3`.
  - `trap-sp0` passes.
  - `trap-kernel-abort` fails on 1536ccef3. Every boot that returned from `boot::init` took a pointer-authentication failure (EC 0x1C), because `init` signed its return address with the MMU off and authenticated it after `init_mmu`. With the key installed after `init` returns (`src/arch/aarch64/boot/entry.rs`), the cell passes.
  - The `core` image then reaches `[NONOS] Core ready` and stops at the bootloader check (next item).
- **design.** The boot sequence:
  1. An arm64 Image header carries the device tree in `x0` and an optional NONOS handoff in `x1`.
  2. EL2 is dropped to EL1, and the kernel relocates itself.
  3. The MMU comes up with 4K granules and 48-bit TTBR0 and TTBR1, and the kernel moves to the high half.
  4. The GIC (v2 or v3, chosen from the tree), the generic timer at 10 ms and PSCI or spin-table secondaries come up.
  5. Then the shared `microkernel_init` and `microkernel_main` run (`src/arch/aarch64/asm/start.S`, `src/arch/aarch64/boot/{entry,init,finish,core_entry}.rs`).
- **booted.** The boot stops at the bootloader check. With `-kernel` there is no loader evidence, and `refuse_unchecked_loader` starts no userspace (`src/kernel_core/init/entry/loader_refusal.rs:37-48`). No aarch64 boot reaches `[UKERNEL] Entering userspace`, a capsule or the desktop.
- **booted (QEMU `virt` GICv3 `-cpu max`, `-smp` 2/4/8: `online=N of N`, no refusal), 24 October 2026; no secondary-scheduled work yet, no hardware.** Every secondary checks in: the boot prints `[SMP] online=8 of 8` at `-smp 8`. After 30 s each secondary is in `idle_states::wait` or its IRQ vector, taking its own tick. Before this, each secondary refused at `secondary timer tick`: the timer's PPI is banked per CPU, its one handler slot was filled by the boot CPU, and registering it again failed as already claimed. A secondary now checks the boot CPU's registration and enables only its own PPI (`timer::arm_on_secondary`). Nothing yet runs work on the secondaries past the bootloader check, so the scheduler on them has not been booted.
- **design.** Parking reads Aff0 only (`MPIDR & 0xFF`, `start.S:51-53`), so a core in cluster 1 with Aff0 of 0 would take the boot CPU's path.
- **booted (QEMU `virt` `-cpu max`, `trap-sp0` and `trap-kernel-abort`), host-tested (`arch_paging_proofs`, 44 tests); FEAT_TTL silicon not run.** `tlbi vaae1is` got `va >> 12` unmasked (`src/arch/aarch64/mmu/tlb.rs:25-29`, `src/arch/aarch64/abi/tlb.rs:22-31`). For a kernel address, bits [47:44] of the operand become `0xF`. A CPU with FEAT_TTL (Armv8.4 and later) reads those bits as a level hint: 64K granule, level 3. NONOS maps with 4K granules, and the architecture does not require a TLBI with a wrong hint to invalidate anything. Both flush paths now take their operand from `tlbi_operand::vaae1is`, which masks to bits [43:0] and leaves TTL at 0, "no hint" (`src/arch/aarch64/mmu/tlbi_operand.rs`). `arch_paging_proofs` holds that encoding to the architecture's layout. QEMU TCG would not show the bug, and neither would the Pi 5's Cortex-A76 (Armv8.2, no FEAT_TTL).

### What exists for real boards

| Piece | State |
|---|---|
| GICv2 and GICv3, ITS, GICv2m, SGIs for IPIs | design; the GIC init no longer requires v3, so [aarch64](aarch64.md) is stale there |
| PSCI over HVC or SMC, spin table | design |
| Per-CPU base in TPIDR_EL1, a single writer enforced by static check | design |
| Broadcast TLBI (`vmalle1is`, `vaae1is`, `aside1is`) | booted on one CPU; the `vaae1is` operand is host-tested; FEAT_TTL silicon not run |
| SMMUv3 (stage 1, two-level stream table, abort by default) | host-tested (`smmu_proofs`); no QEMU lane gives `iommu=smmuv3` |
| SMMUv2 | absent; IORT parses its nodes, but there is no driver |
| Device tree: memory, GIC, timer, PCIe ECAM, PL011, PL031, SMMU, `iommu-map`, `msi-map`, PSCI, CPUs, reserved memory, TPM | design |
| ACPI on ARM: MADT GICC, GTDT, SPCR, IORT, PPTT, SRAT, SLIT, HMAT, DBG2 | host-tested parsers; nothing calls them, and an ACPI-only boot is refused (`src/arch/aarch64/boot/dtb_adapter/parse/acpi.rs:30-39`) |
| UEFI loader for aarch64 | absent; `nonos-bootloader` is x86_64-only in its toolchain file, `arch` module, ELF machine check, page-table builder and jump |
| Kernel side of the loader handoff in `x1` | host-tested (`src/arch/aarch64/boot/loader/`) |
| Display | the GOP framebuffer through a handoff (no loader exists to supply one) or the Raspberry Pi mailbox framebuffer; no ramfb, and `simple-framebuffer` is found in the tree but nothing uses it |
| virtio-gpu capsule for `aarch64-nonos-user` | builds in CI; never booted |
| Desktop input | `microkernel-desktop-base` lacks `driver-virtio-input`, so the arm GUI target has no keyboard or pointer on `virt` (`Cargo.toml:933-948`) |
| std-platform capsules | do not build: the std layer issues x86_64 syscalls (`mk/20-build.mk:1158-1170`) |
| Kernel stack guard pages | absent; `arm_stack_guards` exists only on x86_64 |
| Power, reboot, sleep | PSCI `SYSTEM_OFF` and `SYSTEM_RESET` only; no ACPI power-button equivalent |

### Dead-code warnings

None: `--features nonos-arch-preview` and `--no-default-features --features
microkernel-core,nonos-arch-preview` check with 0 warnings. The items whose only
consumer was x86_64 code were answered without `allow`:

- `zerostate::terminate_critical` and `CRITICAL_TOTAL_MS`: one `terminate_with(off, critical)`
  (`src/security/zerostate/terminate.rs`), which `terminate` calls with `false`; only the
  ACPI broker's critical watch passes `true`, as only ACPI reports a critical zone or battery.
- `device_secret_caller`: its re-export is gone; the x86 seal read imports it from
  `device_proof::gate`.
- `dev_unremapped::holds`: gone; the x86 MSI composer (`src/hardware/broker/irq/bind/message.rs`)
  reads the opt-in itself, as only x86 composes a compatibility format message.

### What a real board needs

A desktop on a UEFI Arm board, whether a Raspberry Pi 5 with community EDK2 firmware, an Ampere box or a SystemReady board, needs these pieces, in dependency order:

1. **An aarch64 UEFI loader.** The `nonos-bootloader` build for `aarch64-unknown-uefi` needs:
   - the x86 port I/O, CPUID, TSC, CR3 and `global_asm!` sites put behind the architecture boundary;
   - an AArch64 TTBR0 and TTBR1 table builder in place of the PML4 builder;
   - an `EM_AARCH64` ELF check;
   - a jump that cleans the handoff to the point of coherency, disables the MMU and enters with `x0` set to the device tree and `x1` set to the handoff.

   It must hand over either the firmware's device tree (EFI DTB configuration table) with `/reserved-memory` covering the handoff and the modules, or an ACPI RSDP.
2. **ACPI boot on the kernel side.** SystemReady boards give ACPI, not a device tree. The parsers exist; `dtb_adapter` needs an ACPI twin that fills `BootInfo` from MADT GICC, GTDT, SPCR, IORT and the EFI memory map.
3. **GOP framebuffer handoff on ARM.** The record is architecture-neutral. Only `tsc_hz` and the SMEP, SMAP and UMIP flag bits mean something only on x86, and an ARM handoff must mark them as not applicable rather than zero.
4. **SMMU coverage.** SMMUv3 for server-class boards. The Pi 5 has no SMMU in front of PCIe, so the fail-closed broker refuses every DMA device there. That is the correct posture, and the page must say so rather than weaken it.
5. **Pi 5 specifics.** The RP1 south bridge sits behind PCIe and carries Ethernet, USB and GPIO. Its drivers are absent. Display needs either the firmware's GOP or a VC7 KMS capsule, which is absent.

## GPU

### What exists

- **design.** No log in the tree shows a desktop frame at this commit. Letter `the design notes` reports two boots. On the GOP path the splash showed once and froze. On the virtio-gpu path the screen stayed black. The build work owns that fix. The display path is entirely capsules: compositor, window manager, shell, and one driver capsule per device (`virtio_gpu`, `vmware_svga`, `bga`, `intel_display`, `hv_synthvid`). The kernel keeps only the firmware framebuffer mapping, the boot log and panic screen, and the `MkSurfacePresent` blit used when no driver is up (`src/syscall/dispatch/router/graphics_present/blit.rs:22-67`).
- **design.** Composition is all CPU. The compositor sends FLUSH with damage rectangles, and the driver copies them into its backing store, then does `TRANSFER_TO_HOST_2D` and `RESOURCE_FLUSH` (`userland/capsule_driver_virtio_gpu/src/display/show.rs:37-58`).
- **design.** virtio-gpu offers `VIRGL`. The capsule runs one 3D probe, creates a context, clears and reads back, then tears down. It logs the result and draws nothing through 3D (`userland/capsule_driver_virtio_gpu/src/setup/probes.rs:53-55`). It polls and holds no interrupt.
- **design.** `intel_display` covers display only: takeover on Haswell to Raptor Lake, modeset on Gen9, GGTT, planes, cursor, DP and HDMI. It has no render or GT engine and no GuC or HuC, and it has never booted, since QEMU has no Intel graphics. AMD has a register-document survey (`the design notes`) and no code. NVIDIA is enumerate-only.
- **design.** DMA confinement is the strongest piece:
  - every claim attaches the capsule's own IOMMU domain before the device is powered, and IOVAs map only the capsule's grants (`src/hardware/broker/claim/claim.rs:56-57`, `confine/capsule.rs:33-64`);
  - a device no IOMMU unit translates is refused, and on x86 a claim without interrupt remapping is refused;
  - a virtio device must offer `ACCESS_PLATFORM`.

  VT-d is on by default and blocks faulting sources. AMD-Vi has never booted while enforcing. A QEMU boot with no vIOMMU therefore refuses the virtio-gpu claim and falls back to the firmware framebuffer. That is fail-closed, and it means a GPU lane must always boot with `intel-iommu`.

### What a real GPU path needs

In order, each step inside a capsule, none in the kernel:

1. **UEFI GOP.** It works today as the fallback. It is unaccelerated, and on most real machines it is one fixed mode.
2. **virtio-gpu 3D under QEMU (`virtio-vga-gl`, virglrenderer on the host).** The pieces, in order:
   - Turn the 3D probe into a context per client capsule.
   - Give each context resources: `RESOURCE_CREATE_3D` and `ATTACH_BACKING`, backed by pages from the capsule's IOMMU domain only.
   - Pass client command streams to `SUBMIT_3D` through a validated virgl command parser, which is fuzzed. The host is also hardened, but the capsule must not forward a stream it did not check.
   - Track fences by interrupt rather than by polling.
   - Let the compositor import a client's render target as a scanout source.

   The command parser and the per-context resource table are the new untrusted-input surface. Each gets a proof crate, a mutant, and a fuzz target.
3. **Intel modeset in a capsule on real hardware.** `intel_display` booted on a Gen9 to Gen12 machine with VT-d enforcing. It must survive takeover from GOP without a black screen, and recover on hotplug.
4. **Intel command submission.** A separate capsule from display. Its pieces:
   - GuC firmware loading, with a signed blob that is versioned and measured;
   - per-context PPGTT, where the GPU's own page tables map only that client's grants, layered under the IOMMU domain;
   - execlists or GuC submission;
   - a batch-buffer validator;
   - a reset and hang path that cannot reach outside the capsule.

   This is the larger task. Nothing exists for it.

The hard rule through all of it: the device's DMA is confined by the IOMMU to the capsule's own domain. Where a machine has no IOMMU in front of the GPU, the capsule gets no DMA at all.

## Plan


1. **x86_64 SMP at high core counts.**
   - Done: shootdown targets `Starting` CPUs; an AP past its online deadline is parked cooperatively and only then reset; `shootdown_proofs` with its mutants; boots at `-smp 1`, 2, 4, 16 and 64 with every AP online and no FATAL (above).
   - Done (P-02): CR4.PGE set on every CPU and checked equal to the boot CPU's; the full flush toggles it (above). Open: a booted full-flush round.
   - Give each AP its own trampoline slot. Log every AP setup failure, and put a guard page between AP stacks.
   - Make `cpu_id()` O(1).
   - State the fairness property of the scheduler, then test it.
   - A user-address-space shootdown in a boot and in a host test, for the asid filter.
2. **aarch64 to a booted desktop on QEMU virt.**
   - The aarch64 loader.
   - `driver-virtio-input` in the arm desktop profile.
   - The `core` cell back in CI, and an `-smp 4` cell and an `iommu=smmuv3` cell.
   - Then the real-board path documented against a Pi 5 with EDK2.
3. **The GPU capsule.** virtio-gpu 3D contexts with a fuzzed virgl validator, then Intel modeset booted on hardware, every lane booted with the IOMMU enforcing.

## See also

- [x86_64](x86_64.md)
- [aarch64](aarch64.md)
- [Scheduler and SMP](../kernel/scheduler-and-smp.md)
- [IOMMU](../kernel/iommu.md)
- [Hardware support matrix](../hardware/MATRIX.md)
