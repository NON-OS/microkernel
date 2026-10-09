# Membership attestation: the trailer

The product of this crate, as consumers experience it, is the trailer: a
self-contained byte string an artifact carries, proving under a 32-byte root
that this exact artifact, with this exact context, is a member of the
enrolled set. This page is the wire format, the statement, and the verifier's
parse, from `src/air/attest_trailer.rs`, `attest_build.rs`, `attest_verify.rs`
and `multi_membership.rs`.

## The statement

The enrolled set is committed as a Merkle tree of the in-circuit Poseidon
(width 8, see [hashing.md](hashing.md)) over the members' measurements, padded
to the fixed width `2^depth` with a reserved slot value that begins with a
byte no real artifact starts with, so no artifact can measure into a padding
leaf. The root is the 32-byte statement.

## How a member is measured, and the choice that costs a thousandfold

A leaf is the Poseidon digest of a member. There are two ways to reach it, and
the crate offers both because they bind different things:

**Direct** (`measure_capsule`, `MeasuredSet::commit`) absorbs the artifact's
bytes into the sponge, seven per lane with a permutation every rate group, so
one permutation per 28 bytes. The proof binds the bytes themselves. Measured on
this crate's own study, that runs at about three megabytes a second, and the
rate is flat in artifact size: a hundred megabyte release spends more than half
a minute in the sponge before any proving starts.

**Hybrid** (`measure_capsule_hybrid`, `MeasuredSet::commit_hybrid`) hashes the
artifact with BLAKE3 and absorbs only the resulting 32 bytes, one permutation
regardless of size. It reaches BLAKE3's own throughput, gigabytes a second,
three orders of magnitude past the direct path, and the gap grows with
artifact size. The current measured figures, with raw samples and the host
that produced them, are in `bench/results.json` and `bench/hash-study.json`
rather than hardcoded here, so this page cannot quietly outlive them.

The security difference, stated plainly: the hybrid binds a BLAKE3 digest where
the direct path binds the bytes, so a BLAKE3 collision would let two artifacts
share a leaf. Whether that is a new assumption depends on the caller. For an
attestation whose proof context already carries the artifact's BLAKE3
measurement, as this crate's does, a BLAKE3 collision already breaks binding
regardless of how the leaf was built, so the hybrid removes redundant work
rather than adding a hypothesis. A caller whose context commits to no
general-purpose digest is making a real trade and should keep the direct path.

The two are domain separated: the hybrid absorbs a fixed domain word before the
digest, so no artifact measures to the same leaf under both schemes, the roots
differ, and a trailer from one never verifies against the other. That is
deliberate. Changing how a set is measured has to invalidate the set rather
than silently reinterpret it.

A trailer for member `i` proves: *I know the authentication path from leaf `i`
to this root, and the proof is bound to this context.* The context is
`BLAKE3(artifact) || caller bytes` (in the OS deployment the caller bytes are
the granted capability mask and the policy epoch). Binding happens in the
STARK transcript, so the same path proven for a different context is a
different proof and fails verification. One verification therefore checks
three things at once: membership under the root, the artifact's exact bytes
(through the measurement in the context), and the issuer's chosen metadata.

## Wire format

```
offset  size                 field
0       8                    magic "NZKSTRK1"
8       1                    depth (tree depth; 0 is invalid)
9       depth * RATE * 8     sibling digests, RATE=4 field elements each,
                             u64 little-endian, each < p (canonical)
+       ceil(depth/8)        direction bits, LSB-first per level
+       rest                 the serialized STARK proof
```

This table once documented the two variable regions in the swapped order,
copied from a canonical verifier that disagreed with the builder beside it and
with every deployed consumer. The wire format above is the one the builder
emits, the kernel's spawn gate parses, and the round-trip test in the crate
now pins; the browser verifier runs this same entry point, which is how the
disagreement was finally caught.

The parse (`verify_attestation_trailer`) is strict and total: magic checked,
depth bounds checked, every sibling element rejected unless canonical
(`v < p`), lengths validated before any slice, and the function is
`#[must_use]` so a consumer cannot accidentally drop the verdict. Malformed
input returns false; nothing panics on any byte string.

## What the trailer cannot claim

The soundness parameters are not in the trailer. The verifier supplies the
round count, query count, grind and blowup from
[the parameter authority](params.md); the source states the rule as the
design intent: the counts are the verifier's, never the trailer's, so a
prover cannot weaken the low-degree test by handing over a proof that claims
an easier one. A trailer proven under weaker parameters simply fails against
a verifier demanding the real ones.

## Enrollment

`attest_build.rs` builds trailers; the `MeasuredSet` type commits the padded
set once and issues every member's trailer from that single commitment, so
enrolling `n` members costs one tree plus `n` proofs rather than `n` trees.
Every emitted trailer is immediately re-verified by the same gate the
consumers run, so a trailer that would fail in the field fails at enrollment
instead.

Trailers embed grinding nonces, so re-enrollment of an identical set produces
byte-different trailers proving identical statements. Nothing may compare
trailers by hash across enrollments; identity lives in the root and the
statement, not the proof bytes.

## Set semantics

One root commits one set. Any member change moves the root and invalidates
every trailer, and there is no sound shortcut that keeps sibling proofs alive
under a new root, because their authentication paths run through changed
nodes. That is the semantics of a set commitment; the operational answer is
to re-enroll on release, not to iterate under proof (see the consumer's build
documentation for the two-loop workflow this implies).
