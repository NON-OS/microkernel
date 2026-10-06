# capsule_clock

`clock` is the desktop clock window: a digital and analog clock with the date, a stopwatch, a
countdown timer and a tab for setting the time of day. It is for any desktop user, and it holds
the TimeSet capability so that tab can correct the system wall clock. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

## Role

A `no_std` application on `nonos_app_skeleton` (`src/main.rs` calls `nonos_app_skeleton::run`).
The kernel embeds it under the feature `nonos-capsule-clock` through the mirror at
`src/userspace/capsule_clock/` (`CAPSULE_KERNEL_MIRROR`), whose `spawn.rs` also starts on-demand
instances on `app.clock.1` and `app.clock.2`.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x401819`, commented in `Capsule.mk` as
`CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate|TimeSet`:

- `0x000001` CoreExec: run.
- `0x000008` IPC: compositor, wm, input_router and toolkit.
- `0x000010` Memory: heap.
- `0x000800` GraphicsDisplayQuery and `0x001000` GraphicsSurfaceCreate: its window surface.
- `0x400000` TimeSet: the `mk_time_adjust` call made by Apply on the Set tab
  (`src/clock/event.rs`). `src/userspace/capsule_clock/spawn.rs` requests the same bit.

Endpoints: service `service:4730:app.clock`, reply `reply:4731:endpoint.app.clock.reply`, plus the
instance pairs `4842/4843` (`app.clock.1`) and `4844/4845` (`app.clock.2`) from
`CAPSULE_INSTANCE_ENDPOINTS`.

## Interface

It serves no IPC requests of its own; it draws a 360 by 440 window (`src/clock/manifest.rs`) and
takes only pointer clicks. `src/clock/tabs.rs` splits the bar into Clock, Stopwatch, Timer and Set.
`src/clock/state.rs` reads `mk_time_millis` and converts it with `src/clock/civil.rs`; while the
kernel has no wall time yet, the Clock tab says "The system clock is not set" instead of drawing
midnight (`src/clock/says.rs`). The stopwatch and timer run on `mk_uptime_ms`, the clock since
boot, so setting the time does not move them;
`src/clock/paint.rs` and the `paint_*.rs` files draw each tab, with the analog hands from
`src/clock/angles.rs` and the fixed-point sine table in `src/clock/fixed.rs`. `src/clock/app.rs`
ticks every 100 ms while the stopwatch or timer runs and every second otherwise.

## State and privacy

It keeps the current time, the selected tab, the stopwatch and timer (`src/clock/stopwatch.rs`,
`src/clock/timer.rs`) and the hour and minute being edited, all in memory. Nothing is saved: a new
window starts with a stopped stopwatch and a one minute timer. It reads no files and holds no
FileSystem, Network or Debug bit. Its only system side effect is the wall clock adjustment above.

## Build and test

- `make nonos-mk-clock` builds the ELF; `make nonos-mk-clock-sign` produces the certificate,
  manifest and trailer.
- `nonos-mk-host-trust-elfs` in `mk/20-build.mk` includes `$(clock_BIN)`.

`userland/apps_proofs` includes `civil.rs`, `says.rs`, `stopwatch.rs` and `timer.rs` and checks
the clock with no system time, the lines Apply shows for each answer the kernel gives, and that
the stopwatch and timer measure spans on the monotonic clock. `userland/clock_resolve_proofs`
tests the kernel's wall clock source picker, not this window.

## Not done yet

- Times are shown as UTC: `src/clock/state.rs` converts the raw millisecond count with no time
  zone.
- Apply sets only hour and minute on today's date, with seconds forced to zero. With no wall
  time there is no date, so Apply refuses and says so; each answer from `mk_time_adjust` is shown
  as a line on the Set tab.
- A finished timer turns the tab red (`src/clock/paint_timer.rs`) but makes no sound.
