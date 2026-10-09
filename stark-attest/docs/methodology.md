# How every number in this repository is produced

Every performance and security figure this project publishes lands in the
repository as JSON, produced by a binary anyone can run, carrying the
conditions that produced it. This page is the discipline behind those files:
what is measured, how, what the guards are, and one worked example of the
guards catching this project's own benchmark lying. If you adopt one thing
from this repository, the candidates are the constants-as-rule pattern in
[poseidon-spec.md](poseidon-spec.md) and this page.

## The records

| File | Producer | Contents |
|---|---|---|
| `bench/results.json` | `emit-bench` | timing: enrolment, proving, verification, rejection, across set and artifact sizes |
| `bench/soundness.json` | `soundness` | security: soundness bits under both the provable floor and the conjectured regime, derived from the compiled parameters |
| `bench/hash-study.json` | `hash-study` | the measurement-hash architecture comparison |

The `measure` job in CI runs all three on every push and commits the results
on main, so the canonical numbers are produced by a public runner from public
code, not by an author's machine. A locally produced record is still a valid
record, because every record names its host; it is simply not the canonical
one.

## Rules for timing

**Every timed result is kept alive.** Each measured call sits inside
`std::hint::black_box`, because an optimizer that can see a result is dropped
is entitled to skip producing it, and a benchmark of skipped work reports a
fantasy. This rule exists here because it was violated here; see the worked
example below.

**Medians, not means.** One descheduled iteration moves a mean and does not
move a median. The raw per-iteration nanoseconds are recorded alongside, so a
reader can recompute anything.

**Spread is published.** Each sample carries `spread_pct`, the max-minus-min
as a fraction of the median. A number with forty percent spread is reported as
a number with forty percent spread, not as its most flattering iteration.

**A warmup pass runs untimed.** First-iteration page faults and cold caches
are real costs of the first call, not of the operation.

**Regressions gate, cautiously.** `compare-bench` fails CI when a median
worsens past a tolerance, compares medians only, and refuses outright to
compare records from different host shapes, because a four-core runner
against an eight-core laptop produces confident nonsense. A gate that fires
on noise gets muted, and a muted gate is worse than none.

## Rules for security figures

**Derived, not asserted.** `soundness` computes every figure from the
constants the verifier is compiled with. Change `attest_params` and the
published soundness moves with it; the run fails if it falls below the gate,
so a parameter downgrade cannot pass silently.

**Both models, always.** The provable unique-decoding floor and the
conjectured list-decoding figure are published side by side, with the honest
reading in the artifact's own text: the truth lies between them and no
independent analysis of this crate pins it. Reporting only the flattering
model is advocacy, not measurement.

**The parameters are auditable from outside.** `regenerate-constants`
re-derives the in-circuit permutation's entire parameter set from the
published rule, independently of the implementation, and CI fails if the
compiled crate disagrees with the rule.

## The audit every published number must survive

Before a number leaves this repository in prose, it is checked against the
floor of its own fastest component. A composite operation cannot be faster
than the cheapest thing it provably does; if it appears to be, the benchmark
is broken, not the record.

**The worked example, kept here deliberately.** The first enrolment sweep
reported the hybrid measurement of a 64 MiB set at 31.7 ms, and 16 MiB at
33.1 ms. Two guards fired on inspection. The scaling guard: four times the
bytes cannot cost the same time when the dominant cost scales with bytes. The
floor guard: an independent probe timed the BLAKE3 pass alone over the same
64 MiB at longer than the whole enrolment claimed, and an operation cannot
undercut its own first step. The cause was the alive-results rule being
violated: the timed closures dropped their results, and the optimizer elided
part of the work. The fix is commit `e519125`; the corrected figures scale
exactly as the arithmetic says they must, floor plus a fixed tree cost.

The uncomfortable part is the lesson: the wrong number was large, flattering,
and about to be published. It was caught not by a reviewer but by the habit
of distrusting results that look better than their own floor. That habit is
the methodology.

## Reproducing everything

```
cargo run --release --bin emit-bench -- bench/results.json
cargo run --release --bin soundness  -- bench/soundness.json
cargo run --release --bin hash-study -- bench/hash-study.json
cargo run --release --bin regenerate-constants
cargo test --workspace --release
```

The Lean development checks with `lake build` in `lean/`, and CI additionally
prints the axiom dependencies of every advertised theorem and fails on
anything beyond the two core axioms, or on any `sorry`.
