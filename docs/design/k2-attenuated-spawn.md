# K2: a started capsule never holds more than its starter

Status: a design for after the 0.9.3 seal, with a minimal fix proposed for
0.9.3 itself. No kernel code yet.

## The problem today

A capsule that holds CoreExec, IPC and Memory can start another capsule,
and the child holds whatever its own signed manifest grants. The
starter's capabilities are never consulted. Every application holds those
three, so this reach is wide.

- **The gate.** `MkCapsuleLoad` needs only CoreExec, IPC and Memory
  (`src/syscall/contract/cap_table/mk_proc.rs:41-44`).
- **The artifacts.** The handler reads the ELF, certificate, manifest and
  trailer from the caller's memory (`src/syscall/microkernel/capsule_load/handle.rs:38-49`).
  The trust chain then verifies them: a signed, attested capsule passes.
- **The child's capabilities.** They are
  `requested & (required | optional)` from the child's manifest
  (`.../from_vfs/load/spawn.rs:66`), then
  `install_caps = required | (optional & requested)`
  (`src/security/capsule_manifest/verify/`). That set is held only to the
  authority's ceiling (`check_ceiling`), never to the caller's own set.
- **The installer as a deputy.** `capsule_installer`
  (`userland/capsule_installer/src/server/handlers/load_by_name.rs`) loads
  any installed capsule, by name, with the capabilities the requester
  chooses, for any sender that reaches its `installer` endpoint. It holds
  SpawnBroker, so the child is attributed to the requester, who can then
  `mk_proc_input` and `mk_proc_output` it. Which senders reach that
  endpoint is not yet established; that is the first thing to check.

So a capsule can gain, by proxy, capabilities it was never granted. One
example: it starts an installed vendor capsule that holds Network, and
drives it. For nsh, that means a role's ceiling does not bound what a
login can reach: an `auditor` login could reach the network.

## The rule

For a capsule `C` started on behalf of a parent `P`:

> `install_caps(C) ⊆ caps(P)`, unless `P` holds `Launch`, in which case
> `install_caps(C) ⊆ caps(P) ∪ launchable(P)`.

`P` is the attributed parent: the caller, or the requester when the
installer loads on its behalf.

- **Refusal.** A start outside that bound is refused with EPERM. It is
  never narrowed silently, because a capsule run with fewer capabilities
  than its manifest requires would fail later, and in a stranger place.
- **`Launch`.** A new capability, held only by the capsules that start
  applications for a person:
  - init;
  - the desktop shell;
  - the Terminal's launcher;
  - the installer, for its own installs.

  It is never in a market grant or a role ceiling, and the static
  cap-audit enforces both.
- **`launchable(P)`.** The ceiling a launcher may confer. For the shell and
  the Terminal, that is the user-visible application ceiling. For init, it
  is its boot plan.
- **`on_behalf_of`.** When the installer loads for a requester, the bound
  is the requester's own capabilities (or theirs plus their `launchable`,
  if they hold `Launch`). The installer's own SpawnBroker never widens it.

## What nsh needs from it

- **A session capsule never holds `Launch`.** So every capsule a login
  starts is bounded by the session's own capabilities, and those are
  bounded by the role's ceiling (`userland/nsh_roles`).
- **An escalation stays separate.** It is a new capsule started by
  `nsh.session` (K2's attenuated spawn, with an expiry, K3). The base
  capsule is never widened.

## Audit

Every start, granted or refused, writes one record to the audit log. It
holds:

- the parent's pid and capsule id;
- the child's capsule id and manifest hash;
- the requested and installed capabilities;
- the bound that was applied, and the outcome.

`audit` shows the records. A refused start is also counted against the
parent, so a capsule probing for capabilities shows up.

## Proofs

- **`kernel_proofs`.** The bound is a pure function of the masks. Every
  case is tested both ways:
  - parent ⊇ child;
  - parent ⊉ child, with and without `Launch`;
  - `on_behalf_of` with a requester that holds less than the installer.
- **Mutants.** The bound skipped; `Launch` taken from the caller rather
  than from the attributed parent; the union taken where the intersection
  belongs.
- **cap-audit.** No market or user capsule holds `Launch`.

## A minimal fix for 0.9.3, if you want one

In the `MkCapsuleLoad` handler: refuse when
`install_caps ⊄ caps(attributed parent)` and the parent does not hold
SpawnBroker. This reuses SpawnBroker in place of `Launch` until K2 adds it.

The installer then becomes the only general starter, so its `load_by_name`
must apply the same rule to its sender. That is a one-check change in
`load_by_name`: the sender's capabilities are known, from the attested
sender pid.

What could break: any capsule other than the installer, init or a
launcher that starts a capsule holding more than itself. Today the only
userland caller of `mk_capsule_load` is the installer
(`userland/capsule_installer/src/server/handlers/`). Every other capsule
asks the installer, so the remaining risk is the shell and the Terminal
launching applications through `load_by_name`. They would then need
SpawnBroker, or the rule's `Launch`, to keep launching applications that
hold Network.
