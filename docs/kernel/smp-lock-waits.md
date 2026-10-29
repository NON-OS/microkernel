# Lock waits that never end

Part of [scheduler and SMP](scheduler-and-smp.md).

Kernel code takes its scheduler, paging and registry locks with interrupts masked, through `lock_responsive` (`src/smp/responsive.rs`). A wait for one of them that never ends stops its CPU without a sound: no tick arrives to say so, and on a machine with one CPU nothing at all runs again. The surface registry's `GATE` was taken twice on one exit path, and the first process to exit stopped the boot in exactly this way.

So every wait through `lock_responsive` that lasts `REPORT_MS`, 1000 ms, prints two short lines on serial, and a third for a re-take, and keeps waiting (`src/smp/spin_watch.rs`, `src/smp/spin_wait.rs`). It is printed once per wait, never once per spin:

```text
[LOCK] cpu=0 waiting ms=1000 pid=0x0000000000000008 at src/kernel_core/surface_registry/pin/dma_held.rs:34
[LOCK] cpu=0 held from src/kernel_core/surface_registry/pin/orphans.rs:37
[FATAL] cpu=0 lock re-taken by its own holder, cannot go on
```

Those are the lines the GATE double take printed on a boot with one CPU before F1.

- `at` is where this wait called `lock_responsive`; each line names its CPU, since another CPU's lines may fall between them.
- `pid` is the process the waiting CPU runs for.
- `held from` is where the lock was last taken. A lock somebody waits on is held, so its last taker is its holder. When another lock has used the same slot in the meantime, the line says `holder unknown`.

## A lock taken again by its own holder

When the holder's frame lies on the waiting CPU's own kernel stack, the waiter's own thread holds the lock, and the wait can never end. Only that thread runs on that stack. Which of the two frames is higher is not asked: inlining can fold the holder and the waiter into one frame, as it folded `reclaim_dma` into `orphans::reclaim`. The third line then says so:

```text
[FATAL] cpu=0 lock re-taken by its own holder, cannot go on
```

The waiter knows its own stack exactly on x86_64: the per-CPU block holds the top of the running process's 32 KiB kernel stack (`kernel_stack_top`), and stacks that are in use never overlap, so a lock held on any other CPU is never judged this way (`src/smp/spin_verdict.rs`). Two cases add only `, likely re-taken by its own holder` to the `held from` line, when the holder's frame is within 32 KiB of the waiter's:

- the waiter's stack bounds are not known: a boot stack, or any waiter on aarch64 or riscv64, where nothing writes `kernel_stack_top`;
- the last take was a reader's. Another reader may still hold the lock after that one let go, so a reader's take is never judged a re-take.

Neither changes what the CPU does: it goes on waiting, as it would have without the report.

## What it costs

Each successful take writes the lock's address, its call site, a stack address and whether it was a reader into one of 64 slots (`src/smp/spin_holders.rs`, `fn note`): four plain stores and a release store, with no read of the interrupt controller. Nothing is cleared on release. The report itself runs only on a wait of a second and allocates nothing. It is bounded, and takes only the serial lock: nothing takes the serial lock through `lock_responsive`, and its holders only write bytes.

Measured on the host, an uncontended take and release with and without the note, 11 runs of 50 million each: median 8.94 ns before and 8.63 ns after, inside the spread between runs (8.3 to 11.9 ns).

## Limits

- Only locks taken through `lock_responsive` are watched. The cryptoblock's `PENDING` and the import stream's `LIVE` are among them. The /data volume's `VOLUME` is not a spin lock any more: it is a trip lock (`src/services/lifecycle/trip_lock.rs`), which a holder may keep across the disk's round trips, whose waiters rest instead of spinning, and whose holder, killed asleep, is swept by the next caller (`src/fs/blockfs_volume/state.rs`). A plain `.lock()` or `RwLock::write()` that never ends is still silent.
- A wait shorter than a second is not reported. A held lock that is slow but finishes, under emulation for example, can print a line and carry on.
- The holder's site is the call site of `lock_responsive`. A helper that takes the lock for its callers names itself, not them.

