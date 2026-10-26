# aarch64

What exists of the aarch64 port of NONOS, how to build it and boot it under QEMU, and what is missing before it is a system you can install.

## Status

aarch64 is a preview. The kernel builds only with the `nonos-arch-preview` feature, which `compile_error!` demands for every architecture but x86_64 (`src/lib.rs:28-35`). It targets QEMU's `virt` board. The NONOS UEFI loader builds for it (below), but no release image carries it, and it has not been tested on real hardware in this release. CI builds it and boot-tests it on every pull request: `ci.yml` runs on each `pull_request` and calls both aarch64 workflows (`.github/workflows/ci.yml:7-35`). Whether those jobs pass at this commit is not recorded here.

The missing pieces below are read from the code. No page here gives a date for them.

## Build and boot under QEMU

```
make nonos-mk-arm
make nonos-mk-arm-run
```

Not tested in this release.

- Outside the flake's shell, a `nonos-mk-*` target enters `nix develop` first, so these need nothing but Nix (`NONOS_IN_FLAKE`, `Makefile:144-147`).
- `nonos-mk-arm` builds the kernel with `ARM_KERNEL_BUILD_FLAGS` and the `microkernel-core` and `nonos-arch-preview` features, and sets `NONOS_USER_TARGET` so the kernel embeds [capsules](../overview/glossary-boot-root-record.md#capsule) built for `aarch64-nonos-user` (`mk/20-build.mk:856-862`).
- The build signs the embedded manifest with an Ed25519 seed. When the file named by `SIGNING_KEY` does not exist, make writes 32 random bytes there first (`mk/10-qemu.mk:163-167`). That seed is for local images only.
- `nonos-mk-arm-run` boots the ELF with `ARM_QEMU_FLAGS`: `-M virt,gic-version=3 -cpu max -m 512 -nographic -serial mon:stdio -device virtio-rng-pci` (`mk/20-build.mk:885-901`). The comment above it gives the reasons: the kernel drives a GICv3 and `virt` defaults to a GICv2, `-cpu max` makes the feature-guarded paths run, and the RNG refuses to seed without an entropy device.

The kernel lands at `target/aarch64-nonos/release/nonos-kernel`. `tools/arm_kernel_report.py` reads it without a cross toolchain and prints its ELF header, segments, section sizes, symbols, the entry path from `_start` to `microkernel_main`, and the capsules built for `aarch64-nonos-user` (`report_entry_path`, `tools/arm_kernel_report.py:284-299`).

```
python3 tools/arm_kernel_report.py --fast
```

With no kernel built, it says so, prints `build one with: make nonos-mk-arm` and exits with status 1 (`KERNEL`, `tools/arm_kernel_report.py:370-374`).

### The desktop

`nonos-mk-arm-gui` builds the desktop base capsules for `aarch64-nonos-user` and a kernel with `microkernel-desktop-base` (`NONOS_USER_TARGET`, `mk/20-build.mk:1171-1182`). `nonos-mk-arm-gui-run` boots it with `ARM_GUI_QEMU_FLAGS`: a virtio GPU, keyboard, tablet and RNG, and 2048 MB (`mk/20-build.mk:926-937`). The kernel gets no device tree on this path either, so it uses its 512 MB default rather than the 2048 MB QEMU provides; the list at the end of this page says why. No CI job boots the desktop, and this release has not tested it.

```
make nonos-mk-arm-gui-run
```

Not tested in this release.

## What CI checks

- `ci-build-aarch64.yml` builds through `nonos-mk-arm` and checks the result is a loadable AArch64 kernel: a static executable whose entry is `_start`, the entry in an executable `PT_LOAD`, no segment both writable and executable, and the manifest and signature sections present (`.github/workflows/ci-build-aarch64.yml:3-12`).
- `ci-boot-aarch64.yml` boots three images under QEMU `virt` with a GICv3 and `-cpu max`, emulated on an x86_64 runner, and holds each serial log to `scripts/check_aarch64_boot.py` (`.github/workflows/ci-boot-aarch64.yml:3-20`).
- The `core` image must print the `CORE_MARKERS` in order, `[KSEC] 4/4 sections mapped as declared`, `[NONOS] Core ready` and the two lines of the stop described under [Where a boot stops today](#where-a-boot-stops-today), and none of the `NEVER` lines (`scripts/check_aarch64_boot.py:63-69`). `NEVER` includes `[UKERNEL] Entering userspace` and `[INIT] Starting`: a boot without a loader that started userspace would have skipped the check of the bootloader.
- The `trap-sp0` and `trap-kernel-abort` images are built with the `nonos-trap-proof-sp0` and `nonos-trap-proof-kernel-abort` features, each of which compiles one deliberate exception into the boot path, `sp_el0_vector` or `kernel_data_abort`; no profile enables either (`src/arch/aarch64/boot/trap_proof.rs:17-59`). Each must print the `[TRAP]` line the architecture defines for it, such as `SP0_ESR` 0xF2005350 for the breakpoint that `BRK_5350` encodes (`scripts/check_aarch64_boot.py:59-63`).

The checker tests itself without QEMU:

```
python3 scripts/check_aarch64_boot.py --self-test
```

On this tree it answers that 19 bad logs were rejected and 4 good ones accepted.

## Boot path

```mermaid
flowchart LR
    S[_start] --> E[kernel_entry]
    E --> D[dtb_adapter]
    D --> I[init]
    I --> H[KernelHandoff]
    H --> K[microkernel_init]
    K --> M[microkernel_main]
```

- `_start` drops from EL2 to EL1 when entered at EL2, resets SCTLR_EL1 with the MMU and caches off, parks every core but the first in `wfe`, clears BSS, turns on FP and SIMD in CPACR_EL1, installs VBAR_EL1 and calls `kernel_entry` with the device tree pointer from x0 (`src/arch/aarch64/asm/start.S:13-77`).
- `kernel_entry` opens the PL011 console on its default base, fills `BootInfo` through `dtb_adapter`, runs `init`, and says on the console when it found no usable device tree and assumed QEMU `virt` (`src/arch/aarch64/boot/entry.rs:32-69`).
- It then arms the bootstrap heap and builds a `KernelHandoff`, so from `microkernel_init` and `microkernel_main` on the kernel runs the same code x86_64 runs (`src/arch/aarch64/boot/entry.rs:76-80`). [Boot handoff](../kernel/boot-handoff.md) explains the [handoff](../overview/glossary-amnesic-boot.md#handoff).
- `init` brings up the console and the CPU, latches the CPU list, publishes the PCI windows and the RTC base, installs the vectors, runs `security::init_all`, `init_mmu`, `init_gic` and the timer, starts the other cores, and unmasks interrupts last (`src/arch/aarch64/boot/init.rs:20-81`).

`init` stops the boot through `refuse` when `security::init_all` fails (`src/arch/aarch64/boot/init.rs:53-55`), when the device tree names a GIC other than v3 (`src/arch/aarch64/boot/init.rs:57-59`) and when the timer tick cannot be installed, for example with no timer interrupt id (`install_on_cpu`, `src/arch/aarch64/timer/preemption/install.rs:23-32`). `security::init_all` turns on pointer authentication, BTI, memory tagging and the speculation mitigations (`init_pac`, `src/arch/aarch64/security/init_all.rs:22-28`), each feature only where the ID registers report it, as `has_feature` does for pointer authentication (`src/arch/aarch64/security/pac/init.rs:24-27`). Each secondary core runs `security::init_all` again as it starts, and stops the same way when it fails (`aarch64_ap_entry`, `src/arch/aarch64/boot/multicore/ap_entry.rs:30-35`). The speculation barrier runs on every CPU whatever the ID registers say: `sb` where the CPU has it, `dsb sy` then `isb` where it does not (`speculative_barrier`, `src/arch/aarch64/security/spectre/barrier.rs:33-46`).

The generic timer ticks every 10 ms and each tick calls the shared scheduler's `tick` (`TICK_PERIOD_NS`, `src/arch/aarch64/timer/preemption/handler.rs:20-25`). Page descriptors keep execution from crossing privilege: `execute_never` sets PXN on every user page and UXN on every kernel page, whatever the caller asked (`src/arch/paging/descriptor/aarch64/build.rs:53-63`).

## What exists

`src/arch/aarch64` holds 292 files and 12508 lines, headers included.

| Module | Size | What it holds |
|---|---|---|
| `abi` (`src/arch/aarch64/mod.rs:17`) | 10 files, 306 lines | the `Aarch64` implementation of `ArchOps` |
| `asm` (`src/arch/aarch64/mod.rs:18`) | 10 files, 638 lines | `start.S`, the vector table, user entry and resume, FP and SIMD save and restore |
| `boot` (`src/arch/aarch64/mod.rs:19`) | 26 files, 1474 lines | entry, the device tree adapter, `BootInfo`, the memory map, secondary cores, PCI windows, refusals, trap proofs |
| `context` (`src/arch/aarch64/mod.rs:21`) | 14 files, 719 lines | first entry to EL0, saving and resuming user frames, the kernel-side switch context |
| `cpu` (`src/arch/aarch64/mod.rs:22`) | 23 files, 994 lines | barriers, feature registers, MPIDR affinity, interrupt masking, wait for event |
| `cpu_random` (`src/arch/aarch64/mod.rs:23`) | 3 files, 136 lines | RNDR and RNDRRS from FEAT_RNG |
| `exceptions` (`src/arch/aarch64/mod.rs:24`) | 28 files, 1293 lines | the vector install, exception frames, syndrome decoding, the trap contract, fatal paths |
| `fpu` (`src/arch/aarch64/mod.rs:25`) | 10 files, 349 lines | lazy FP and SIMD state per task |
| `gic` (`src/arch/aarch64/mod.rs:26`) | 38 files, 1371 lines | GICv3 distributor, redistributors, ICC registers, SGIs, interrupt handlers including ones bound for capsules |
| `interrupt_controller` (`src/arch/aarch64/mod.rs:27`) | 3 files, 104 lines | the shared IPI interface, carried on GIC SGIs |
| `mmu` (`src/arch/aarch64/mod.rs:28`) | 35 files, 1825 lines | MAIR, TCR and SCTLR, the boot map, the image map, tables, TLB and TTBR |
| `psci` (`src/arch/aarch64/mod.rs:29`) | 18 files, 726 lines | PSCI calls: CPU on and off, suspend, system off and reset |
| `rtc` (`src/arch/aarch64/mod.rs:30`) | 3 files, 92 lines | the PL031 real-time clock |
| `security` (`src/arch/aarch64/mod.rs:31`) | 29 files, 1097 lines | pointer authentication, BTI, MTE, speculation barriers and SSBS |
| `timer` (`src/arch/aarch64/mod.rs:32`) | 29 files, 928 lines | the generic timer, physical and virtual, deadlines, delays, the preemption tick |
| `uart` (`src/arch/aarch64/mod.rs:33`) | 11 files, 393 lines | the PL011 console |

`aarch64-nonos.json` keeps `cpu` at `generic`, so ordinary code is ARMv8.0, and its `features` add pointer authentication, the speculation barriers, memory tagging and the RNG for the instructions the kernel writes by hand (`aarch64-nonos.json:20-21`). The note on `aarch64-nonos.json` in `mk/20-build.mk` gives the reason for `generic`: naming an architecture version instead would let LLVM put newer instructions into ordinary code, and every hand-written use sits behind an ID register check (`mk/20-build.mk:836-843`). `linker_aarch64.ld` links the image at `0x40080000` on the `virt` board and starts writable data on a 2 MB boundary, so the boot map's 2 MB blocks can keep text read only and executable (`__kernel_rw_start`, `linker_aarch64.ld:18-41`).

## Where a boot stops today

A boot started without a NONOS loader, as QEMU's `-kernel` starts it, comes up to the kernel's check of the bootloader and stops there. `check_bootloader` needs the loader's boot-root record and boot trailer, and either a TCG log with a TPM to read or the loader's own image (`decide`, `src/security/boot/loader_check/run.rs:30-55`). With no loader there is none of them, so the verdict is `NoEvidence`, and `refuse_unchecked_loader` starts no userspace (`src/kernel_core/init/entry/loader_refusal.rs:27-50`). The serial log then ends with:

```
[BOOT-ATTEST] bootloader not checked: no boot-root record or trailer
[ERROR] bootloader not admitted by the kernel's check; stopping
```

The check is never skipped on aarch64. A boot reaches userspace only when a loader hands the kernel its evidence.

## Booting from the NONOS loader

`nonos-bootloader` builds for `aarch64-unknown-uefi`. The loader verifies the kernel as it does on x86_64, then leaves UEFI by the arm64 boot protocol:
- It takes the device tree from the UEFI configuration table (`device_tree`, `nonos-bootloader/src/arch/aarch64/dtb.rs`). Firmware that publishes ACPI and no tree, as AAVMF does on `virt` unless started with `acpi=off`, is refused: the loader says why on the console and halts, since the kernel takes its platform from the tree (`refuse_no_tree`, `nonos-bootloader/src/handoff/exit/no_tree.rs`).
- It places the kernel, a self-relocating ET_DYN, in a 2 MiB aligned block at its link offset, across the size its arm64 Image header claims, so the boot stack past the last segment is the kernel's (`place_image`, `nonos-bootloader/src/loader/core/exec/placement.rs`; `footprint`, `footprint.rs`). Anything else, an ET_EXEC or an image linked high, and an image with no Image header, is refused rather than moved (`route` and `footprint_rule`, `image_rules.rs`).
- After `ExitBootServices` it cleans its own allocations and the tree to the point of coherency by address (`clean_handed_over`, `nonos-bootloader/src/arch/aarch64/exit/clean.rs`), then masks interrupts, turns the data cache off, cleans every cache level by set and way with a barrier after each level, turns the MMU and instruction cache off and branches with `x0` the tree and `x1` the handoff (`nonos-bootloader/src/arch/aarch64/asm/`).
- Its entropy needs the UEFI RNG protocol or RNDR, or it refuses the boot (`strong_enough`, `nonos-bootloader/src/entropy/strong_rule.rs`).

The kernel takes the handoff in `x1` beside the tree in `x0` (`src/arch/aarch64/boot/loader/`). The handoff is the same `BootHandoffV1` as x86_64's; it is copied while the MMU is off and installed through the same checks, and a non-zero `x1` that is not a whole, current handoff refuses the boot. With a handoff, free RAM comes from the loader's EFI map: Conventional memory only, as on x86_64, and of that only write-back memory not set aside for a specific purpose. The tree calls RAM available that the handoff, the map and the runtime services still hold. Each free run is then held inside the tree's RAM, which is what the direct map covers, and below where the direct map stops (`ram`, `src/arch/aarch64/boot/memory/ram.rs`; `within`, `clip.rs`). The kernel's TPM driver reads its register window from the tree's `tcg,tpm-tis-mmio` node (`src/arch/fdt/find/tpm/`), and the loader uses only the firmware's TCG2 protocol on Arm.

Under QEMU with AAVMF (`-M virt,gic-version=3,acpi=off -cpu max`), with throwaway keys, the serial log reads:

```
  [CRYPTO] Ed25519 verify ....................... [PASS]
  [CRYPTO] ML-DSA-65 verify .................... [PASS]
[NONOS] dtb at 0x0000000047EF6000
[BOOT] NONOS loader handoff taken: kernel verified=1 secure boot=0
[MEM] phys init OK
[NONOS] Core ready
[BOOT-ATTEST] bootloader not checked: no boot-root record or trailer
[ERROR] bootloader not admitted by the kernel's check; stopping
```

That boot had no boot-root record, which only a policy key signs, so it stops at the same check.

## What is missing

- A sealed image and a runner for it. The loader builds for aarch64 (above), but the Nix image build makes the x86_64 kernel and loader only (`tools/nix/image.nix`), the profiles carry no architecture (`tools/nix/config.nix`), and `tools/nonos_qemu` starts only x86_64 machines. So `make dev-image` has no aarch64 profile, and nothing signs a boot-root record for an aarch64 loader.
- Booting from ACPI alone. Servers built to SBSA publish ACPI and no device tree, and the kernel reads its platform only from a tree.
- Kernel stack guard pages. `arm_stack_guards` exists only on x86_64 (`src/kernel_core/init/entry/init_vm_and_protection.rs`).
- The capsules that link the std platform layer in the desktop (`DESKTOP_STD_TOOL_ARTIFACTS`, `mk/20-build.mk`). They build for `aarch64-nonos-user`: the layer reaches the kernel through one file per architecture's registers (`toolchain/nonos-std/sys/pal/nonos/trap.rs`). The arm desktop does not carry them yet, and none has run on aarch64.
- Checks for every profile. The flake's `profileChecks` type-check each profile against `x86_64-nonos.json` only (`tools/nix/checks.nix`).

Since this page was first written, the kernel gained:
- a device tree under QEMU, by booting the arm64 Image;
- the SMMUv3 (`src/arch/aarch64/iommu/smmu`);
- PAN on every CPU that has it (`src/arch/aarch64/security/pan.rs`);
- TLS through `tpidr_el0`, kept per thread (`src/arch/aarch64/context/tls.rs`);
- a GIC driver that is not limited to v3.

These are written and tested on the host. Which of them a boot has exercised is the boot gate's record, not this page's.

## See also

- [Architectures](README.md)
- [x86_64](x86_64.md)
- [riscv64](riscv64.md)
- [Boot handoff](../kernel/boot-handoff.md)
- [IOMMU](../kernel/iommu.md)
- [Make targets](../build/make-targets.md)
