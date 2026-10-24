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
(`checks`, `userland/capsule_about/src/about/data/verify/scan.rs`):

- `nothing holding keys, input or secrets can log`: no live process holds
  `Debug` together with `InputSource`, `InputConsumer`, `Keyring` or
  `DeviceSecret` (`SECRETS`, `userland/capsule_about/src/about/data/verify/held_by.rs`).
- `admin: only init, policy, acpi, power, installers`: no process holds
  `Admin` but `init`, `policy`, `acpi`, `power`, `app.install` and
  `tool.install` (`ADMIN_HOLDERS`, same file).
- `every running capsule carries a capability mask`.
- `this window holds exactly what its manifest declares`: the [capability
  word](../overview/glossary-boot-root-record.md#capability-word) About holds
  equals the mask it was built to ask for.

Until 0.9.3 these two read `no running capsule can write a log` and `only init
holds admin authority`, and both were false on every desktop: the kernel asks
`Admin` of reboot, shutdown, the policy push and every ACPI call, so the policy,
ACPI and power capsules and the installers hold it by their signed manifests; a
standard, qemu or dev image grants `Debug` to every capsule whose manifest asks
for it (`capsule-serial-debug`), and the Linux personality holds it on every
image (`LINUX_CAPS`, `src/userspace/capsule_linux/spawn.rs`). They now state
what holds, and a holder outside the named set is still a red. Off the dev
profile the kernel withholds `Debug` from any capsule that also holds
`InputSource`, `InputConsumer`, `Keyring` or `DeviceSecret`, whatever its
manifest offers (`unlogged`,
`src/kernel_core/process_spawn/capsule_spawn/runner/secret_log.rs`), so the
input drivers and the proof capsule write nothing to the console on a standard
image; `kernel_proofs` holds every manifest in the tree to that rule.

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
(`INVARIANTS`, `userland/capsule_attest/src/state/invariants/table.rs`).
Three are checked against the live process table: `SECRETS NOT LOGGED`,
`PRIVACY MICROKERNEL` and `ADMIN IS NAMED`. `SECRETS NOT LOGGED` and `ADMIN IS
NAMED` count the same holders as About's first two checks, with the same list
of names; `exposure_proofs` holds the two readers to one rule. The other four, `NO TRACES`, `EPHEMERAL`, `NOT LINUX` and
`HYBRID-PQ SIGNATURES`, are settled where the service cannot look, and come back
unchecked, never as a pass (`NotAtRuntime`,
`userland/capsule_attest/src/state/invariants/probe.rs:32-36`). The service
holds `CoreExec`, `IPC`, `Memory` and `AttestRead` and nothing else
(`CAPSULE_REQUIRED_CAPS`, `userland/capsule_attest/Capsule.mk:17`), and nothing
in a reply is signed. `OP_PROOF_BOOT` returns the time and a fixed bootloader
label, not the loader's verdict (`bootloader`,
`userland/capsule_attest/src/server/handlers/proof_boot.rs:27-28`). Its verdicts
are a report from the kernel's own table, not a proof to anyone else.
