# Migration plan

The order across all subsystems, what each phase ships, the measurable target for ring 0, and what a smaller kernel lets the project prove. The per-subsystem steps are in [migration/](migration/); this page sequences them.

## Principles of the ordering

1. **Delete before moving.** About 65,000 phys lines are dead or inert. Deleting them first shrinks every later review and makes every "who calls this" question answerable by the compiler.
2. **Land the audit's in-place fixes first.** They protect the current system while the structure changes, and several (the page-safe store primitive, the fail-closed ceiling, the vfs filters) stay in the target.
3. **Move what needs no new kernel interface before building the object model.** Userland-only changes (vfs, signals, the crypto proxy, self-relocation) ship on today's kernel.
4. **Build the object model next to today's interface, then move services onto it, then remove the old interface.** Old and new system calls coexist through phases 3 and 4.
5. **Every step ships alone and keeps an invariant** stated in its migration spec, with a test that checks it.

## Phases

| Phase | Ships | Depends on | Specs |
|---|---|---|---|
| 0. Cleanup | Deletion of the 64,964 dead, inert and never-compiled phys lines ([04-classification.md](04-classification.md), "Deleted"; the dead ELF trees first, with their proof bindings rebound to the live relocator, [migration/elf-loading.md](migration/elf-loading.md) step 1), each group followed by a build of every Cargo profile and every `nonos-mk-*` recipe; the audit's in-place fixes | none | all |
| 1. Userland-only moves | Signals reduced to exit codes; `nonos_start` and self-relocation for every capsule, the kernel's relocator and RELRO removed; vfs namespaces with today's IPC; the verification crate `nonos_admit` and a userland verifier capsule beside the kernel's gate; the registry rule, the kernel family-fact call and grant checks in the anonymity fronts, and the personality's Network removed; capsules call the crypto capsule directly and the crypto proxy is deleted; the ceiling fails closed; the floor's NV indices protected by the owner and lockout authorisations, with a floor shadow in the volume header | Phase 0 | [signal-delivery](migration/signal-delivery.md) 1 to 3; [elf-loading](migration/elf-loading.md) 1 to 6; [vfs-guarded-trees](migration/vfs-guarded-trees.md) 1 to 5; [spawn-gate](migration/spawn-gate.md) 1 to 5; [tpm-rollback-floor](migration/tpm-rollback-floor.md) 1 and 2; [network](12-target-network-authority.md) 1 to 4 |
| 2. Services out on today's interface | Init in ring 3, spawning through `MkCapsuleLoad` with the boot package mapped read-only; storage, volume, tpm, attest, procmgr, installer and settings as userland services; the kernel-side IPC clients and `services/lifecycle` deleted; interim verdicts | Phase 1 | [elf-loading](migration/elf-loading.md) 8; [spawn-gate](migration/spawn-gate.md) 6; [tpm-rollback-floor](migration/tpm-rollback-floor.md) 3 to 5; [11-target-services-out-of-kernel.md](11-target-services-out-of-kernel.md) |
| 3. The object model | Capability tables, endpoints with badges and transfer, notifications, address-space, thread and process capabilities, MemoryObjects, Verdicts, Device, IommuDomain, IrqHandler and PortRange objects, class masks; the new 46 calls beside the old 196 | Phase 2 | [07-target-object-model.md](07-target-object-model.md), [08-target-syscall-surface.md](08-target-syscall-surface.md) |
| 4. Services onto capabilities | The userland loader and `process_start` measurement; the names service; devmgr and acpi with device objects; display on MemoryObjects; netgrant taking the network grant decision from the fronts, with factories and streams as capabilities; vfs namespaces as badged capabilities; fault endpoints; the kernel's verification deleted | Phase 3 | [elf-loading](migration/elf-loading.md) 7; [spawn-gate](migration/spawn-gate.md) 7 and 8; [vfs-guarded-trees](migration/vfs-guarded-trees.md) 6; [signal-delivery](migration/signal-delivery.md) 4 and 5; [network](12-target-network-authority.md) 5; [10-target-device-authority.md](10-target-device-authority.md) |
| 5. Remove the old interface | The 196 old calls, the per-token allowlist, the capability predicate table, the token MAC and the in-kernel registry deleted | Phase 4 | [08-target-syscall-surface.md](08-target-syscall-surface.md) |

Phases 1 and 2 can overlap per subsystem: each line in them is independent of the others except that init in ring 3 needs the verifier capsule and the boot package.

## The measurable target

The target is stated in the two counts of [method.md](method.md), over the code compiled for x86_64, and in system calls.

| Milestone | Ring-0 phys lines (x86_64) | Ring-0 code lines | System calls |
|---|---:|---:|---:|
| Today, `551bbd34` (measured) | 349,816 | 186,747 | 196 |
| After phase 0 (measured: today less class D "dead", "inert" and "uncompiled") | 284,852 | 146,781 | 196 |
| After phase 5 (budget below) | 132,410 | about 67,000 | 46 |

