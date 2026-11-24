# The audit as a checklist

The audit (`NON-OS/auditing` at `0f5e792`) has 16 published findings, all open (`findings/published/README.md` in that repository). Each was read in full. For each: the boundary it shows failing, using the audit's six trust boundaries (firmware to kernel, kernel to capsule, capsule to capsule, capsule to device, machine to network, build to release); what the target design does to its class; and whether its in-place fix stays necessary after the migration.

"Removes the class" means the target makes the whole family of bugs impossible or harmless at the boundary, by construction. "Moves" means the same code, with the same kind of bug, runs elsewhere with less authority. "Unrelated" means the finding is about something the kernel's shape does not touch: a process, a document, a cryptographic parameter or another component.

## Summary

| Finding | Severity | Boundary | Target design | In-place fix after migration |
|---|---|---|---|---|
| NKA-2026-020 | medium | Kernel to capsule (2) | Removes the class | Loader check deleted with the loader; the page-safe store primitive stays for the remaining cross-space writes |
| NKA-2026-021 | low | Kernel to capsule (2) | Removes the class | Deleted with the kernel loader; the same checks belong in the userland loader |
| NKA-2026-031 | informational | Kernel to capsule (2) | Removes the class | The fix is the deletion |
| NKA-2026-015 | low | Capsule to capsule (3) | Removes the class, including the `OP_USAGE` residual the in-place fix leaves | Redundant once namespaces and stores are separate; kept as defence in depth |
| NKA-2026-049 | low | Capsule to capsule (3) | Removes the class | As NKA-2026-015 |
| NKA-2026-023 | informational | Build to release (6) | Removes the class at the kernel; the build check stays | Stays necessary |
| NKA-2026-026 | informational | Build to release (6) | Removes the class at the kernel; the dev path moves to the verifier | Stays until the verifier leaves ring 0, then deleted with it |
| NKA-2026-012 | informational | Firmware to kernel (1) | Moves | Stays necessary |
| NKA-2026-006 | low | Firmware to kernel (1) | Unrelated | Stays necessary |
| NKA-2026-046 | informational | Build to release (6) | Unrelated today: `verify_aggregate` has no caller in the kernel; verification code of its kind would run in the userland verifier | Stays necessary |
| NKA-2026-038 | informational | none (no caller) | Deleted | Not needed if deleted |
| NKA-2026-043 | low | Capsule to capsule (3) | Unrelated; already contained in userland | Stays necessary |
| NKA-2026-014 | low | Build to release (6) | Unrelated (process) | Stays necessary |
| NKA-2026-016 | informational | Build to release (6) | Unrelated (cryptographic parameter) | Stays necessary |
| NKA-2026-017 | informational | Build to release (6) | Unrelated (process) | Stays necessary |
| NKA-2026-039 | informational | none (documents) | Unrelated (documentation) | Stays necessary |

Seven of the sixteen sit on the kernel's own boundaries, on the capsule-to-capsule boundary that the kernel's object model can make structural, or on the spawn gate's build inputs, and the target removes all seven classes (020, 021, 031, 015, 049, 023, 026). Two concern the boot chain: 012 moves with its code, and 006 lies in the bootloader's NV definitions, which the kernel's shape does not touch. NKA-2026-046 is in code with no kernel caller, and NKA-2026-038 is dead code that phase 0 deletes. The remaining five (014, 016, 017, 039 and 043) are process, documentation, cryptographic-parameter and userland-codec findings that no kernel architecture changes.

## Findings on the kernel-to-capsule boundary

### NKA-2026-020: the relocator writes capsule-chosen bytes across a frame boundary

- **What fails.** Ring 0 performs a store whose address and value a capsule's ELF chooses. `apply_relative_relocations` checks `r_offset` only against the writable `PT_LOAD` range in virtual terms, translates one address and stores eight bytes through the direct map (`src/elf/loader/core/relocate.rs:99-107`). An unaligned offset near the end of a page spills into the physically next frame, which the allocator's ascending order and `populate_page` allocating the data frame before any table frame can make a page-table frame (`src/elf/loader/core/load_segment/populate_page.rs:38`).
- **Target.** Ring 0 never writes bytes chosen by a capsule image into a frame while loading. Capsules self-relocate in their own address space before they hold any capability ([migration/elf-loading.md](migration/elf-loading.md)); a bad relocation faults the capsule. The loader in userland maps file bytes into frames through ordinary mappings, so even a loader bug writes only the child's frames.
- **In-place fix.** The page-safe store primitive for every kernel write into another address space stays necessary: after the migration, ring 0 still copies IPC messages into a receiver, serves `copy_to_user` and copies for the foreign-process peer operations. The loader-specific alignment check is deleted with the loader. The frame-adjacency hazard is answered separately by a page-table frame pool ([07-target-object-model.md](07-target-object-model.md)).

