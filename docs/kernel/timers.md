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