The phase 5 figure is a budget, the sum of today's K and R lines per area with each R area reduced to the target its design states. It is checked by running `classify.py` on the tree after phase 5, where every file left in `src/` must be class K.

| Area | K today | R today | Target | Basis |
|---|---:|---:|---:|---|
| arch: x86_64 privileged core and arch-neutral layers | 33,923 | 0 | 33,923 | unchanged |
| arch: ACPI parser and tables, hypervisor, duplicate IDT, time | 0 | 18,536 | 5,450 | fixed-table reader 2,000; SynIC and hypercall setup 800; time 2,500; ACPI glue 150; duplicate IDT deleted |
| boot | 3,044 | 0 | 3,044 | unchanged |
| bus | 0 | 1,181 | 0 | enumeration in drivers/pci; BAR assignment and AER to devmgr |
| capabilities | 1,327 | 0 | 2,500 | object table, derivation tree, classes |
| crypto (RNG, entropy, constant time, core) | 3,797 | 0 | 3,797 | unchanged |
| drivers/pci | 7,196 | 0 | 6,200 | statistics and duplicate tables removed |
| entry, crate root, time, log | 1,689 | 0 | 1,689 | unchanged |
| hardware/broker | 16,189 | 11,018 | 11,000 | mechanism only (10-target-device-authority.md) |
| interrupts | 5,236 | 0 | 5,236 | unchanged |
| ipc | 1,511 | 0 | 1,800 | endpoints, notifications, capability transfer |
| kernel_core: init and stacks | 2,509 | 0 | 2,509 | unchanged |
| kernel_core: capsule_spawn | 0 | 2,765 | 600 | verdict, pinned table, measurement at start, flat init image |
| kernel_core: surface_registry | 0 | 4,637 | 0 | replaced by MemoryObject, counted under memory |
| memory | 20,285 | 2,193 | 21,835 | live allocators and paging; MemoryObject and page-table pool 800; sealed-page digests 150; guard and protection residue 600 |
| process: core, scheduler, exit, entry, caps | 10,618 | 0 | 10,618 | unchanged |
| process/foreign | 0 | 4,842 | 1,500 | syscall-redirect and fault endpoints, register read and write, cross-space copy |
| process/signal | 0 | 2,057 | 150 | fault kind to exit code |
| sched | 92 | 0 | 92 | unchanged |
| security: hardening, ceiling, observability, boot session, random | 3,055 | 0 | 3,055 | unchanged |
| security: attest_registry, zerostate | 0 | 1,166 | 700 | append-only measurement log and root 400; RAM wipe 300 |
| security: PCR 14 extend at locality 1 (new) | 0 | 0 | 300 | one TPM command path, from today's 5,674-line driver |
| smp | 5,216 | 0 | 5,216 | unchanged |
| sys | 5,203 | 941 | 4,800 | duplicate APIC init merged (-700); clock-page writer 300 |
| syscall | 12,995 | 2,613 | 5,500 | 46 calls: handlers 4,000, decode and check 1,500 |
| usercopy | 896 | 0 | 896 | unchanged |
| **Total** | **134,781** | **51,949** | **132,410** | |

The code-line figure applies today's K ratio of code to phys lines (68,469 of 134,781) to the budget.

Of the ring-0 lines that remain, the largest areas are platform code: the x86_64 privileged core (33,923, of which the IOMMU drivers are 10,937), memory management (21,835) and the device mechanism (11,000). They stay because they write the hardware's isolation state. The capability, IPC, address-space, thread and scheduling core is a fraction of the total; the next section treats it separately.

The kernel also compiles C today: `build.rs:42-43` builds ML-KEM and ML-DSA from `third_party/pqclean/` with `src/crypto/pqclean_support` (the ML-KEM-768 and ML-DSA-65 sources are 1,629 and 2,531 lines, plus 1,094 lines of FIPS 202). ML-DSA verification leaves ring 0 with the verifier in phase 4, and no live caller of ML-KEM was found; after phase 4 the kernel compiles no C.

## What a smaller kernel lets the project prove

The project has 158 Lean modules under `verification/lean/Nonos`, 232 `*_proofs` crates under `userland/` and a refinement table that says how each model binds to code (`verification/lean/REFINEMENT.md`). Three facts limit what those proofs can claim about the running kernel today:

