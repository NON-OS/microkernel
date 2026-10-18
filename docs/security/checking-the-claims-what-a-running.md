# Checking the security claims yourself: What a running machine shows about itself

Part of [Checking the security claims yourself](checking-the-claims.md).

## What a running machine shows about itself

Two of About's seven sections test claims while you watch: Proofs and Verify
(`nav_label`, `userland/capsule_about/src/about/section.rs:43-53`).

Proofs shows whether the running system is attested and whether its network
route is anonymous. Each half is Holds only when everything under it holds,
Broken when any piece is broken, and Unknown otherwise, and Unknown is never
drawn as a pass (`all`,
`userland/capsule_about/src/about/data/proofs/session.rs:103-111`). The boot
part is the bootloader's record as the kernel reports it, which the kernel does
not verify again (`recorded`,
`userland/capsule_about/src/about/data/verify/mod.rs:24-28`). The [proof
crate](../overview/glossary-peer-list.md#proof-crate) `attest_doc_proofs` tests
the verdict rule on the host; its 46 tests passed in the flake checks of this
commit.

Verify runs four checks against one read of the process table, and each can fail
(`checks`, `userland/capsule_about/src/about/data/verify/scan.rs:38-48`):

- `no running capsule can write a log`: no live process holds `Debug`.
- `only init holds admin authority`: no process but `init` holds `Admin`.
- `every running capsule carries a capability mask`.
- `this window holds exactly what its manifest declares`: the [capability
  word](../overview/glossary-boot-root-record.md#capability-word) About holds
  equals the mask it was built to ask for.

By the code, the first two read broken on a running desktop in this release. The
policy capsule, which the kernel starts at boot on every desktop image, holds
`Admin` (`REQUIRED_CAPS`, `src/userspace/capsule_policy/spawn.rs:32-34`). The
file service holds `Debug` on a standard, qemu or dev image, through
`serial_debug_cap` (`src/fs/vfs_capsule/spawn.rs:51-53`), and the Linux
personality holds it on every image while it runs (`LINUX_CAPS`,
`src/userspace/capsule_linux/spawn.rs:39-43`). A broken verdict on those two
lines is expected, not a sign of tampering.

Beside them a census counts the capsules, the holders of `FileSystem`, and the
holders of any of `Hardware`, `Driver`, `Mmio`, `Irq`, `Dma` and `Pio`
(`RAW_HARDWARE`, `userland/capsule_about/src/about/data/caps.rs:54`). Those are
counts, not checks.

The `attest` service, `systems.nonos.attest`, listens on port 4444
(`CAPSULE_SERVICE_ENDPOINT`, `userland/capsule_attest/Capsule.mk:8`) and answers
any capsule that holds `IPC`, the bit a service
[endpoint](../overview/glossary-dma-pool.md#endpoint) asks of senders by default
(`required_caps`,
`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:104`).
`OP_PROOF_INVARIANTS` returns seven invariants, each with a verdict
(`INVARIANTS`, `userland/capsule_attest/src/state/invariants/table.rs:29-72`).
Three are checked against the live process table: `NO LOGS`, `PRIVACY
MICROKERNEL` and `ADMIN IS INIT ONLY`. `NO LOGS` and `ADMIN IS INIT ONLY` count
the same holders as About's first two checks, so by the code they fail for the
same reasons. The other four, `NO TRACES`, `EPHEMERAL`, `NOT LINUX` and
`HYBRID-PQ SIGNATURES`, are settled where the service cannot look, and come back
unchecked, never as a pass (`NotAtRuntime`,
`userland/capsule_attest/src/state/invariants/probe.rs:32-36`). The service
holds `CoreExec`, `IPC`, `Memory` and `AttestRead` and nothing else
(`CAPSULE_REQUIRED_CAPS`, `userland/capsule_attest/Capsule.mk:17`), and nothing
in a reply is signed. `OP_PROOF_BOOT` returns the time and a fixed bootloader
label, not the loader's verdict (`bootloader`,
`userland/capsule_attest/src/server/handlers/proof_boot.rs:27-28`). Its verdicts
are a report from the kernel's own table, not a proof to anyone else.
