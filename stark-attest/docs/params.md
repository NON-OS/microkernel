# The parameter authority

`src/attest_params.rs` is the shortest file in the crate and the one a reviewer
should read first, because every attestation prover and verifier in every
consumer reads its four constants, and the whole security story hangs on them
agreeing everywhere.

```rust
pub const LOG_ROUNDS: u32 = 5;        // 32 Poseidon rounds in the AIR hash
pub const N_QUERIES: usize = 32;      // independent FRI queries
pub const GRIND_BITS: u32 = 16;       // transcript proof-of-work
pub const EXTRA_BLOWUP_BITS: u32 = 3; // blowup beyond the AIR's own
```

## Why one file matters more than four numbers

These values used to live in five separate copies across the consumers of the
library: the process-spawn gate, the image self-attestation check, the
pre-boot verifier, the enrollment tool, and a test harness. Prover and
verifier must agree exactly or nothing verifies, so an upward drift in any
copy is loud: proofs stop verifying and someone investigates. A downward
drift is the dangerous direction, and it is silent: a verifier that checks 16
queries against a prover that ground 32 accepts everything it should, at half
the soundness anyone reading the other copies believes the system has. One
module, imported everywhere, turns that entire failure class into a compile
error. The module's own doc comment says exactly this, because the reason is
more important than the values.

Changing any value re-defines what every existing trailer verifies against.
The operational consequence is stated where the values live: after a change,
every artifact must be re-enrolled.

## What each parameter buys

**`LOG_ROUNDS = 5`.** The AIR's in-circuit hash is a width-8 Poseidon (distinct
from the width-12 one the Merkle tree and FRI transcript use outside the
circuit), run for `2^5 = 32` rounds, all of them full rounds. The published
schedule for a Poseidon of this field and S-box shape is 8 full plus 22
partial rounds; a partial round applies the S-box to one lane instead of all
eight, so 32 full rounds strictly dominate the 30 mixed ones. The width and
round constants still differ from any published instantiation, so this margin
defends the round count, not the provenance of the constants; the security.md
page carries that caveat in full. Cost scales as
`2^(LOG_ROUNDS + log_slots)` trace rows, which is why the round count is a
log, not a free integer.

**`N_QUERIES = 32`.** Each FRI query independently samples the committed
codeword; a prover who committed to something far from low-degree survives a
query only with probability bounded by the proximity gap. Thirty-two
independent chances compound to the headline soundness, in concert with the
blowup.

**`GRIND_BITS = 16`.** The prover must find a nonce making the transcript hash
carry 16 leading zero bits before challenges are drawn. It adds 16 bits of
work to any grinding attack on challenge selection for the cost of a fraction
of a second at proving time, and is free to verify.

**`EXTRA_BLOWUP_BITS = 3`.** The evaluation domain is 8 times larger than the
AIR's own minimum blowup, improving the proximity of the low-degree test per
query. Larger blowup means larger proofs and slower proving; three extra bits
is the chosen point on that curve.

## The verifier's contract

`stark_verify_ext_blown_bound` takes `N_QUERIES`, `GRIND_BITS` and
`EXTRA_BLOWUP_BITS` explicitly, and the AIR takes `LOG_ROUNDS` in two places
(the hasher and the schedule). Every call site in this crate and its consumers
passes the constants from this module, never a literal. A consumer that needs
different soundness for a different use case should define its own authority
module with the same discipline, not inline numbers; the failure mode being
prevented is not wrong values, it is two right values in two places growing
apart.