1. **A global property needs every line that can break it.** "No capsule obtains authority it was not granted" is broken by any ring-0 bug that writes kernel memory, and ring 0 includes an ELF relocator (NKA-2026-020 shows one such write), certificate and manifest parsers, a STARK verifier, an ACPI table parser, a filesystem, a TPM driver and init. No proof of the capability model's arithmetic, however exact, establishes the property while those run in ring 0.
2. **Proofs bind to dead code.** Of the level 2 bindings, `Bounds` checks `src/elf/reloc` and `Seqlock` checks `src/sys/sync/seqlock` (`verification/lean/REFINEMENT.md:36`, `verification/lean/REFINEMENT.md:46`), neither of which runs; level 3 models `Rwlock`, `Signal` and `Fd`, `Vfs` (`verification/lean/REFINEMENT.md:82`, `verification/lean/REFINEMENT.md:91`, `verification/lean/REFINEMENT.md:94`) describe code that is dead or unreachable. Effort goes to models of code the kernel does not execute.
3. **The capability model is a bit test.** The level 1 extraction covers `src/capabilities` (`verification/lean/REFINEMENT.md:29`), which is the 44-bit word and its arithmetic. Whether the right bit was tested for the right object is outside that model, because there are no objects.

After the migration the project can state and prove properties that are not provable now:

| Property | Statement | Scope it needs | Today |
|---|---|---|---|
| Capability integrity | Every capability in every table was created by the kernel or derived from one with no more rights | The capability table, the six capability calls and IPC transfer, about 4,300 lines | Not statable: no objects |
| Ceiling | For every process, `class_mask ⊆ CEILING`, and no capability of a class outside its mask is ever in its table | Process creation and every transfer path | Checked at one spawn point (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:70-74`) and argued for grants |
| Revocation completeness | After `cap_revoke(h)`, no capability derived from `h` remains in any table | The derivation tree | Not statable |
| Address-space isolation | A user frame is mapped into an address space only through a MemoryObject capability its creator held | `as_map`, the page-table code, MemoryObject | Not provable: the kernel writes user frames from many places (`src/elf/loader/core/relocate.rs:106`, `src/elf/loader/core/load_segment/populate_page.rs:44-52`, the peer copies, the surface registry) |
| No unchecked cross-space store | Every ring-0 store into a user frame goes through the one page-safe primitive | A syntactic check over ring 0, plus a Kani proof of the primitive | Seven writers by the inventory ([01-current-state.md](01-current-state.md)) |
| Measurement binding | A process's recorded `D_load` is the digest of the executable span mapped when its first thread started | `process_start` | Implicit in one function's control flow |
| Device containment | After a Device capability is revoked, the device's DMA reaches no frame and its interrupts reach no thread | Revocation in the device objects, the IOMMU and IRQ code | Spread across the broker's 29,348 lines with its policy |

The first four properties concern the capability, IPC, memory-object and address-space core. Measured against the budget, that core (capabilities 2,500, IPC 1,800, the capability-facing part of the syscall layer, MemoryObjects and `as_map`) is on the order of 10,000 phys lines, which is the size of kernels that have been verified end to end. The platform code around it (APIC, IOMMU drivers, SMP bring-up, ACPI fixed tables) stays trusted and unproved for now, behind internal interfaces narrow enough to state what the core assumes of it: "the IOMMU maps exactly the domain's table", "an interrupt for a vector reaches the bound notification".

Three process changes go with this:

- **Bind proofs to live code only.** When phase 0 deletes `src/elf/reloc` and `src/sys/sync/seqlock`, their level 2 bindings must move to the code that replaces them or be removed; the refinement table should list, for each binding, the live caller that makes it matter.
- **Gate the corpus root properly.** NKA-2026-014 shows the axiom allowlist accepts any axiom. A smaller kernel makes "the corpus root covers ring 0" a meaningful claim, which makes the gate's correctness more valuable.
- **Widen the corpus root to everything the proofs rest on.** The root hashes the pinned toolchain, the axiom profile and the Lean sources under `verification/lean/Nonos` only (`verification/lean/proof-corpus-root.sh:57-66`; the file list is `find Nonos -name '*.lean'` at `verification/lean/proof-corpus-root.sh:63`). The extraction that the level 1 bindings rest on (`verification/lean/REFINEMENT.md:29`), the Aeneas output and its Lean project under `verification/extraction/`, and the corpus under `formal/` are outside it, so a change to any of them leaves the root unchanged. Decided ([15-decisions.md](15-decisions.md)): the root is widened to hash every directory a proof in the corpus depends on. The change lands together with the ELF deletion, and one root-change note for the next seal records both.

## Relation to the in-place fixes and the network grant

- **The audit's in-place fixes** are phase 0. [06-audit-checklist.md](06-audit-checklist.md) says for each finding whether the fix stays necessary after the migration; the page-safe store primitive, the fail-closed ceiling, the NV index protections, the boot-verdict rule and the vfs filters all stay.
- **The Linux network grant** (kernel facts, the anonymity fronts as the interim grant authority, no Network in the personality) is steps 1 to 4 of [12-target-network-authority.md](12-target-network-authority.md) and removes the run-again-with-network restart of the personality. The startup measurements also bear on where image hashing runs ([09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md), "The cost of measuring").
