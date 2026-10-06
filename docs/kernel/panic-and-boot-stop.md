# Panic and boot stop

What the NONOS kernel does when it cannot go on, what the panel and the [serial console](../overview/glossary.md#serial-console) show in each case, and what a person can do next.

## At a glance

| Event | Panel | Serial console | Other CPUs |
|---|---|---|---|
| A boot step fails | notice band `NONOS BOOT STOPPED` | `[FATAL]` line | not started yet, or idle |
| Install asked, no installer | notice band | `[ERROR]` line | keep idling |
| Bootloader not admitted | notice band | `[ERROR]` line | keep idling |
| Handoff refused | breadcrumb strip, VGA text | `[NONOS] Handoff FAIL` | not started yet |
| Kernel panic | red band `KERNEL PANIC` | panic message | stopped by NMI |
| Kernel CPU exception | nothing new | `[PANIC xx]`, after `[TRAP xx]` for some | not signalled |
| Fault in a [capsule](../overview/glossary.md#capsule) | nothing | `[TRAP xx]` for `PF`, `GP`, `UD` | unaffected; the capsule ends |
| Kernel heap exhausted | VGA text | `[OOM]` lines | not signalled |
| [TLB shootdown](../overview/glossary.md#tlb-shootdown) timeout | nothing new | `[FATAL]` line | stopped by NMI |

Every stop ends in a loop that masks interrupts and halts the CPU, as `halt` does (`src/arch/x86_64/abi.rs:30-36`). NONOS does not restart the machine on its own. Nothing in the kernel arms or disarms the chipset watchdog that `detect_tco_watchdog` could find (`src/arch/x86_64/watchdog/mod.rs:24-25`), so a watchdog that firmware left running is not the kernel's to stop. Whether any firmware does that was not tested in this release.
