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

## The tick

`install_on_bsp` registers the timer handler and programs the boot CPU's local APIC timer at 100 Hz, and each other CPU calls `install_on_ap` for its own (`src/arch/x86_64/interrupt/apic/preemption/install.rs:25-38`). A failure on the boot CPU stops the boot with `preemption timer install failed`.

`setup_timer` puts the timer in periodic mode with a divider of 16 and prints `[APIC] Setting up timer at 100 Hz` (`src/sys/apic/local/timer.rs:28-47`). Its count comes from `calibrate_lapic_ticks_per_ms`, which lets the timer run against a 10 ms TSC window (`src/sys/apic/local_calibrate/calibrate_lapic_ticks_per_ms.rs:26-37`). The result is clamped between `LAPIC_TICKS_PER_MS_MIN`, 500, and `LAPIC_TICKS_PER_MS_MAX`, 60 000, per millisecond (`src/sys/apic/local_calibrate/consts.rs:48-49`). The TSC rate it trusts comes from `accurate_tsc_hz`: CPUID when it lies between 300 MHz and 6 GHz, else the timer module's rate in that band, else 2 GHz (`src/sys/apic/local_calibrate/accurate_tsc_hz.rs:19-32`).

Each tick enters `timer_tick`, which signals end-of-interrupt to the local APIC first, so the timer keeps firing while a switch runs another task (`src/arch/x86_64/interrupt/apic/preemption/tick_handler.rs:26-35`). Then `on_timer_interrupt` runs (`src/interrupts/timer/tick.rs:17-72`):

1. It records this CPU's tick time in its per-CPU block.
2. On the boot CPU only, it advances the machine's tick count.
3. It runs the scheduler tick, wakes sleepers whose deadline passed, and finishes deferred wakes.
4. On the boot CPU only, it runs the paced work.
5. If the tick interrupted user mode and a reschedule is due, it switches; see [scheduler and SMP](scheduler-and-smp.md).

The paced work in `paced_work` runs alarms and polls the ACPI power button every 10 ticks, updates the load averages every `LOAD_SAMPLE_TICKS`, 500 ticks, and polls the IOMMU for faults (`src/interrupts/timer/clock.rs:32-58`).

## Keeping the tick alive in idle

An idle CPU halts and waits for the next tick. The idle-timer code records that on parts with C1E an enhanced halt can stop the local APIC timer, and the tick with it. The idle-timer `init` handles this at boot (`src/arch/x86_64/interrupt/apic/idle_timer/init.rs:26-47`):

- If CPUID reports ARAT, the timer runs in every C-state and `hlt` is safe.
- Otherwise, on Intel, it clears the C1E enable bit in `MSR_IA32_POWER_CTL`, `0x1FC` (`src/arch/x86_64/interrupt/apic/idle_timer/consts.rs:22-23`).
- Otherwise `halt_safe` reports false and the idle loop spins instead of halting, which costs power and keeps the tick.

## Wall clock and the time calls

At boot `sys::clock` takes its epoch from the loader's UEFI time, else from the timer module's RTC reading, else from the RTC directly, through `resolve_epoch_ms` (`src/sys/clock/core/init.rs:39-47`). `unix_ms` is that epoch plus the elapsed time plus a correction offset (`src/sys/clock/core/time.rs:51-62`).

| Call | Number | Returns |
|---|---|---|
| `MkTimeMillis` | `0x534D544D` | the wall clock in Unix milliseconds; -61 until the clock has a rate and an epoch |
| `MkTimeMonotonic` | `0x4E4F4D4D` | milliseconds since boot, never adjusted, so it never goes backwards |
| `MkTimeRtc` | `0x5452544D` | the RTC date and time as year, month, day, hour, minute, second; -61 with no RTC |
| `MkTimeAdjust` | `0x4441544D` | sets the correction so the wall clock reads `correct_ms`; -22 for a value before 2025-01-01 or after 2100-01-01 |

The numbers are the tags `SYS_TIME_MILLIS`, `SYS_TIME_MONOTONIC`, `SYS_TIME_RTC` and `SYS_TIME_ADJUST` (`src/syscall/microkernel/numbers.rs:44-47`). The handlers are `sys_time_millis`, `sys_time_monotonic`, `sys_time_rtc` and `sys_time_adjust` (`src/syscall/microkernel/time.rs:33-95`). The first three need only a valid token. `MkTimeAdjust` needs the `TimeSet` [capability](../overview/glossary.md#capability), checked by `can_set_time` (`src/syscall/contract/cap_table/mk.rs:80`, `abi/syscalls.toml:850-854`). The kernel has no network time client of its own; a [capsule](../overview/glossary.md#capsule) that holds `TimeSet` can correct the clock.
