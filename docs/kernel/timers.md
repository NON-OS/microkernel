# Timers and clocks

Which hardware counters the NONOS kernel reads, how it learns their rate, where the scheduler tick comes from, and what each time system call returns.

## The sources at a glance

| Source | What NONOS uses it for |
|---|---|
| CPU cycle counter, read by `read_time_counter` (`src/arch/time_counter.rs:26-30`) | all elapsed time; the TSC on x86_64, `CNTPCT_EL0` on aarch64 |
| Local APIC timer, at `TICK_HZ` (`src/arch/x86_64/interrupt/apic/preemption/install.rs:25`) | the 100 Hz tick on every CPU |
| PIT, through `calibrate_with_pit` (`src/arch/x86_64/time/tsc/calibration/pit.rs:24-25`) | measuring the TSC rate when CPUID does not give it |
| ACPI PM timer, through `calibrate_with_pm_timer` (`src/arch/x86_64/time/tsc/calibration/pmtimer.rs:45`) | measuring the TSC rate once the ACPI tables are read |
| CMOS real-time clock, through `unix_timestamp` (`src/arch/wall_clock.rs:28-36`) | the date at boot when the loader gave none, and the `MkTimeRtc` call |
| HPET, found by `detect_hpet` (`src/arch/x86_64/time/hpet.rs:49`) | nothing on the boot path in this release |

## Elapsed time

Elapsed time is counter ticks since an anchor, divided by the counter's rate. `anchor` latches the rate and the current reading once, early in `init_core_systems` (`src/time/boot.rs:31-35`, `src/boot/main/core_init/init_core_systems.rs:27-31`). `now_ns` multiplies in 128 bits, so it does not wrap, and `ticks_since_anchor` saturates at zero, so a counter read below the anchor counts as no time passed (`src/time/now.rs:33-45`, `src/time/boot.rs:40-42`). Until a rate is known, `now_ns` divides by the assumed rate below; the rate is filled once and then never replaced.

`timestamp_millis` is `now_ns` in milliseconds, and it is the unit of every sleep deadline and timeout in the kernel, the [futex](futex.md) included (`src/time/units.rs:23-28`).

## Learning the counter's rate

A PC has no register that states the TSC rate for every part. `time_counter_hz` tries CPUID, then the PIT (`src/arch/time_counter.rs:39-52`); the ACPI step adds a third source later, and a fixed guess is the last resort:

1. CPUID leaves 0x15 and 0x16, read by `get_cpuid_frequency` and accepted between `MIN_FREQUENCY`, 100 MHz, and `MAX_FREQUENCY`, 10 GHz (`src/arch/x86_64/time/tsc/calibration/cpuid.rs:24-37`, `src/arch/x86_64/time/tsc/constants.rs:23-25`). AMD parts and older Intel parts report no rate.
2. A measurement against the PIT's fixed 1.193182 MHz `PIT_FREQUENCY`: five samples of 50 ms, at least three good ones, and the median wins (`src/arch/x86_64/time/tsc/calibration/pit.rs:24-110`). Every wait is bounded in TSC ticks, so a PIT that is gated off ends the attempt instead of hanging the boot.
3. Once the ACPI tables are parsed, `calibrate_against_pm_timer` measures against the 3.579545 MHz ACPI PM timer if no rate is known yet (`src/boot/main/core_init/acpi_tables.rs:42-58`). It prints `[TIMER] TSC measured against the ACPI PM timer: N MHz`. `set_counter_hz_if_unknown` only fills an empty rate and never replaces one already latched (`src/time/boot.rs:49-55`).
4. With no reference at all, the rate is `ASSUMED_HZ`, 2.5 GHz (`src/time/now.rs:26`), and the console says `[TIMER] no TSC reference (CPUID, PIT, PM timer); rate is a guess`. Time then runs at the wrong speed, but it runs.

On aarch64 the rate is read from `CNTFRQ_EL0` and is exact.

The loader adds one more figure. `estimate_tsc_frequency` counts TSC ticks across a 10 ms firmware stall, with 2 GHz when that fails, and passes the result in the handoff (`nonos-bootloader/src/handoff/prepare/security/estimate_tsc_frequency.rs:19-28`).

Three kernel modules keep a rate, and all three read the same counter:

- `crate::time`, for elapsed time, sleeps and the scheduler, uses the order above.
- `sys::clock`, for the wall clock and the monotonic call, takes the loader's estimate first, then the timer module's rate, then a fresh calibration, through `resolve_tsc_hz` (`src/sys/clock/core/init.rs:22-47`).
- `sys::timer::tsc` takes the CPUID or PIT rate in `init_default`, or 2.5 GHz when both fail (`src/sys/timer/tsc/init.rs:23-55`). A later PM timer measurement replaces that value through `TSC_FREQ_HZ` (`src/boot/main/core_init/acpi_tables.rs:47-49`).

The pickers and the TSC arithmetic are compiled into the `clock_resolve_proofs` [proof crate](../overview/glossary.md#proof-crate), starting with the `resolve` module (`userland/clock_resolve_proofs/src/lib.rs:6-7`). It passes its 16 tests on this commit.
