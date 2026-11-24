# Current state, measured

This is what `src/` holds at `551bbd34`, according to the code: for each top-level module and each subtree over about 1,000 lines, what it does, its size, the privileged operation it actually needs, and who calls it. [method.md](method.md) gives the scripts. The outcome classes (K, R, U, D, X) are argued in [04-classification.md](04-classification.md); they appear here only so the table can be read once.

## Totals

| Measure | Value |
|---|---:|
| `*.rs` and `*.S` files under `src/` | 7,676 (7,648 `.rs`, 28 `.S`) |
| Physical lines | 396,916 |
| Code lines (blank and comment lines removed) | 209,644 |
| Physical lines in files not gated to another architecture, that is, the x86_64 release build (all but class X; this includes the 382 lines of three files no `mod.rs` declares) | 349,816 |
| Code lines compiled for x86_64 | 186,747 |
| Physical lines no live path reaches, or reaches with no effect, by grep (class D, destinations "dead", "inert" and "uncompiled") | 64,964 |
| System calls published in `abi/syscalls.toml` and routed | 196 |

About one line in six of `src/` is dead or inert. A further 47,100 lines are not compiled for x86_64, the only release target (`src/lib.rs:32-36`): `arch/aarch64`, `arch/riscv64` and `arch/fdt` (44,198), the aarch64 and riscv64 interrupt binding under `hardware/broker/irq` (1,624), the non-x86 handoff readers and stubs such as `src/process/foreign_absent*.rs`.

## Per module

phys and code are line counts; K, R, U, D and X split the phys lines by outcome class.

| module | phys | code | files | K | R | U | D | X |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| arch | 112764 | 58738 | 2228 | 33923 | 18536 | 3752 | 11755 | 44798 |
| hardware | 55041 | 27177 | 1126 | 16189 | 11018 | 22205 | 3936 | 1693 |
| memory | 30306 | 15849 | 664 | 20285 | 2193 | 0 | 7828 | 0 |
| security | 24340 | 12535 | 453 | 3055 | 1166 | 14141 | 5978 | 0 |
| crypto | 23749 | 14985 | 336 | 3797 | 0 | 11547 | 8405 | 0 |
| userspace | 22452 | 11526 | 481 | 0 | 0 | 22091 | 361 | 0 |
| syscall | 22331 | 11408 | 448 | 12995 | 2613 | 5593 | 1094 | 36 |
| fs | 21927 | 13270 | 336 | 0 | 0 | 9147 | 12709 | 71 |
| process | 19572 | 10722 | 315 | 10618 | 6899 | 0 | 1819 | 236 |
| elf | 11459 | 6199 | 290 | 0 | 0 | 4155 | 7304 | 0 |
| kernel_core | 10127 | 4835 | 198 | 2509 | 7402 | 0 | 216 | 0 |
| drivers | 8075 | 4926 | 121 | 7196 | 0 | 0 | 879 | 0 |
| sys | 7587 | 3432 | 173 | 5203 | 941 | 618 | 825 | 0 |
| smp | 5274 | 2489 | 90 | 5216 | 0 | 0 | 0 | 58 |
| interrupts | 5236 | 2627 | 112 | 5236 | 0 | 0 | 0 | 0 |
| capabilities | 3733 | 2209 | 72 | 1327 | 0 | 0 | 2406 | 0 |
| boot | 3720 | 1994 | 70 | 3044 | 0 | 0 | 468 | 208 |
| services | 2770 | 1237 | 53 | 0 | 0 | 2770 | 0 | 0 |
| ipc | 2291 | 1340 | 30 | 1511 | 0 | 0 | 780 | 0 |
| bus | 1181 | 634 | 22 | 0 | 1181 | 0 | 0 | 0 |
| log | 1091 | 598 | 21 | 1091 | 0 | 0 | 0 | 0 |
| usercopy | 896 | 428 | 19 | 896 | 0 | 0 | 0 | 0 |
| context | 304 | 156 | 5 | 0 | 0 | 0 | 304 | 0 |
| (crate root) | 216 | 147 | 2 | 216 | 0 | 0 | 0 | 0 |
| time | 215 | 59 | 5 | 215 | 0 | 0 | 0 | 0 |
| entry | 167 | 77 | 4 | 167 | 0 | 0 | 0 | 0 |
| sched | 92 | 47 | 2 | 92 | 0 | 0 | 0 | 0 |

## How the kernel is entered

