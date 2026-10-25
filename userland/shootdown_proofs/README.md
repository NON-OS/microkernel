# shootdown_proofs

An exhaustive search of the x86_64 TLB shootdown protocol
(`src/memory/paging/manager/shootdown/`) over every interleaving of one round
with the CPUs it must reach. It also covers the APs that are brought up, or
given up on, while the round runs (`src/smp/init/ap_unit.rs`, `src/smp/ap/`).

The model is a state machine written from the code, not the code itself. Each
step is one atomic action that the code takes:

- the page-table write;
- `select` reading one descriptor;
- marking one target;
- a target taking its mark, flushing and acknowledging;
- an AP being started, woken by INIT, claiming its start and publishing
  `Online`;
- the boot CPU giving up on an AP before it claimed;
- the boot CPU asking an AP that claimed and missed its online deadline to
  park, the AP parking at a checkpoint of its bring-up, and INIT resetting it
  once it has (`src/smp/init/ap_unit.rs`, `src/smp/ap/park.rs`).

The hardware may fill or evict a TLB entry at any step on any CPU that runs
kernel code. INIT empties a TLB.

Three properties are checked in every reachable state:

- Once the round returns, no CPU that can still run kernel code holds the
  old translation.
- While the round waits, every outstanding acknowledgement belongs to a target
  that will answer it, so the wait ends.
- No AP can park while it holds a lock taken in `bring_up`.

## Results

Every machine from 2 to 4 CPUs is searched, with every number of APs online
at the start and both flush kinds (`invlpg` and a CR3 reload). Five CPUs are
searched once for the kernel's rules. With the park path in play, where the
boot CPU may ask any AP to park at any step of its bring-up, the step counter
multiplies the states (395 thousand at three CPUs), so that path is searched
up to three CPUs.

Each rule has a mutant that removes it, and the search must find a
counterexample for each one:

| Mutant | What breaks |
|---|---|
| target `Online` only, as the code did before | an AP that woke and cached the old page before the write keeps it after the round |
| no acknowledgement for an abandoned target | the wait never ends |
| select before the write | an AP that comes up in between keeps the old page |
| acknowledge before the flush | the round returns before the target flushed |
| CR4.PGE on the initiator with a CR3-reload flush | the GLOBAL kernel entry survives the reload |
| leave the set at the park request, before `Parked` and INIT | the AP still runs with the old page after the round |
| a checkpoint inside a lock | an AP can park holding it |

`scripts/check_ap_checkpoints.py` holds the kernel's `bring_up` to the
model's: every step followed by a checkpoint, and no lock held across its
statements.

Memory is sequentially consistent in the model. The code gets that from its
`SeqCst` fences. They sit before `select`, between marking a target and
sending to it, and between the boot CPU's `Starting` store and INIT.

## Run

```
cargo test --release
cargo clippy --release --tests -- -D warnings
```

13 tests, about 50 seconds on one core, most of it the five-CPU search.
