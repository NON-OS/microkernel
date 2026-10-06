# Recursion and aggregation

Beyond single-membership trailers, the crate carries the machinery to verify
FRI proofs *inside* a STARK and to aggregate many statements into one. The
deep map of the monolith lives in the crate itself at `src/air/README.md`;
this page is the orientation layer above it and the notes that page does not
cover.

## The idea

A FRI verification is re-deriving fold challenges from a transcript, opening
committed values at query positions, folding them, and checking the final
constant. Each of those steps is arithmetic and hashing, so the whole
verification can itself be written as an AIR and proven, yielding a proof
that a proof verifies. Done naively per query and per layer, the verifier
cost is `O(queries x layers)` separate circuits; the monolith fuses the whole
verification into a single trace whose proof size does not grow with either.

The prerequisite is algebraic commitments, which is why the
`fri_poseidon*` variants exist (see [fri.md](fri.md) and
[hashing.md](hashing.md)).

## The four pieces

As `src/air/README.md` lays out with the trace diagrams:

- **`Fused`** stacks heterogeneous AIR regions into one trace with a per-row
  selector, verified as one STARK.
- **`Wired`** adds a Plonk-style grand-product copy constraint over chosen
  cells, so values in different regions are forced equal by public wiring
  rather than by trust.
- **`TraceFold`** is a FRI fold whose challenge is witnessed in the trace so
  the wiring can bind it to the transcript region that derived it.
- **`TupleLookup`** range-checks query indices via a lookup argument.

Openings, folds, transcript checks and index checks become regions of one
trace, wired together, proven once.

## DEEP composition across padding

Fusing regions of different sizes pads the composition; the DEEP accumulator
must be carried through every padding row, not only the last one. With a
single padding row the two behaviours coincide, which is exactly the kind of
coincidence that hides a completeness bug until a wider assembly hits it: the
accumulator logic in `deep_check_ext.rs` carries through all padding rows,
and any change there must keep the consumer digest gates green for existing
assemblies while unlocking wider ones.

## Aggregation

`aggregate.rs`, `aggregate_build.rs` and `aggregate_verify.rs` batch multiple
membership statements into one proof, and `shared_root.rs` specializes the
common case where all members open against the same root, deduplicating what
the naive batch would prove repeatedly. `multi_membership.rs` exposes the
opened-cell coordinates that the wiring binds. The aggregate path is how a
consumer turns "verify n trailers" into "verify one proof about n trailers"
when n is large and the verifier is expensive to invoke, at the cost of
proving the aggregate.

## Honest status

The monolith and the aggregates are implemented and exercised by the
consumers' proof corpora, including forgery cases. The recursion path is the
newest surface in the crate: treat width changes and new assemblies as
security-relevant modifications that must pass the full corpus and the
serial-vs-parallel digest gate, not as refactors.