- **Boot.** The live sequence is `src/boot/main/core_init/`: CPU tables, logging and the IDT (`src/boot/main/core_init/cpu_tables.rs:47-48`), the SYSCALL MSRs (`src/boot/main/core_init/cpu_tables.rs:31`), ACPI (`src/boot/main/core_init/acpi_tables.rs:25`) and PCI (`src/boot/main/core_init/init_core_systems.rs:52`). Then `kernel_core::init` brings up memory (`src/kernel_core/init/entry/init_vm_and_protection.rs:28`), SMP (`src/kernel_core/init/entry/init_core_services.rs:48`), DMA protection (`src/kernel_core/init/entry/init_dma_protection.rs:51`), the device broker (`src/kernel_core/init/platform/hardware_broker.rs:20-27`) and the bootloader verdict (`src/kernel_core/init/entry/microkernel_main.rs:26`).
- **Init.** Init is kernel code. `microkernel_main` creates a process named `init` (`src/kernel_core/init/entry/microkernel_main.rs:42`), gives it an address space and a kernel stack, and calls `crate::userspace::run_init()` as an ordinary function (`src/kernel_core/init/entry/microkernel_main.rs:61`). There is no transition to ring 3: the whole spawn plan and supervisor run at CPL 0 under the init pid.
- **System calls.** `SYSCALL` lands in `src/arch/x86_64/asm/syscall.S`, then `src/arch/x86_64/syscall/manager/entry.rs`. A number the table does not know goes to the foreign-process redirect (`src/arch/x86_64/syscall/manager/entry.rs:49`); a known one goes through the capability contract (`src/arch/x86_64/syscall/manager/entry.rs:63`, `src/syscall/contract/dispatch.rs:32`). [02-syscall-surface-today.md](02-syscall-surface-today.md) has the detail.
- **Interrupts.** The live IDT is `src/interrupts/idt` (`src/boot/main/core_init/cpu_tables.rs:48`); device interrupts brokered to userland arrive at `src/arch/x86_64/interrupt/broker/isr.rs:43`.

## arch (112,764 phys)

`src/arch/mod.rs` gates the trees: `aarch64` (29,301) on `target_arch = "aarch64"` (`src/arch/mod.rs:17-18`), `fdt` (6,352) on aarch64 or riscv64 (`src/arch/mod.rs:32-33`), `riscv64` (8,545) on riscv64 (`src/arch/mod.rs:58-59`). The x86_64 tree is glob re-exported (`src/arch/mod.rs:116`), so `crate::arch::foo` can name `x86_64::foo`.

