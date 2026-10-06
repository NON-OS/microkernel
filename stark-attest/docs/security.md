# Security: the claims, their sources, and their limits

This page states what the crate's security rests on, mechanism by mechanism,
with the status each claim honestly deserves. The categories: INHERITED means
the property follows from a standard construction instantiated faithfully;
TESTED means an in-tree test enforces it; DESIGNED means the code embodies
the property but no artifact checks it; NOT PROVEN means exactly that.

## The trust base

No trusted setup, no ceremony, no secret material anywhere in the system.
Every proof is publicly re-verifiable from the root and the trailer alone.
The assumptions are: collision resistance and random-oracle-style behaviour
of Keccak-256 (transcript, commitments), the security of the Poseidon
permutation at the chosen parameters (the in-circuit hash), and the FRI
proximity gap (the low-degree test). All three are hash-type assumptions;
none is broken by a quantum computer's algebraic attacks, which is why the
post-quantum claim is by construction rather than by migration.

## Soundness accounting

- Folding challenges from `Fp2` put the per-fold soundness error near
  `2^-128` (INHERITED from FRI over a 128-bit challenge space; the base-field
  variant at `2^-64` exists only as the recursion's arithmetization target,
  see [fri.md](fri.md)).
- `N_QUERIES = 32` with `EXTRA_BLOWUP_BITS = 3` compounds the query-side
  soundness (INHERITED; the parameters are explicit at every verifier call).
- `GRIND_BITS = 16` prices challenge grinding (DESIGNED; its value is a cost
  multiplier, not a proof).
- Parameters cannot be weakened by a proof: they are supplied by the
  verifier from one module (TESTED at the type level by the single import
  path; see [params.md](params.md)).

There is no machine-checked soundness proof of the composed system in this
repository. NOT PROVEN, stated here and in the README, until it exists.

## The Poseidon caveat

The in-circuit hash runs 32 full rounds where the published schedule for the
shape is 8 full plus 22 partial, so the round margin dominates (INHERITED).
But the width and round constants are this crate's own generation, not a
published, externally cryptanalyzed instance. The margin defends the round
count; it does not import anyone else's cryptanalysis of these exact
constants. NOT PROVEN beyond that, and a review by an external cryptanalyst
of the constant generation is the single highest-value outside contribution
this crate could receive.

## Binding

A trailer binds membership, measurement and context in one verification
(see [membership.md](membership.md)): swapping the artifact changes the
measurement, tampering the metadata changes the context, replaying across
epochs changes the context, and all three fail the same gate (TESTED: the
CLI selftest enrolls, verifies, then confirms that a tampered artifact and a
tampered context are both refused; consumer corpora carry forgery cases).

## Implementation-level defenses

- The trailer parse is total: strict magic, bounds before slices, canonical
  field-element checks, no panics on arbitrary bytes, `#[must_use]` on the
  verdict (DESIGNED; fuzzing it is a worthwhile addition and is not currently
  in-tree).
- The fast field reduction is pinned to the reference `u128` modulo by an
  in-file property test (TESTED); the generic tower is pinned
  element-for-element to the concrete extension (TESTED). Both are
  duplicate-truth gates: the dangerous failure was never a wrong constant, it
  was two right implementations drifting apart.
- Serial and parallel proving are held bit-exact by a digest comparison in
  the consumers' corpora (TESTED there).

## What this crate does not defend

It proves what was enrolled, not that what was enrolled is good: a malicious
artifact enrolled by its publisher verifies perfectly. Publisher authenticity
is signature-layer work that lives with the consumer, alongside this crate's
membership proofs, not inside them. Nor does it defend the machine doing the
verifying: a verifier whose binary is tampered can lie, which is why the OS
consumer anchors the root inside the artifact that enforces it and gates its
own build on re-verification. Composition is the consumer's responsibility;
this crate's job is that the mathematical statement it makes is exactly the
one it verifies.
