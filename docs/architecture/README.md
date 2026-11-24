# NØNOS kernel architecture: toward a small, checkable trusted base

This directory holds the architecture review that takes the NØNOS kernel from what it is at commit `551bbd34` (main, 2026-11-09) to a kernel that keeps in ring 0 only what has to be trusted there: capability enforcement, memory isolation, scheduling and IPC. Everything else moves to userland services or is deleted.

The documents describe the code as it is at `551bbd34`. Every claim about the current kernel cites `file:line` at that commit. Where the project's own documentation disagrees with the code, the disagreement is stated with both locations.

No code changes accompany this review. It is meant to be read and argued over before the structural work starts.

## Reading order

| File | Concern |
|---|---|
| [method.md](method.md) | How every number here was measured, with the scripts, so each one can be regenerated |
| [01-current-state.md](01-current-state.md) | What each part of `src/` does, its size, the privileged operation it needs and who calls it |
| [02-syscall-surface-today.md](02-syscall-surface-today.md) | The 196 system calls as they are today, and how one is checked |
| [03-docs-versus-code.md](03-docs-versus-code.md) | Places where the project's documents and its code disagree |
| [04-classification.md](04-classification.md) | Every subtree: stays in ring 0, moves to a named userland service, or is deleted |
| [05-trusted-computing-base.md](05-trusted-computing-base.md) | Kernel versus TCB: which userland components stay trusted, why, and how their trust is bounded |
| [06-audit-checklist.md](06-audit-checklist.md) | The 16 published audit findings against the target design |
| [07-target-object-model.md](07-target-object-model.md) | Capabilities, address spaces, threads, endpoints, frames, IRQ and IOMMU objects |
| [08-target-syscall-surface.md](08-target-syscall-surface.md) | The system calls the target kernel keeps, against today's |
| [09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md) | Spawn and boot trust once ELF parsing and attestation verification leave ring 0 |
| [10-target-device-authority.md](10-target-device-authority.md) | How drivers get MMIO, DMA and IRQ authority, and whether the broker is the right boundary |
| [11-target-services-out-of-kernel.md](11-target-services-out-of-kernel.md) | How the per-capsule IPC clients, the service registry and the init supervisor leave the kernel |
| [12-target-network-authority.md](12-target-network-authority.md) | Narrow network authority for the Linux personality |
| [13-migration-plan.md](13-migration-plan.md) | Ordering across subsystems, the measurable TCB target, and what a smaller kernel lets the project prove |
| [14-open-questions.md](14-open-questions.md) | Questions this review cannot settle alone |
| [15-decisions.md](15-decisions.md) | The decisions taken on the review's open points, 2026-11-23 |

Migration specifications, one per subsystem the audit flagged:

| File | Findings |
|---|---|
| [migration/elf-loading.md](migration/elf-loading.md) | NKA-2026-020, NKA-2026-021 |
| [migration/signal-delivery.md](migration/signal-delivery.md) | NKA-2026-031 |
| [migration/vfs-guarded-trees.md](migration/vfs-guarded-trees.md) | NKA-2026-015, NKA-2026-049 |
| [migration/spawn-gate.md](migration/spawn-gate.md) | NKA-2026-023, NKA-2026-026, NKA-2026-012 |
| [migration/tpm-rollback-floor.md](migration/tpm-rollback-floor.md) | NKA-2026-006 |

## Three structural findings

| Finding at `551bbd34` | Where the target answers it |
|---|---|
| Capabilities are ambient bits: a 44-bit word per process, checked against a per-token allowlist, with services asking the kernel about a pid | [07-target-object-model.md](07-target-object-model.md) replaces them with object capabilities held in per-process tables and passed over IPC; the 44 bits survive only as classes, which is how the image ceiling stays one monotone bound the kernel enforces. [05-trusted-computing-base.md](05-trusted-computing-base.md) gives the general pattern, the kernel as the source of facts and userland as the policy decider |
| Init runs in ring 0 as a function call | [11-target-services-out-of-kernel.md](11-target-services-out-of-kernel.md) makes init the root task, started from a flat image inside the signed kernel image; [09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md) gives the pinned set that lets the verifier and init exist before anything can be verified; phase 2 of [13-migration-plan.md](13-migration-plan.md) moves init to ring 3 before the object model exists |
| Proofs are bound to dead code while the live relocator has no model | The live relocator leaves ring 0 altogether ([migration/elf-loading.md](migration/elf-loading.md)); [13-migration-plan.md](13-migration-plan.md) requires every proof binding to name the live caller that makes it matter, and lists the properties a kernel of the target's size can state and prove that it cannot today |

## Terms

- **Ring 0** and **kernel** mean code compiled into the kernel image and run at CPL 0.
- **TCB** (trusted computing base) means every component whose misbehaviour can break a security property the system claims. The kernel is part of it; some userland services are too. [05-trusted-computing-base.md](05-trusted-computing-base.md) draws the line.
- **K, R, U, D, X** are the outcome classes of [04-classification.md](04-classification.md): stays in ring 0, stays in reduced form, moves to userland, deleted, not compiled for x86_64.
- **phys** lines are what `wc -l` counts. **code** lines exclude blank lines and comment-only lines, which matters here because every source file opens with a fifteen-line licence header.
