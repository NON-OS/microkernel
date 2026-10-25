# Lock waits that never end

Part of [scheduler and SMP](scheduler-and-smp.md).

Kernel code takes its scheduler, paging and registry locks with interrupts masked, through `lock_responsive` (`src/smp/responsive.rs`). A wait for one of them that never ends stops its CPU without a sound: no tick arrives to say so, and on a machine with one CPU nothing at all runs again. The surface registry's `GATE` was taken twice on one exit path, and the first process to exit stopped the boot in exactly this way.

So every wait through `lock_responsive` that lasts `REPORT_MS`, 1000 ms, prints one line on serial and keeps waiting (`src/smp/spin_watch.rs`, `src/smp/spin_wait.rs`). It is printed once per wait, never once per spin:

```text
[LOCK] waiting ms=1000 at src/kernel_core/surface_registry/pin/dma_held.rs:34 cpu=0 pid=0x1E held from src/kernel_core/surface_registry/pin/orphans.rs:37
```

- `at` is where this wait called `lock_responsive`.
- `cpu` and `pid` are the waiting CPU and the process it runs for.
- `held from` is where the lock was last taken. A lock somebody waits on is held, so its last taker is its holder. When another lock has used the same slot in the meantime, the line says `holder unknown`.

## A lock taken again by its own holder

When the holder's frame lies on the waiting CPU's own kernel stack, above the waiting frame, the waiter's own caller holds the lock, and the wait can never end. The line then starts:

```text
[FATAL] lock re-taken by its own holder, this CPU cannot go on: [LOCK] waiting ...
```

The waiter knows its own stack exactly on x86_64: the per-CPU block holds the top of the running process's 32 KiB kernel stack (`kernel_stack_top`), and stacks that are in use never overlap, so a lock held on any other CPU is never judged this way. A waiter on a stack the kernel does not know the bounds of, such as a boot stack, or any waiter on aarch64, adds `, likely re-taken by its own holder` instead, when the holder's frame is within 32 KiB above its own. Neither changes what the CPU does: it goes on waiting, as it would have without the report.

## What it costs

Each successful take writes the lock's address, its call site and a stack address into one of 64 slots (`src/smp/spin_holders.rs`, `fn note`): three plain stores and a release store, with no read of the interrupt controller. Nothing is cleared on release. The report itself runs only on a wait of a second, allocates nothing, and takes no lock but the serial port's. Nothing takes the serial lock through `lock_responsive`, and its holders only write bytes.

Measured on the host, an uncontended take and release with and without the note, 11 runs of 50 million each: median 9.93 ns before and 9.70 ns after, inside the spread between runs (8.9 to 10.6 ns).

## Limits

- Only locks taken through `lock_responsive` are watched. A plain `.lock()` or `RwLock::write()` that never ends is still silent.
- A wait shorter than a second is not reported. A held lock that is slow but finishes, under emulation for example, can print a line and carry on.
- The holder's site is the call site of `lock_responsive`. A helper that takes the lock for its callers names itself, not them.

## Proofs

`userland/kernel_proofs/src/spin_watch` compiles `src/smp/spin_watch.rs` and checks: the report comes once and only at the mark, a clock read behind the start is no wait, a holder above the waiter on its own stack is a re-take, a hold on any other stack never is, a waiter off its known stack says only "likely", the window is one process kernel stack, and neighbouring locks get their own slots.

## See also

- [Scheduler and SMP](scheduler-and-smp.md)
- [Panic and boot stop](panic-and-boot-stop.md)
