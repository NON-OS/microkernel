# nonos-device-attest

The anonymous device statement. A device proves to a verifier, who learns
nothing else, that an approved bootloader started it, that an approved kernel
runs on it, that it is an enrolled device, and that `t` is its tag for the
verifier's scope. Which bootloader, which kernel and which device stay
private. One device has one tag per scope, so a verifier can rate-limit
without identifying. The full account, with what the proof does not say, is
[the device proof page](../docs/handbook/trust/device-proof.md).

## The statement

Four public trees and values: B, the bootloader tree; P, the kernel tree; R,
the device registry; the scope `e` and the context `c`. The witness is a
bootloader slot of kind `Bootloader` in B, a kernel slot of kind `Kernel` in
P, the device secret `s` with `commit(s)` a leaf of R, and the tag
`t = tag(s, e)`, with the proof bound to the context `c`. The circuit ties the secret in the registry leaf to the
secret in the tag lane for lane, so the tag belongs to the enrolled secret and
no other.

The proof shows that an approved chain exists and that this device holds an
enrolled secret. That the chain is the one running comes from the TPM: the
kernel derives `s` only under a PCR 9 value the release signed for and under
PCRs 0, 4 and 7 as the machine found them (see
[the TPM page](../docs/handbook/trust/tpm.md)).

## What is in it

- `prove` runs `check` first, which folds each opening natively and refuses a
 false statement before the prover sees it. It then blinds the trace,
 proves, verifies its own proof, and withholds any proof that fails the
 zero-knowledge rank certificate; the caller proves again with fresh entropy.
- `verify` takes the statement and the bytes, rebuilds the circuit's shape
 from the statement, and refuses a proof at any other parameter point.
- `Registry` is R as a registrar keeps it, keyed by the BLAKE3 of the TPM
 endorsement key's public area. `enroll` refuses a revoked key and a
 commitment held under another key; `transcript` and `recompute` let anyone
 check that a published root is the one its entries give.

The prover and its field come from STARKs main (`nonos-stark` and
`stark_proofs`, feature `fri8`), at the commit the flake's `starks` input
locks. The default features `std` and `parallel` are for host tools; the
`nonos.prove` capsule builds without them and proves on one core.

## Checks

31 host tests: known answers, every checkpoint cell changed alone and
refused, forged kinds and forged tag secrets refused, and a tie that holds the
built circuit to the STARK lane's Lean model of its layout.
`userland/prove_proofs` runs the capsule's assembly half against this crate
with 30 more.

## What is not done

- Never run on a booted image: lists the device anonymity
 proof as built, never booted.
- The checkpoint tests still name a `launch_v1` feature the crate no longer
 declares, so the test that showed the old checkpoint rule refused cannot be
 built.
- Soundness rests on the STARK parameters and FRI; no Lean proof covers FRI.