| Subtree | phys | What it does | Privileged operation | Callers / status |
|---|---:|---|---|---|
| x86_64/acpi | 14,203 | Parses RSDP, XSDT, FADT, MADT, MCFG, HPET, SRAT, SLIT, DMAR and other tables from firmware memory; `aml/` (2,071) is a bounded resource extractor that scans DSDT bytes for `_HID`/`_CRS` without executing AML; `hw/` (1,336) does PM1 sleep, reset and power-button port I/O | Port I/O for PM1 and reset; reads of physical firmware memory | `src/boot/main/core_init/acpi_tables.rs:25`; parsing untrusted firmware data is most of the tree |
| x86_64/iommu | 10,937 | VT-d and AMD-Vi DMA remapping and interrupt remapping, translation tables, ACS | IOMMU MMIO registers, IOMMU page tables | `src/kernel_core/init/entry/init_dma_protection.rs:51`; fault polling from the timer, `src/interrupts/timer/clock.rs:54` |
| x86_64/time | 5,272 | TSC calibration, PIT, CMOS RTC, HPET | `rdtsc`, PIT and RTC port I/O, HPET MMIO | `src/boot/main/core_init/init_core_systems.rs:26`; `x86_64::time::init` is reached only from dead `x86_64/api.rs` |
| x86_64/interrupt | 4,723 | LAPIC and x2APIC, IOAPIC routing including remapped routes, legacy PIC masking, the brokered-IRQ ISR stubs | APIC MSRs and MMIO, PIC port I/O | `src/arch/init_broker_irq_routing.rs`, boot sequence |
| x86_64/uefi | 3,392 | UEFI runtime services, variables, secure boot | none exercised | Dead: nothing outside the directory names `uefi::` |
| x86_64/cpu | 2,936 | CPUID features, hybrid core kind, frequency calibration | `cpuid`, MSRs | Scheduler and logging |
| x86_64/multiboot | 2,884 | Multiboot2 information parser | none | Dead: only caller is `src/arch/nonos_boot.rs:30`, inside `init_early`, which nothing calls |
| x86_64/hypervisor | 2,184 | CPUID hypervisor classification, the VMware backdoor (port 0x5658), Hyper-V SynIC and hypercalls for VMBus | `in eax,dx` on the backdoor port, SynIC MSRs, `vmcall` | `src/syscall/microkernel/hv_backdoor.rs:26` passes userland requests through |
| x86_64/boot | 2,064 | MSR and CPUID wrappers, SSE and AVX enable | MSRs, control registers | AP bring-up, extended state init |
| x86_64/smm | 1,630 | SMM detection and management | MSRs | Dead: nothing outside the directory names `smm::` |
| x86_64/gdt | 1,522 | GDT, TSS, IST stacks | `lgdt`, `ltr`, MSRs | Boot and AP bring-up; the TSS has I/O-bitmap support (`src/arch/x86_64/gdt/tss_struct.rs:83`) that nothing uses |
| x86_64/pci | 1,518 | A third PCI enumerator | config space | Dead: only caller is `src/arch/nonos_boot.rs` |
| x86_64/idt | 1,437 | A second IDT implementation | `lidt` | Its `init` is reached only from dead code; the live IDT is `src/interrupts/idt` |
| x86_64/port | 1,445 | `in`/`out` wrappers | port I/O | ACPI hardware code |
| x86_64/serial | 1,145 | 16550 driver | port I/O | Dead: the live console is `sys::serial` over `arch::console` |
| x86_64/vga, watchdog, smaller | 1,094 | Text-mode VGA, TCO watchdog | port I/O | Dead apart from the `Color` type |
| x86_64 asm, syscall, context, paging, pat, diag | 2,598 | SYSCALL entry and exit, context switch, CR3, PAT | the instructions themselves | Every path |
| paging (arch-neutral) | 1,132 | CR3 read and write, `invlpg`, PCID | CR3, TLB | Address-space switch |
| trap, interrupt_controller | 1,539 | Arch-neutral facades over the IDT, GIC and PLIC | none of their own | Exception and IRQ paths |

## hardware (55,041 phys)

| Subtree | phys | What it does | Privileged operation | Callers / status |
|---|---:|---|---|---|
| broker | 29,348 | The device broker: device table, claims, IOMMU confinement, DMA, MMIO, IRQ, port I/O and PCI-config grants to userland drivers, plus the ACPI capsule's grants | See below | 27 device syscalls and about 30 ACPI syscalls |
| broker/acpi | 6,172 | Grant and policy layer for the userland ACPI capsule: which tables it may read, which OperationRegion windows its interpreter may reach, sleep and S5, power button ownership, NUMA, thermal and PCI hot-unplug stores. About 400 lines touch registers; the rest are rules and stores | Port I/O and config space, through about 400 lines | `src/syscall/microkernel/acpi/` |
| broker/irq | 4,513 | Vector slots, INTx, MSI and MSI-X binding, interrupt remapping, the ISR dispatch (1,624 of it is aarch64 and riscv64) | IOAPIC, MSI, MSI-X and IR programming, EOI | `src/arch/x86_64/interrupt/broker/isr.rs:43` |
| broker/claim | 3,592 | The claim state machine: firmware check, hold rules, scope check against the services the holder serves, bus master off, D0, reset, BAR restore, takeover, admit; release and exit; wipe between holders (`src/hardware/broker/claim/claim.rs:22`) | PCI config writes (Command, PM, FLR) | `MkDeviceClaim`, process exit |
| broker/confine | 3,237 | One IOMMU domain per holder (`src/hardware/broker/confine/capsule.rs:46`), ACS and isolation-group checks, IOVA allocation, RMRR | IOMMU domain programming | Claim and DMA paths |
| broker/dma | 3,235 | Kernel-allocated zeroed buffers mapped into both the caller's address space and the holder's IOMMU domain; surface import for GPUs | page tables, IOMMU tables | `MkDmaMap`, surface pinning |
| broker/mmio, pci, pio | 3,328 | BAR mapping into the caller (`src/hardware/broker/mmio/map_install.rs:39`); allow-listed config reads and writes; port I/O, where every single `in` or `out` is a system call run in ring 0 (`src/hardware/broker/pio/access/raw/in16.rs:22`) | page tables, config space, port I/O | Device syscalls |
| block_device | 4,673 | The kernel block layer: boot and store disk choice, GPT and MBR, store copy and mirror, volume-scope rules, the SCSI wire protocol for eight driver capsules. No register access; every I/O is IPC to a userland driver | none | Store syscalls, the data volume (`src/syscall/microkernel/data/read.rs:58`), zerostate |
| inventory | 1,453 | Classifies broker records into hardware families by ID tables | none | Init's spawn plan |
| boot_census | 589 | Builds the `[CENSUS]` lines | none | Init, `MkBootCensus` |
| 64 `*_capsule` dirs | 18,907 | Per driver capsule: `embed.rs` (3,547 lines) bakes the driver ELF, certificate, manifest and trailer into the kernel; `spawn.rs` (4,015) builds the spawn spec; 11 dirs also hold a kernel-side IPC client in `client/` (5,175 lines, with `protocol/` 1,264), and usb_msc_capsule holds one in loose files. No ring-0 driver code: none contains a register access | none | Spawned from init's spawn plan. Only four clients have a kernel caller, all in `block_device`: nvme, ahci, virtio_blk and usb_msc. The other eight clients (e1000, xhci, hda, ps2_kbd, rtl8139, rtl8169, virtio_net, virtio_rng) are dead |

