# Security policy

## Reporting

Report vulnerabilities to **ekisanon@proton.me**. You will get an
acknowledgement within 72 hours and a substantive answer within two weeks.
If the finding is confirmed, we coordinate a fix and a disclosure date with
you; if we dispute it, we will say exactly why, in writing, and you are free
to publish our reasoning alongside yours. Please do not open a public issue
for anything you believe is exploitable before contact.

There is no bounty program. Credit is given in the changelog and the advisory
unless you ask otherwise.

## Scope

In scope, in decreasing order of severity:

1. **Soundness**: any input accepted by `verify_attestation_trailer`,
   `stark_verify_ext_blown_bound`, or the aggregate verifier that was not
   produced by honest proving over an enrolled member. A forged membership,
   a context or epoch that verifies for bytes it does not bind, a padding
   slot reachable by a real artifact.
2. **Parameter integrity**: any path by which prover-supplied data influences
   the verifier's effective soundness parameters, or by which the compiled
   parameters can disagree with `attest_params` without a build failure.
3. **Verifier robustness**: any byte string that panics, loops, or reads out
   of bounds in the verifier or any deserializer. The parse is proven total
   in the Lean model and pinned to the wire by a round-trip test; a
   counterexample to either is a serious finding.
4. **Measurement collisions**: two admissible artifacts measuring to the same
   leaf under either measurement scheme, or one artifact measuring to the
   same leaf under both schemes despite the domain separation.
5. **The wasm boundary**: memory unsafety in `verifier-wasm`'s exports, or a
   divergence between the wasm gate's verdict and the native gate's verdict
   on any input.

Out of scope: the throughput of the prover, denial of service by handing the
prover a huge honest workload, vulnerabilities in dependencies with no
demonstrated impact here, and anything that requires the verifier to be run
with parameters other than the compiled ones.

## What you can assume when analysing

The threat model and the honest limits of every claim are documented where
the claims are made: `docs/security.md` for the construction,
`docs/membership.md` for what one verification proves,
`docs/poseidon-spec.md` for the hash instance and what has not been
cryptanalysed, `bench/soundness.json` for the soundness figures under both
the provable and the conjectured model. If you find a claim anywhere in this
repository that the code does not back, that is a valid report too.
