# capsule_power: wire, state and release

Part of [the README](../README.md).

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