## memory (30,306 phys)

The live initialisation is `unified::init_unified_vm` (`src/kernel_core/init/entry/init_vm_and_protection.rs:28`). A second entry, `unified::init_all_memory_subsystems`, has no caller.

| Subtree | phys | What it does | Privileged operation | Status |
|---|---:|---|---|---|
| paging | 6,766 | The page-table manager: address spaces, map and unmap in an ASID, `translate_in_asid` (`src/memory/paging/manager/api/mapping_in_asid.rs:71`), TLB shootdown (1,062 lines), faults | page tables, CR3, `invlpg`, IPIs | Live, authoritative |
| phys | 1,676 | The real physical frame allocator: bitmap, quarantine, KASLR-seeded start | none itself | Live |
| frame_alloc | 595 | Facade over `phys` | none | Live |
| buddy_alloc | 1,230 | Kernel virtual ranges and guarded kernel stacks, backed by `frame_alloc` | page tables | Live |
| mmu | 1,602 | CR0.WP, CR4 SMEP, SMAP and UMIP, EFER.NX; a `mapping/` subtree that duplicates `paging` | control registers | Protection bits live; mapper dead |
| mmio, heap, layout, iommu front end, kaslr, encryption, proof, unified, addr | 8,993 | Kernel MMIO mappings, the global allocator, address-space layout constants, IOMMU posture, KASLR, SME/TME, MMIO-map audit records | page tables, MSRs | Live |
| boot_memory, region, safety, secure_memory, page_allocator, page_info, dma | 8,047 | A second boot allocator, a region manager, integrity checks, a secure allocator, a page allocator API, a page tracker and a DMA allocator | none exercised | Dead apart from `page_allocator::init`, the `PageFlags` type and the `dma::coherency` trait |

## process (19,572 phys)

| Subtree | phys | What it does | Privileged operation | Status |
|---|---:|---|---|---|
| foreign | 4,842 | The Linux-guest mechanism: an empty, capability-free guest process supervised by its creator; unknown syscall numbers parked and handed to the supervisor (`src/process/foreign/trap.rs:28`); peer map, copy, protect, pin and TLS on the guest's address space; fork, exec and threads for guests; register-context read and write for signal delivery (`src/process/foreign/signal_call.rs`); the seal that records what a guest ran | page tables, user frames through the direct map | Live, x86_64 only; the personality in userland drives it |
| scheduler | 4,320 | Five fixed priority bands with round robin inside a band (`src/process/scheduler/selection/band_scan.rs:52`), directed yield, tick preemption, kernel realtime and deadline task queues. `policy.rs` and `policy_types.rs` (407) implement a Linux-style SCHED_FIFO, RR and DEADLINE registry that selection never reads | context switch, CR3, timer | Live apart from the policy registry |
| core | 2,300 | PCB, process table, TIDs, thread groups, argv, VMAs, FPU state | none itself | Live |
| signal | 2,057 | Signal state, sigaction storage, an in-kernel handler-frame builder and sigreturn | user stack writes | The handler path is unreachable: nothing calls `SignalState::set_action` and no syscall installs a handler ([migration/signal-delivery.md](migration/signal-delivery.md)) |
| address_space | 1,225 | `lifecycle/` (242) creates and releases address spaces through `memory::paging` (`src/process/core/table/create.rs:63`); `fork.rs`, `ops.rs`, `pcid.rs`, `tlb.rs`, `protection.rs` are a third, unused page-table walker | page tables | lifecycle live; the rest dead |
| exit, userspace, context, caps, accounting, small files | 4,592 | Teardown and reclaim, ring-3 entry and return, CPU context, the per-process capability token, accounting; `operations_exec.rs` holds `enumerate_all_processes`, which the shutdown wipe uses (`src/security/hardening/memory_sanitization/kernel_stacks.rs:37`); `fd_table.rs`, `process_fd_table.rs` and `fd_types.rs` define a descriptor table every PCB carries (`src/process/core/pcb.rs:107`) and only the dead `src/fs/api.rs` reads | `sysret`, `iretq` | Live, apart from the descriptor table, which is inert |

