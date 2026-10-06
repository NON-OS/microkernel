# Proving

A spend, proved on the device, and what the prover trusts and returns. What is checked and what is not is in [20-security-status.md](../../../docs/20-security-status.md).

## What this owns

A spend, proved on this device. It turns the notes of this wallet and the history of the pool into
the request the launch prover proves against (`launch/request.rs`), the secrets it proves with
(`launch/seed.rs`), and its proof into what the pool and a lander read: 37 public limbs expanded
into the 13 words of an intent, the last the not-before time (`launch/publics.rs`).

`launch/prove.rs` makes one spend in this order:

1. The request is checked by the prover against the trees it rebuilds. A request whose leaves do
   not reach the root it names is refused before any proving time is spent.
2. The proof is made from 512 bytes of fresh entropy (`launch/entropy.rs`), and verified before it
   is returned.
3. The rank condition behind zero knowledge is checked on the proof itself. A proof that fails it
   is thrown away and made again with fresh entropy, up to 3 attempts, and so is a proof the prover
   itself refuses with `Error::Rank`.

What comes back is a `LaunchProof`: the proof bytes (94,760 to 95,752 for the four pinned vectors,
in format 7), the 37 limbs, the 13 words, the two notes created with their blindings, the grinding
work, the anonymity defaults it gave up if any, and the periodic cache if this proof built it.

The seed file the prover reads holds the spend secret and the blindings. It is written in place
into one buffer sized for the longest file, so it never grows and leaves no copy behind, and it is
wiped when dropped (`launch/seed.rs`).

It owns one refusal that matters more than the rest: a draw of entropy is used once. A second use
in the same process is refused, because the same bytes would make the same blinding.

## What it trusts

The production prover, vendored whole in `vendor/` from the STARK repository at `e8f02a0`, whose
tarball SHA-256 is recorded in `vendor/README.md`, built with its `not_before` feature (radix-8
FRI, 32-byte digests, the 37-limb statement): `nox_prover` for the proof and its policy,
`stark_proofs` for the circuit and the rank check, `nonos-stark` for the field and FRI. This module
re-derives none of the soundness argument. The four pinned 37-limb vectors prove byte for byte with
this build (`core/tests/prod_vectors.rs`), and the production verifier on Sepolia accepts their
single-call layout, as [20-security-status.md](../../../docs/20-security-status.md) records.

Where the time goes is measured by `core/tests/profile_prod.rs`, phase by phase: on 4 virtual
cores, 67.7 s from the periodic cache, of which FRI is 56%, the region commit 17% and the products
9%, and 82.7 s from nothing.

The platform CSPRNG, through `entropy`, for every blinding and every note secret. The hiding is
as good as that draw and no better.

## What it hands its neighbours

To `wallet`: the `LaunchProof`, from which the spend flow seals the two outputs and writes the four
hand-off files a lander reads: the proof, its public limbs, and the two sealed notes.

To `bench`: the same proving path, timed, with its proof written out so it can be checked against
the live verifier. And the phase profile: the pinned `transfer-eth` vector of the STARK repository,
whose request, seed and entropy `core/src/bench/profile/vector` carries byte for byte, proved from
the periodic cache with a mark at every phase the prover reports.

## What an attacker who controls a neighbour can do

**A caller that reuses entropy.** Refused, process wide, by digest (`entropy_was_used`). Two blinded
proofs of one statement under one blinding reveal the witness, which is the worst outcome in this
module.

**A caller that hands a lander an unverified proof.** It cannot: the prover verifies before it
returns, and a proof that fails the rank check is never returned.

**A lander that keeps every proof it is sent.** It holds proofs that verify against public words.
That is the design: the words are public. It does not hold the witness, because every proof carries
a fresh blinding.

**An RPC that serves a stale root.** The wallet anchors a spend to the newest root the pool
published over the leaves it read (`wallet/anchor.rs`). A wrong root makes a proof that settles
nowhere, which is a denial of service and reveals nothing.

**An interrupted device.** Cancellation is a flag the prover checks between stages. Nothing is
written and nothing is sent, so a cancelled proof costs its entropy, which is spent and cannot be
reused, and some battery.
