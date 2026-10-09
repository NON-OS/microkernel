# FRI, four ways

FRI is the low-degree test at the heart of every STARK here: a transparent,
post-quantum argument that a committed codeword is close to a low-degree
polynomial, built from nothing but hashing and folding. The crate carries four
variants, and they are not redundancy; each exists for a different consumer,
and the split is the module structure making a security decision legible.

## The protocol, once

The prover evaluates a polynomial over a domain `2^blowup` times larger than
its degree bound and commits to the codeword in a Merkle tree. Then, layer by
layer, a challenge `beta` folds even and odd halves into a codeword of half
the degree, each layer committed, until a constant remains. The verifier
re-derives every `beta` from the transcript, spot-checks `N_QUERIES` random
positions per layer (two openings and one fold equation each), and confirms
the final constant. A prover who committed to something far from low-degree
gets caught at each query with probability bounded by the proximity gap;
queries compound independently.

`src/fri/domain.rs` builds the evaluation domains from the field's `2^32`
two-adic subgroup; `fold.rs` is the fold; `prove.rs`/`verify.rs` the
protocol; `types.rs` the proof shape.

## The four variants

| Module | Challenges | Commitments | Consumer |
|---|---|---|---|
| `fri` | `Fp` | Keccak | the recursive verifier's arithmetization target |
| `fri_ext` | `Fp2` + grind | Keccak | the money-grade path: every real attestation proof |
| `fri_poseidon` | `Fp` | Poseidon | FRI that an AIR can verify: recursion, base strength |
| `fri_poseidon_ext` | `Fp2` + grind | Poseidon | recursion at money-grade strength |

Two axes, four points. The **challenge axis** is soundness: base-field
challenges give a folding soundness error around `2^-64`, which an adversary
with `2^64` grinding attempts can attack; drawing `beta` from `Fp2` takes the
folding error to around `2^-128`, and the transcript grind
(`GRIND_BITS = 16`) raises the cost of challenge-grinding on top. The source
states this split plainly: `fri_ext` is the FRI a high-value proof uses, and
the base module remains because a recursive verifier must arithmetize
something, and arithmetizing the simpler protocol first is how the monolith
was built.

The **commitment axis** is arithmetizability, not soundness: Keccak
commitments are cheap everywhere except inside a circuit; Poseidon
commitments make the whole verification expressible as an AIR. See
[hashing.md](hashing.md).

## What the verifier enforces

`stark_verify_ext_blown_bound` (the entry the attestation gate calls) takes
the query count, grind bits and extra blowup explicitly from
[the parameter authority](params.md), never from the proof. A proof cannot
carry its own security level; a trailer that claims fewer queries than the
verifier demands simply fails. The same holds for the domain: the verifier
recomputes the expected domain from the AIR's shape and
`EXTRA_BLOWUP_BITS = 3`, so an undersized commitment is rejected structurally.

## Cost shape

Proving is dominated by committing the blown-up codeword and its layers:
`O(domain · log domain)` hashing and NTT work. The extension prover streams
per coset, holding coefficients plus a single coset resident rather than the
whole evaluation domain, which is what makes very large traces provable on
ordinary machines; behind the `parallel` feature the work fans out across
cores, with serial and parallel provers held bit-exact by a digest gate in
the consumers' test suites. Denominator inversions in the DEEP pass are
batched with one Montgomery walk per block instead of one inversion per
term.
Verification is `N_QUERIES · layers` hash paths plus one fold equation each:
milliseconds, allocation-light, `no_std`. That asymmetry is the product: pay
once at proving, verify anywhere forever.
