# The hashes, and why there are three

The crate uses three hash constructions, each where its properties matter, and
confusing them is the fastest way to misread the system.

## Keccak-256: the transcript and the tree

`src/hash/keccak.rs` implements Keccak-256, used in two places outside the
circuit: the Fiat-Shamir transcript (`src/transcript.rs`) and the Merkle
commitment over codewords (`src/merkle/hash.rs`, domain-separated leaf and
node hashing).

The choice is deliberate and stated in the source: Keccak is the EVM-native
hash. A proof whose commitments and transcript are Keccak can be re-verified
on-chain with a single native opcode per hash, where an in-Solidity algebraic
hash would cost orders of magnitude more gas. Nothing about FRI requires an
algebraic hash on this path, so the cheap-everywhere one wins.

## Width-8 Poseidon: the in-circuit hash

`src/air/poseidon.rs` is the hash the attestation AIR proves knowledge about,
and it has real parameters, not demonstration ones:

- width 8 over Goldilocks, rate 4 and capacity 4: a 256-bit capacity, so the
  sponge targets 128-bit security;
- the `x^7` S-box, a permutation because 7 is coprime to `p - 1`;
- a maximum-distance-separable diffusion matrix built as a Cauchy matrix;
- `2^LOG_ROUNDS = 32` rounds, all full. The published schedule for this shape
  is 8 full plus 22 partial; since a partial round S-boxes one lane instead of
  eight, 32 full rounds strictly dominate. The caveat, carried in
  [params.md](params.md) and [security.md](security.md): the width and round
  constants are this crate's own, so the margin defends the round count, not
  the pedigree of the constants.

This is the hash inside the membership tree that the STARK opens, which is why
it must be cheap *as constraints*, not as CPU: an algebraic permutation costs
a few rows per round in the trace where Keccak would cost thousands.

## Poseidon commitments: the recursion path

`src/fri_poseidon/` and `src/fri_poseidon_ext/` run the same FRI protocol but
commit with Poseidon instead of Keccak, and draw challenges algebraically
(`src/air/draw_ood_poseidon.rs`). The point is stated in the module header:
when the commitments and the transcript are algebraic, a FRI verification can
itself be expressed as an AIR, which is the prerequisite for a recursive
STARK. Same protocol, different hash, different consumer.

## The rule of thumb

| Hash | Where | Why |
|---|---|---|
| Keccak-256 | transcript, codeword Merkle tree | native on EVM, cheap everywhere outside a circuit |
| Poseidon w8 | the membership tree the AIR opens | cheap as constraints |
| Poseidon commitments | `fri_poseidon*` | makes FRI verification arithmetizable, enabling recursion |

BLAKE3 also appears in the system, but not in this crate's cryptography: it is
the artifact measurement (the leaf *content* is a BLAKE3 digest of the
artifact, computed by the enrollment tool and bound into the proof context).
The proof machinery never depends on BLAKE3's algebraic structure; it treats
the measurement as opaque bytes.