## security (24,340 phys)

| Subtree | phys | What it does | Privileged operation | Status / callers |
|---|---:|---|---|---|
| tpm | 5,674 | A TPM 2.0 driver (FIFO/TIS and CRB) and fixed command builders: PCR read and extend, NV read of the rollback floor (`src/security/tpm/boot_reads/floor.rs:60-66`), quote, an AK, a PCR-policy-bound HMAC machine key, a release-policy-approved device secret, EK and AK enrolment | MMIO of the TPM window | Boot verdict, PCR 14 extend on every capsule admission, the data volume key, and five system calls (`CryptoMachineKey`, `MkDeviceSecret`, `MkEnroll`, `MkAttestDoc`, `MkBootSlots`) |
| hardening | 2,697 | Spectre mitigations (IBPB, IBRS, STIBP, SSBD, RSB fill, L1D flush, MDS clear), speculation barriers on entry and exit, secure zeroing; `memory_encryption/` (338) is dead | MSRs, `verw`, cache flush | Context switch, syscall entry and exit |
| crypto | 1,723 | Fallback random source; constant-time helpers, key management, trusted keys and hashes with no live caller | none | Mostly dead |
| boot | 1,377 | `loader_check` (379), the kernel's own verdict on the bootloader, run once (`src/security/boot/loader_check/run.rs:28`): replay of the TCG log against PCR 4 and the rollback floor through `nonos_boot_measure`; `slots`, the boot-slot record; `secure_boot` (612) is dead | TPM reads | `src/kernel_core/init/entry/microkernel_main.rs:26`; a refusal halts the boot (`src/kernel_core/init/entry/loader_refusal.rs:27`) |
| capsule_manifest, nonos_id_cert, nonos_trust_anchor, capsule_attest, admission_cache, dev_roots, local_build, attest_policy | 4,029 | The spawn gate's verifiers: identity certificate, publisher-signed manifest, the v4 attestation trailer (Merkle path plus STARK through `nox_verify`), the vendor root and enrolled roots, the keyed-tag trailers of locally built capsules, and an acceptance cache | none | Spawn path, `MkCapsuleVerify`, `MkLocalVerify`, `MkLocalSign` |
| crypto_capsule, entropy_capsule, keyring_capsule, market_capsule | 3,659 | Embed and spawn of four userland capsules, plus kernel-side IPC clients: the crypto client serves the crypto system calls by forwarding them, the keyring client serves the data volume, the market client serves init's Linux jobs | none | See [11-target-services-out-of-kernel.md](11-target-services-out-of-kernel.md) |
| attest_registry, attest_doc, capsule_measure | 1,693 | The table of running capsules with what was proved about each; the TPM-quoted "what is running" document; the PCR 14 capsule event log | TPM | Spawn path, `MkAttestDoc` |
| policy, quantum, monitoring, loose dead files | 2,378 | Canary and CFI configuration, user sessions, a post-quantum "engine", a rootkit scanner, kernel self-attestation | none | Dead |
| zerostate, observability, boot_session, image_ceiling | 970 | Shutdown and panic wipe and the only route to power-off; address redaction in fault output; the boot nonce in capability tokens; the image capability ceiling | power control | Live |

## crypto (23,749 phys)

The kernel's own verification needs Ed25519 and ML-DSA-65 (`src/crypto/asymmetric/alg_id/verify.rs:33-40`), P-256 for the signed boot-root record, SHA-2 for TPM banks and BLAKE3 (the external `blake3` crate for measurement, the in-tree one for capability-token material). The data volume needs ChaCha20-Poly1305 and Argon2. Hashing, AEAD, HMAC, HKDF and X25519 requested by capsules are forwarded to the userland crypto capsule; only Keccak-256, the random source and the machine key are computed in ring 0 for a caller. P-384, RSA, Curve25519, Kyber, NTRU, Classic McEliece, SPHINCS+, AES and AES-GCM have no live caller; with the unused parts of `application` they are 8,405 phys lines.

