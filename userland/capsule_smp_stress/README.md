# capsule_smp_stress

Holds every core busy with the three kinds of wake the scheduler must never
lose, and says on the log whether one was lost:

| workers | what they do | a fault is |
|---|---|---|
| 4 futex pairs (8 threads) | take turns moving a shared word, each waking the other | a turn that comes back 1 s or more after the hand-off (`stalls`) |
| 2 IPC pairs (4 threads) | one sends a sequence number to the other's `proc.<tid>` inbox, which sends it back | a reply 1 s or more late, or none within 2 s (`stalls`) |
| 4 sleepers | timed sleeps of 1 to 7 ms, which only the tick sweep ends | a sleep that ends 50 ms or more past its deadline (`late`) |

A call the kernel refuses counts under `errors`. The run passes when every
kind of work moved and `stalls`, `late` and `errors` are all zero.

## Running it

From the Terminal: `smp_stress 3600` runs one hour, `smp_stress 600` ten
minutes; with no number it runs one hour. Every minute it prints

```
[SMP-STRESS] t_s=60 futex=... ipc=... sleeps=... stalls=0 late=0 errors=0 max_futex_ms=... max_ipc_ms=... max_sleep_over_ms=...
```

and at the end `[SMP-STRESS] PASS ran_ms=...` or `[SMP-STRESS] FAIL ...`.
`ran_ms` is the time measured from the start to the stop, not the time asked
for. With Debug granted the same lines go to the serial log, where
`log SMP-STRESS` finds them.

## Status

Not yet part of the image. A capsule is signed with its own publisher key,
and that key is minted by the owner, so the build does not include this
`Capsule.mk` until it exists. To wire it in once it does: add
`include userland/capsule_smp_stress/Capsule.mk` beside the
`capsule_std_proof` line in `mk/20-build.mk`, and the four
`/capsules/smp_stress.*` entries beside `std_proof`'s in
`NONOS_STORE_DEMO_ENTRIES` in `mk/40-run.mk`.

The host proofs for what counts as a fault, the verdict, the run length and
the printed lines are in `userland/smp_stress_proofs`.