### NKA-2026-021: `e_entry` overflow panics the kernel; the RELRO walk is unbounded under a global lock

- **What fails.** Ring 0 parses an untrusted image with overflow checks on and `panic = "abort"` (`Cargo.toml:1633`, `Cargo.toml:1641`), so a crafted `e_entry` halts the machine at `base_addr + header.e_entry` (`src/elf/loader/core/loader/base_addr.rs:33`). `enforce_relro` walks the `PT_GNU_RELRO` span page by page with no bound to the image (`src/elf/loader/core/relro.rs:44-54`) while the global `ELF_LOADER` lock is held (`src/elf/loader/global.rs:50-53`).
- **Target.** The parse runs in the userland loader. A panic ends the loader's handling of one spawn; RELRO protection is a `ProtectRange` call on an address space the loader holds, bounded by that address space and holding no kernel-wide lock.
- **In-place fix.** Needed until the kernel loader is deleted. The same checks (checked addition, entry inside an executable segment, RELRO inside the image) belong in the userland loader as correctness checks; they are no longer safety checks.

### NKA-2026-031: the syscall-return signal hook reads callee-saved registers live

- **What fails.** Ring 0 builds a user-visible frame from kernel register state: `read_callee_saved` copies `rbx` and `r12`-`r15` after the hook's prologue may have reused them (`src/arch/x86_64/syscall/manager/signal_return.rs:84-111`), although the assembly saved the user values (`src/arch/x86_64/asm/syscall.S:49-53`). The path is unreachable today because nothing installs a handler.
- **Target.** The kernel delivers no signal frames ([migration/signal-delivery.md](migration/signal-delivery.md)). Faults and kills terminate with a code; Linux guest signals are already built by the personality from the parked frame, which the kernel captures in the assembly entry path.
- **In-place fix.** Deleting the unreachable install path is the fix, and it is also the migration's first step.

## Findings on the capsule-to-capsule boundary

### NKA-2026-015 and NKA-2026-049: guarded trees in a shared store

- **What fails.** Isolation of the Linux personality's private trees is a filter each handler must remember. `named()` has no `OP_SEARCH` arm (`userland/capsule_vfs/src/server/fs_gate/paths.rs:32-43`); the search truncates before it filters (`userland/capsule_vfs/src/store/fdtable/search.rs:83`, `userland/capsule_vfs/src/server/handlers/search.rs:46-49`). `OP_DIRSTAT` is checked with `guarded()` on the named path only, so `/` passes, and the handler sums every file under the prefix (`userland/capsule_vfs/src/server/fs_gate/reach.rs:37-42`, `userland/capsule_vfs/src/server/handlers/dirstat.rs:48`).
- **Target.** Namespaces are capabilities: each client's requests are interpreted relative to the root its badged endpoint names, and the Linux trees are not under any other client's root. A handler that forgets a check can reach only the caller's own namespace ([migration/vfs-guarded-trees.md](migration/vfs-guarded-trees.md)).
- **In-place fix.** Done in place: every vfs operation is classified as named-path, subtree, handle or whole-store, unclassified operations are refused, and guarded nodes are removed before any count, sort or cap. It leaves one residual: `OP_USAGE` reports whole-store totals (`userland/capsule_vfs/src/store/fdtable/usage.rs:22-25`), from which the guarded totals follow by subtraction. The target closes that by giving the Linux trees their own store partition. After the migration the filters are redundant; keeping them costs nothing and catches a root-assignment mistake.

### NKA-2026-043: JPEG DHT overflow aborts the image codec

Unrelated to the kernel's shape. It is the target model working: a parser bug in a capsule ends that capsule, and nothing in the kernel is affected. The restart policy that brings the codec back moves from ring 0 to init in userland (`src/userspace/init/supervisor/watch_rule.rs:28`). The fix stays necessary.

## Findings on the spawn gate and the build

### NKA-2026-023: the committed image ceiling disagrees with the catalogue

