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