| Subtree | phys | Status |
|---|---:|---|
| asymmetric | 6,798 | Ed25519 and P-256 live; P-384, RSA, Curve25519 dead |
| util | 5,996 | RNG, entropy and constant-time live in ring 0; bigint and HMAC live for verification; Argon2 for the volume |
| pqc | 3,988 | ML-DSA-65 verify live; the rest dead |
| hash | 2,313 | Live |
| symmetric | 1,830 | ChaCha20-Poly1305 live for the volume; AES dead |
| application | 1,091 | A self-test at init, a vault and a signing helper; dead apart from the self-test |

## userspace (22,452 phys)

| Subtree | phys | What it does | Callers |
|---|---:|---|---|
| init | 7,459 | Init, in ring 0: `spawn_plan` (4,125) orders 17 driver stages with a compile-time topological sort and spawns capsules from 131 call sites; `supervisor` (436) restarts 32 watched services (`src/userspace/init/supervisor/watch_rule.rs:28`); `linux_jobs` (1,054) runs install, uninstall and run jobs for the Linux personality and queries the market capsule as a kernel IPC client; `instance_spawn` (703) opens extra windows | `src/kernel_core/init/entry/microkernel_main.rs:61`, and syscalls that enqueue work for it |
| 69 `capsule_*` dirs | 13,872 | Per capsule: embedded ELF, certificate, manifest and trailer (for example `src/userspace/capsule_linux/embed.rs:22`), a spawn spec, a liveness cell | Init's spawn plan |
| capsule_linux | 1,438 | The personality's roles (`src/userspace/capsule_linux/roles.rs`), its network roles (`src/userspace/capsule_linux/run_net.rs:29-36`), terminal tickets, and direct reaches into the process table | Syscalls, process exit, init |
| tool_capsules | 679 | A registry of tool capsules run by `MkToolRun` | `src/syscall/microkernel/tool_run.rs` |

## syscall (22,331 phys)

| Subtree | phys | What it does |
|---|---:|---|
| microkernel | 15,317 | The handlers of 169 `Mk*` system calls: IPC (3,170), ACPI (1,748), the data volume (988), memory, futex, device broker, foreign process, attestation and TPM, install and launch, process statistics, terminals |
| dispatch | 3,749 | The router (`src/syscall/dispatch/router/dispatch_fn.rs:22`), the crypto proxy (1,094), surface and input handlers, the in-ring-0 framebuffer blit for `MkSurfacePresent` |
| contract | 1,474 | The single capability gate and the per-token syscall allowlist |
| abi, caps, numbers, types, validation | 1,723 | The FourCC number registry, capability checks, argument types |

## fs (21,927 phys)

The in-kernel filesystem is never mounted. `fs::init` (`src/fs/ops.rs:25`) initialises the VFS, cryptofs, ramfs and procfs, and `FileSystemManager::init` (`src/fs/manager.rs:48-60`) also initialises the VFS; neither `fs::init` nor `init_filesystem_manager` has a caller in `src/`, so `get_vfs()` always returns `None` (`src/fs/vfs/vfs_global.rs:33-35`). Files are served by the userland `capsule_vfs`.

| Subtree | phys | Status |
|---|---:|---|
| blockfs_volume | 4,112 | Live: the machine's encrypted data volume behind the `MkData*` and `MkUserObj*` system calls, keyed from the TPM machine key or an Argon2 passphrase key |
| blockfs | 3,391 | Live: the on-disk format under the volume |
| cryptoblock | 1,131 | Live: AEAD per sector, I/O to the block-driver capsule over IPC |
| procfs | 1,949 | Only the load-average computation is live (`src/interrupts/timer/clock.rs:50`) |
| vfs, ramfs, cryptofs, fd, utils, cache, path, pipe, storage, loose files | 9,862 | Dead |
| vfs_capsule, ramfs_capsule | 1,481 | Embed and spawn of the two userland capsules; their kernel-side clients and protocol code (968) have no caller |

## elf (11,459 phys)