- **What fails.** A missing, short or zero ceiling file reads back as unrestricted (`src/security/image_ceiling/value.rs:38-49`), and the build writes eight zero bytes when the file is absent (`build_trust_stage.rs:34`). A staging mistake weakens the bound instead of failing the build.
- **Target.** The ceiling stays in ring 0 as the one bound no userland verdict can raise, so its integrity matters more. The target makes it a required artefact generated from the catalogue, with no unrestricted fallback in a release build ([migration/spawn-gate.md](migration/spawn-gate.md)).
- **In-place fix.** Stays necessary; the target builds on it.

### NKA-2026-026: `nonos-dev-attest` compiles beside `nonos-production`

- **What fails.** A compile-time feature changes what ring 0 admits, and the guard is incomplete (`src/lib.rs:44-48`, `src/security/capsule_attest/path.rs:64-66`).
- **Target.** No admission policy is compiled into the kernel. Path-only admission is a property of a verifier build; the release kernel pins the release verifier's digest and grants verdict authority to nothing else, so a development verifier in a release image fails closed.
- **In-place fix.** Stays until the verifier leaves ring 0, then is deleted with the kernel's copy of the verification code.

### NKA-2026-046: the aggregate verifier panics on a crafted depth

Unrelated to the kernel today: the audit found no caller of `verify_aggregate` in the kernel, the bootloader or the shipped capsules, and the kernel links `nox_verify` and `nonos-attest-path` (`Cargo.toml:1475`, `Cargo.toml:1479`), not the aggregate verifier. The code is the kind the kernel runs in its spawn gate today (`nox_verify` and `nonos_attest_path` are called from `src/security/capsule_attest/path.rs:68-75` with `panic = "abort"`). In the target that verification runs in the userland verifier, where a panic ends the verifier's handling of one request, and the supervisor restarts it. The fix (reject depth over 64) stays necessary.

## Findings on the boot chain

### NKA-2026-012: the measured-boot verdict takes the last PCR 4 application as the loader

Moves. The replay keeps only the last `EV_EFI_BOOT_SERVICES_APPLICATION` digest (`nonos-boot-measure/src/tcg/replay.rs:59-60`) and the verdict treats it as the loader (`nonos-boot-measure/src/gate/verdict.rs:36-57`). In the target the verdict is computed by the tpm and attest services in userland, which read the same log; the bug moves with the code. The verdict's effect also moves: today a `Refused` verdict halts the boot in ring 0 (`src/kernel_core/init/entry/loader_refusal.rs:27-47`); in the target init refuses to start the boot set. The fix (apply the nsh PCR 4 rule) stays necessary.

### NKA-2026-006: the rollback floor's NV indices have empty authorisation

Unrelated to the kernel's shape: the weakness is in how the bootloader defines the NV indices (`nonos-bootloader/src/security/tpm_nv/floor_cmd.rs:25-46`). The kernel only reads the floor (`src/security/tpm/boot_reads/floor.rs:60-66`), and in the target that read moves to the tpm service. [migration/tpm-rollback-floor.md](migration/tpm-rollback-floor.md) covers what the kernel side must still do once the indices are protected. The audit's remediation of `POLICY_DELETE` on the base cannot be applied, because the TPM refuses it on an owner index; the spec protects deletion with the owner and lockout authorisations and adds a floor shadow. The fix stays necessary.

## Process, documentation and parameter findings

| Finding | Why the target does not touch it |
|---|---|
| NKA-2026-014 | The proof corpus's axiom gate is a script in `verification/lean`. A smaller kernel makes the corpus root a stronger claim, so the gate matters more. |
| NKA-2026-016 | A soundness figure of the STARK system in the pinned `nox_verify` dependency. The verifier moves to userland with the same parameters. |
| NKA-2026-017 | A CI gate for vendored PQClean. ML-DSA verification leaves ring 0 with the verifier, so a tampered PQClean would compromise the verifier instead of the kernel; the gate is still needed. |
| NKA-2026-038 | `SeqLock` has no caller and is deleted in phase 0 of [13-migration-plan.md](13-migration-plan.md). The clock page has its own seqlock writer (`src/sys/clock/page/layout.rs:48`), which stays and is not covered by the finding. |
| NKA-2026-039 | Counts and release state disagree across documents. The scripts in [method.md](method.md) can generate the kernel-size figures; the rest is documentation work. |
