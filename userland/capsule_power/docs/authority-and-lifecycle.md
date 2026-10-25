# capsule_power: authority and lifecycle

Part of [the README](../README.md).

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x218`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x08 | IPC | recv + reply on port 4448 |
| 0x10 | Memory | bounded reply buffer |
| 0x200 | Admin | invoke `AdminReboot` / `AdminShutdown` |

Admin is broad: it admits `AdminReboot`, `AdminShutdown` and `AdminPolicyPush`
alike (`src/syscall/contract/cap_table/admin.rs`), and the first two admit no
narrower bit; they are this capsule's whole job. Four
catalogued capsules hold Admin: this one, `policy`, `install` and
`install-cli` (`tools/nix/capsules.json`). This one uses it only for the
desktop shell's process, and never calls `AdminPolicyPush`.

`Debug` is **deliberately absent**: power transitions must never
leak to the serial surface.

## Privacy posture

| Invariant | How `capsule_power` honors it |
|---|---|
| NO LOGS | Debug cap dropped; no `MkDebug` calls. |
| NO TRACES | The capsule keeps a single `last_reboot_request_unix` and `last_shutdown_request_unix` for the current process lifetime only. Nothing persists across reboot. |
| EPHEMERAL | Zero files. |
| NOT LINUX | NCMP-style wire, Mk-tag syscall ABI. No `init(8)` semantics, no SysV runlevels. |
| PRIVACY MICROKERNEL | Three-bit cap mask. Admin is gated by `caps.can_admin()` at the kernel dispatch. Other capsules hold Admin too (the policy store, for one); this capsule reboots or powers off for the desktop shell's process alone, and HELD lets only the shell address `power.off`. |

## Runtime lifecycle

1. `_start` initializes the heap.
2. `server::run()` enters the IPC loop on port `4448`.
3. Each request:
   - Healthcheck: status 0.
   - Reboot: record timestamp, reply with status 0, then issue
     `mk_admin_reboot()`. The reply lands first so the caller can
     audit the response before the box dies.
   - Shutdown: call `mk_admin_shutdown()`, which does not return.

## Failure model

- Heap init failure: exit `1`.
- Reboot: the kernel handler is real and fires regardless of which
  fallback stage triggers (ACPI reset reg / 8042 / triple fault).
  Worst case the box hard-resets; nothing is silently swallowed.
- Shutdown: divergent in the kernel. Where S5 cannot be entered (no
  constant `\_S5`, no PM1 control block) the CPU is parked after the
  zerostate wipe rather than returning to a wiped system.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + heap init | `main.rs` |
| Wire protocol | `protocol/*.rs` (decode, encode, header, ops, errno, mod) |
| Server loop, one inbox for both names | `server/runner.rs` |
| `power.off`, and who may power off | `off/mod.rs`, `off/rule.rs` |
| The sender's process | `capsule_acpi/src/server/caller.rs`, mounted |
| Reply builder | `server/respond.rs` |
| Per-op handlers | `server/handlers/{health,reboot,shutdown,router}.rs` |
| State (last request timestamps) | `state/mod.rs` |
