# The transcript

`src/transcript.rs` is the Fiat-Shamir heartbeat: it turns an interactive
protocol into a non-interactive one by deriving every verifier challenge from
a running hash of everything the prover has committed so far. Get this wrong
and soundness dies quietly, so the design is minimal and inspectable.

## Construction

The state is a single 32-byte Keccak-256 digest. Initialization hashes a
protocol label, so transcripts of different protocols can never be confused
for one another. Every absorption is

```
state = keccak256(tag || state || data)
```

with a distinct one-byte domain tag per message kind (commitment roots,
public inputs, out-of-domain values, nonces). The tag is what prevents a
cross-kind splice: bytes absorbed as a root can never be replayed as a
challenge seed for a different phase, because the tag byte differs and the
digest diverges.

Challenges are squeezed from the state and mapped into `Fp` or `Fp2` by
rejection into the canonical range, so challenge distribution is uniform over
the field rather than biased by the modulus.

## Ordering is the security

Fiat-Shamir is only sound if every challenge is derived after the material it
must bind. The prover absorbs the trace commitment before drawing the
composition challenge, absorbs the composition commitment before the DEEP
point, absorbs each FRI layer root before that layer's fold challenge, and
absorbs everything before the query indices. The verifier re-runs the same
sequence and gets the same challenges only if it saw the same commitments;
any reordering or omission yields different challenges and a failed proof.
The order is fixed in code on both sides of the protocol, not negotiated.

## Grinding

Before query indices are drawn, the prover searches a nonce such that the
state hash of the nonce clears `GRIND_BITS = 16` leading zero bits, and the
nonce is absorbed. This is a proof-of-work on the transcript: an adversary
attempting to grind toward favourable queries must pay `2^16` hashes per
attempt, multiplying the cost of any challenge-space attack, while the honest
prover pays a fraction of a second once and the verifier checks it with a
single hash.

Behind the `parallel` feature the nonce search fans out across cores. The
found nonce is the same class of object either way and the transcript that
absorbs it is identical, so serial and parallel proving remain bit-exact;
the consumer's digest gate holds that property under test.

## What the transcript does not do

It does not authenticate the parameters. Query count, grind bits and blowup
come from [the parameter authority](params.md) on the verifier's side, never
from transcript material, so a prover cannot talk the verifier down to a
weaker check. And it does not hide anything: transcripts are deterministic
functions of public material, which is exactly what makes the proofs
publicly re-verifiable.