One function loads every capsule: `load_elf_entry_into`, called once, from `src/kernel_core/process_spawn/capsule_spawn/runner/install/load_elf_into_pid.rs:34`. It parses the header, maps each `PT_LOAD`, applies `R_X86_64_RELATIVE` relocations through the direct map (`src/elf/loader/core/relocate.rs:49-110`), drops write on `PT_GNU_RELRO` (`src/elf/loader/core/relro.rs:31-57`) and computes the entry (`src/elf/loader/core/loader/base_addr.rs:31-37`). That path, with the types, errors and ASLR it uses, is about 3,160 phys lines; `src/elf/loader` also holds about 990 lines reached only from `load_elf_executable` and `load_elf_executable_into` (`src/elf/loader/global.rs:44-60`), which nothing outside `src/elf` calls, and the classification counts those 4,155 lines together as the loader that moves. The other 7,304 (`libmgr`, `got`, `symbol`, `hash`, `dynlink`, `reloc`, `embedded`, `cache`, `init`, `fini`, `interpreter`, `tls`, `auxv`, `stack`, `process`, `minimal`) have no caller outside `src/elf`. [migration/elf-loading.md](migration/elf-loading.md) covers both.

## kernel_core (10,127 phys)

| Subtree | phys | What it does | Status |
|---|---:|---|---|
| surface_registry | 4,637 | A kernel table of 256 shared framebuffer surfaces: owner frames pinned and mapped into receivers' address spaces, share rights, fences, vsync, a 1,024-entry input event ring, Linux-guest window pins | Live |
| process_spawn | 3,152 | The capsule spawn gate (`capsule_spawn`, 2,765) and kernel and user stack allocation | Live |
| init | 2,095 | Boot-time bring-up (entry, framebuffer, memory, platform) | Live |
| boundary.rs, service.rs, spawn.rs | 216 | An older component registry | Dead |

## Smaller modules

| Module | phys | What it does | Privileged operation | Callers |
|---|---:|---|---|---|
| drivers | 8,075 | `drivers/pci` (7,155) is the live PCI core: ECAM and legacy config access, enumeration (`src/kernel_core/init/platform/hardware_broker.rs:20`), Intel VMD bring-up, MSI and MSI-X programming. `drivers/security` (879) is dead | config space, MMIO | Broker |
| sys | 7,587 | Serial console, boot log, APIC init (a second APIC layer beside `arch/x86_64/interrupt`), clock page and civil time, uptime, hostname and timezone policy, benchmarks, an `IrqMutex`; `SeqLock`, `Semaphore`, `IrqRwLock` and `sys/gdt` are dead | APIC, port I/O | Everywhere |
| smp | 5,274 | AP bring-up through a trampoline, per-CPU data, IPIs and TLB shootdown, spin watchdogs | CR3, MSRs, IPIs | `src/kernel_core/init/entry/init_core_services.rs:48` |
| interrupts | 5,236 | The live IDT, exception and IRQ handlers, the timer tick | `lidt`, EOI | `src/boot/main/core_init/cpu_tables.rs:48` |
| capabilities | 3,733 | The 44-bit capability enum and the per-process token with its keyed-BLAKE3 MAC (`src/capabilities/token/material.rs:41`). `audit`, `chain`, `delegation`, `multisig`, `resource` and `roles` (2,406) are dead | none | Token mint `src/process/caps/mint.rs:34` |
| boot | 3,720 | Validation and queries of the bootloader handoff (2,303), the boot sequence, panic | none | Boot |
| services | 2,770 | An in-kernel name service: up to 256 named endpoints with a port, an owner pid and the capability bits a sender needs (`src/services/registry.rs:58`, `src/services/registry.rs:76`); kernel-side capsule supervision and a kernel-as-client RPC transport (`lifecycle`, 1,289) | none | Spawn, IPC, 38 lookup sites |
| ipc | 2,291 | Per-process named inboxes of byte messages (`src/ipc/nonos_channel/message.rs:25`); `pipe/` (398) duplicates `fs/pipe`; three files in `nonos_channel` are not declared in its `mod.rs` and are never compiled | none | IPC system calls (`src/ipc/kernel_ipc.rs:39`) |
| bus | 1,181 | A second PCI enumerator with BAR and bridge-window assignment and AER reporting | config space through `drivers/pci` | `src/kernel_core/init/platform/baseline.rs:46`, `src/kernel_core/init/platform/baseline.rs:55` |
| log, usercopy, context, time, entry, sched | 2,765 | Logging; `copy_from_user`/`copy_to_user` by a software page walk and the direct map (`src/usercopy/direct.rs:37`); a per-CPU context tag whose setters nothing calls, so `src/usercopy/walk/root.rs:28` always falls back to the active CR3; `now_ns`; OOM handling (`src/lib.rs:80`); re-exports of `process::scheduler` | none beyond the direct map | Everywhere |

