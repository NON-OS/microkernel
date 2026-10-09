# capsule_load_proofs

Host test crate for the kernel's capsule-load certificate clock gate. It compiles one kernel
source file through `#[path]` and checks when a certificate validity window is enforced. It
has no dependencies (`Cargo.toml`).

## What is under test

`src/lib.rs` mounts:

- `../../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/validity_clock.rs` as
  `validity_clock`.

That file defines `validity_now_ms(now)`. It returns `Some(now)` when `now` is at or above
`MIN_PLAUSIBLE_UNIX_MS` (1_577_836_800_000, 2020-01-01 UTC) and `None` otherwise, so a clock
that still reads uptime since boot does not trip the `valid_from`/`valid_until` check. The
kernel calls it from `src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/spawn.rs` and
`src/syscall/microkernel/capsule_verify/verify.rs`, passing `crate::sys::unix_ms()`.

## What the tests check

`src/tests.rs` has two tests:

- `uptime_before_the_clock_is_set_skips_the_window`: 0, 5 s and about 16 h of uptime all
  yield `None`.
- `a_plausible_wall_clock_enforces_the_window`: one millisecond under the floor yields
  `None`; the floor itself and a later timestamp pass through unchanged.

## Running

```sh
cd userland/capsule_load_proofs
cargo test --release
```

At the time of writing this runs 2 tests. This crate is not named in
`.github/workflows/verify.yml`, so CI does not run it.

## Not covered

The tests check the threshold function only. Whether the callers then enforce the window
correctly, how `unix_ms()` is set from the RTC or firmware, and the signature and trust anchor
checks that still gate a load when the window is skipped are not exercised here.