## The paging manager's lock

Main `30a4b58c4` printed up to 80 `[LOCK]` lines a boot on four CPUs, every waiter at `api/address_space.rs:46` (`lookup_asid_for_process`) or `map_page.rs:35`, every holder at `address_space.rs:22` (`create_address_space`) or `mapping_in_asid.rs:34` (`map_page_in_asid`), and up to 44 services stopped answering ([evidence](../release-evidence/0.9.3/regression-30a4b58c4-paging-lock/README.md)). Three things together made it:

- `PAGING_MANAGER` was a plain `spin::Mutex`, which goes to whichever CPU's try lands first after a release. A mapping loop lets go and takes again page by page and nearly always won that race, so a CPU spinning from further away lost it for a second and more.
- Every system call's capability check (`Capability::resolve`) and every context switch (`switch_to_process_address_space`, `thread_asid::publish`) took that lock to look an address space up.
- `create_address_space` and a few lookups held it with interrupts open.

`0448b98c3` (MkGetPid for every capsule) only raised the traffic: the display driver's real pid made its surface paths run. Now:

- `PAGING_MANAGER` is a `TicketMutex` (`src/smp/ticket.rs`): each taker draws a number and is served when it comes up, so none waits behind more than the takers that asked first. `lock_responsive` takes it as it takes a spin mutex (`src/smp/responsive_lock.rs`), with the same shootdown service and the same report.
- Who owns which address space is also kept in a directory read without the lock (`src/memory/paging/manager/directory/`): 1024 slots, each a sequence lock whose only writer is the manager lock's holder. Switches, capability checks and the asid lookups read it; only if it ever runs out of room does a miss ask the manager.
- Every holder of the manager lock masks interrupts. A number drawn by code an interrupt has stopped would hold up every CPU behind it.
- A switch no longer writes a manager-wide "active table": on more than one CPU it was whichever CPU switched last. Walks of the running space read the root register.

`userland/kernel_proofs/src/ticket_lock` proves the order: numbers served as drawn, a holder taking again comes after a waiter, a try never jumps the line or leaves a number behind, one holder at a time across four threads, and a waiter behind three hogs gets in within three holds. `userland/kernel_proofs/src/asid_directory` proves the directory: found by asid, pid and root until withdrawn, the lowest asid answers as the manager's table did, slots are reused, an overflow is known, and readers on three threads never see half of a writer's change across 20,000 of them.

## The static check

`nonos-ci/check-lock-retake.py`, run by the static checks, reads `src/` and fails when a lock is held across its own take: taken again in the same function, or held across a call to a function that takes it, directly or through further calls. It reads by name. A lock is a `static` `Mutex` or `RwLock` whose name no other static has. A call is followed when it names, as `f(` or `a::b::f(`, a function no other function shares a name with. A guard is held as Rust 2021 holds it: a `let` of the take itself to the end of its block or its `drop`, a `match`, `if let` or `while let` on a take through its body, any other take to the end of its statement or of a plain `if` condition. Method calls, function values, trait calls and `Drop` are not followed; a re-take through one of those is left to the runtime report above. Its fixtures (`nonos-ci/lock_retake/fixtures/`) run first, and every rule in it has a fixture that fails without it. On the source before F1 it names `orphans.rs:37: GATE held across reclaim_dma()`, the deadlock above, and found one more: `fs/storage/quota.rs`'s `update_exceeded_timestamp` read the quota under its own write lock; it was decided under the one guard, and the module has since gone, as nothing called it.

## Proofs

`userland/kernel_proofs/src/spin_watch` compiles `src/smp/spin_watch.rs` and `src/smp/spin_verdict.rs` and checks: the report comes once and only at the mark, a clock read behind the start is no wait, a holder anywhere on the waiter's own stack is a re-take whichever frame is higher, a hold on any other stack never is, a reader's take is at most "likely", a waiter off its known stack says only "likely", the window is one process kernel stack, and neighbouring locks get their own slots.

## See also

- [Scheduler and SMP](scheduler-and-smp.md)
- [Panic and boot stop](panic-and-boot-stop.md)