## The trust chain into the kernel

Only the parts that define trust are covered here.

1. **The bootloader verifies the kernel.** `nonos-bootloader` (39,437 lines) checks the kernel image's signature over its hash and a rollback index (`nonos-bootloader/src/kernel_verify/signature_message.rs:17`, `nonos-bootloader/src/kernel_verify/verify.rs:29`), checks the kernel's own attestation trailer (`nonos-bootloader/src/kernel_verify/self_attest.rs:31`), reads and commits the TPM rollback floor (`nonos-bootloader/src/security/tpm_nv/floor.rs:28`, `nonos-bootloader/src/security/tpm_nv/floor.rs:42`) and extends the measurements into the TPM.
2. **The build bakes the trust set into the kernel.** `build_baked_trust.rs:30-51` chooses `nonos-data/trust` (or a test set named by `NONOS_BAKED_TRUST_DIR`) and exports it as `NONOS_BAKED_TRUST`. `build_trust_stage.rs:29-36` stages the image capability ceiling, writing eight zero bytes when the file is absent, and `build_trust_stage.rs:43-57` stages the P-256 device-policy key. The kernel reads every capsule's certificate, manifest and trailer through `baked_trust!` (`src/security/nonos_trust_anchor/baked_trust.rs:23`): 422 sites in `src/`. Each capsule's ELF is an `include_bytes!` of the userland build output (147 sites).
3. **The kernel re-checks the boot.** `check_bootloader` replays the TCG log to PCR 4, reads the rollback floor from TPM NV and decides a verdict (`src/security/boot/loader_check/run.rs:34-55`); `Refused` and `NoEvidence` halt.
4. **The kernel gates every capsule.** Certificate, manifest, ceiling and attestation, in ring 0, then the kernel's ELF loader maps the bytes it measured. [09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md) walks the path with citations.

The trust set itself is 168 capsules with three files each in `nonos-data/trust/capsules/`; the attestation trailers are 19,254,712 bytes together, about 114 KB each, and are compiled into the kernel image.

## What surprised

1. **Init runs in ring 0.** The spawn plan, the supervisor's restart policy, Linux job handling and its market lookups are kernel code called as a function (`src/kernel_core/init/entry/microkernel_main.rs:61`).
2. **About 65,000 lines are dead or inert**, and some are trusted by proofs. The project's refinement table binds the `Bounds` model to `src/elf/reloc` and the `Seqlock` model to `src/sys/sync/seqlock` (`verification/lean/REFINEMENT.md:46`, `verification/lean/REFINEMENT.md:36`); neither has a live caller, and the live relocator, `src/elf/loader/core/relocate.rs`, is bound to no model.
3. **Three of everything.** Three IDTs, two APIC layers, three PCI enumerators (one dead), four frame or page allocator layers plus two dead ones, three page-table walkers (two dead), two pipe implementations.
4. **The capability model is ambient.** A capability is a bit in a 44-bit per-process word, checked by the syscall gate and by services asking the kernel about a pid (`mk_cap_check`). Nothing names an object, nothing is transferable over IPC, and the Network bit grants no system call: its one effect on the system-call gate is to disqualify its holder from `MkAttestDoc` (`src/syscall/caps/checks/system.rs:63`), and otherwise it is read by the service registry when a sender addresses a network service (`src/services/registry/policy.rs:26-38`, `src/services/registry/policy.rs:49`).
5. **The attestation proves the required capabilities only.** The spawn gate passes `required_caps` to the attestation check (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:81`). Optional bits, such as the Network a Linux network role adds (`src/userspace/capsule_linux/run_net.rs:35`), are bounded by the signed manifest and the certificate ceiling and are not part of what was proved.
6. **A filesystem service is live in ring 0.** The encrypted data volume, its on-disk format and its sector crypto (8,634 phys lines) run in the kernel behind the `MkData*` and `MkUserObj*` system calls, with the disk I/O done over IPC to a userland driver.
7. **The kernel is a client of its own services.** Crypto system calls block the caller while the kernel round-trips to the crypto capsule; the block layer is a client of four driver capsules; init is a client of the market capsule.
8. **Port I/O is a system call per access.** The TSS I/O bitmap exists and is unused.
