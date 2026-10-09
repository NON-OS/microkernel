# capsule_power

## Role

`capsule_power` is the userland power-management service. It exposes
reboot and shutdown to other capsules via IPC. Reboot lands real on
every x86 box (ACPI reset register if the FADT offers one, 8042
pulse, both again, port 0xCF9, triple fault as last resort). Shutdown
enters ACPI S5 with the SLP_TYPa/b read from the constant `\_S5`
package in the DSDT or an SSDT (PM1a/PM1b control, or the sleep control
register on hardware-reduced platforms). A firmware whose `\_S5` is not
a constant package (it would need an AML interpreter to evaluate) or has
no PM1 control block cannot be powered off; the kernel then parks the
CPU. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

The capsule is built and signed, but it has no kernel mirror
(`src/userspace/capsule_power` does not exist) and no spawn plan starts it,
so no running system carries it and nothing in the tree calls it.

```text
any capsule
    |
    | OP_REBOOT / OP_SHUTDOWN
    v
capsule_power -> mk_admin_reboot / mk_admin_shutdown
    |
    v
kernel admin_ops dispatcher
    |
    v
arch::x86_64::acpi::power_reboot::reboot()  (ACPI, 8042, ACPI, 8042, 0xCF9, triple fault)
arch::x86_64::acpi::power_sleep::shutdown() (S5 from the \_S5 package)
```

## Microkernel contract

- `MkIpcRecvFrom` on port `4448` reads power requests and the sender's pid.
- `MkIpcReply` returns each status.
- `AdminReboot` syscall invokes the kernel reboot path.
- `AdminShutdown` syscall invokes the kernel shutdown path. It does not
  return: the machine powers off, or parks when S5 is unavailable.
- `MkTimeMillis` reads the wall clock to track last-request timestamps.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_REBOOT` | 0x0002 | invoke `AdminReboot`; the system reboots |
| `OP_SHUTDOWN` | 0x0003 | invoke `AdminShutdown`; the system powers off |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x218`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x08 | IPC | recv + reply on port 4448 |
| 0x10 | Memory | bounded reply buffer |
| 0x200 | Admin | invoke `AdminReboot` / `AdminShutdown` |

`Debug` is **deliberately absent**: power transitions must never
leak to the serial surface.

## Privacy posture

| Invariant | How `capsule_power` honors it |
|---|---|
| NO LOGS | Debug cap dropped; no `MkDebug` calls. |
| NO TRACES | The capsule keeps a single `last_reboot_request_unix` and `last_shutdown_request_unix` for the current process lifetime only. Nothing persists across reboot. |
| EPHEMERAL | Zero files. |
| NOT LINUX | NCMP-style wire, Mk-tag syscall ABI. No `init(8)` semantics, no SysV runlevels. |
| PRIVACY MICROKERNEL | Three-bit cap mask. Admin is gated by `caps.can_admin()` at the kernel dispatch. Other capsules hold Admin too (the policy store, for one), and this capsule checks nothing about a sender beyond a non-zero pid, so any capsule that can reach its port could reboot the box. |

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
| Server loop | `server/runner.rs` |
| Reply builder | `server/respond.rs` |
| Per-op handlers | `server/handlers/{health,reboot,shutdown,router}.rs` |
| State (last request timestamps) | `state/mod.rs` |

## Wire format

20-byte header (magic `0x504F5752` = `'POWR'` LE,
version 1) followed by typed payload.

## State ownership

`PowerState` carries two `u64` last-request timestamps. No other
mutable state.

## Operating rules

- No inline comments past the 15-line license header.
- No `unsafe` past `_start`.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where non-trivial; `mod.rs` re-exports only.

## Release target

x86_64-nonos-user.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-power`
must compile clean.

## Release checklist

- [x] Every file ≤ 75 LOC
- [x] 15-line license header on every file
- [x] `Capsule.mk` mask `0x218` includes Admin
- [x] `Admin` cap is gated on `caps.can_admin()` in
      `src/syscall/contract/cap_table/admin.rs`
- [x] Kernel handler `admin_ops::handle` calls `power_reboot::reboot()`
- [x] S5 shutdown from the constant `\_S5` package (`src/arch/x86_64/acpi/hw/sleep.rs`)
- [ ] `_PTS(5)` before S5, which needs an AML interpreter
- [ ] Kernel mirror at `src/userspace/capsule_power/`
- [ ] Spawn wired through `src/userspace/init/spawn_plan/`

## Explicit non-goals today

- ACPI suspend (S3) requires AML for `_S3` package + GPE wakeup
  configuration. Out of scope until AML lands.
- CPU frequency scaling, thermal management, battery monitoring: all
  require ACPI table parsing beyond what is shipped.

## Verification

- Build: `make nonos-mk-power`; sign: `make nonos-mk-power-sign`.
- No boot exercises this capsule, since nothing spawns it.
- The kernel `power_reboot::reboot` has three independent fallback
  stages (ACPI reset reg, 8042, triple fault) so the reboot is
  unconditional regardless of board quirks.
