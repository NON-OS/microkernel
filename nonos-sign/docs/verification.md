# Verification

One chain, run identically in three places: this crate's `verify`
module on the host, `nonos-verify` in CI, and the kernel's spawn gate
mirror. A capsule that passes on one passes on all three or the
mirrors have drifted, which is itself a bug.

## The chain

Verifying a capsule is four checks in order, each with everything it
needs from public material:

1. **Policy decodes and self-verifies.** The trust-anchor policy's own
   signature trailer must verify under the keys the policy carries,
   for every required algorithm, inside each key's validity window.

2. **Certificate under policy.** `verify_cert` checks the certificate's
   trust-anchor signatures against the policy keys, the epoch binding,
   the validity window against the caller's clock, and the revocation
   lists: serial, NONOS-ID, and publisher key id all get a veto.

3. **Manifest under certificate.** `verify_manifest` recomputes BLAKE3
   of the exact certificate bytes and requires it to equal the
   manifest's `nonos_id_cert_id`, so a manifest cannot float between
   certificates. The namespace must match one of the certificate's
   globs, `required_caps | optional_caps` must fit under the
   certificate's ceiling, and the publisher signatures must verify for
   every required algorithm using keys named by id in the certificate.

4. **Payload binding.** With `--elf`, BLAKE3 of the given file must
   equal the manifest's `payload_hash`. This is the check that turns
   the chain from "this manifest is authentic" into "this manifest is
   about these exact bytes", and it is what production pipelines use
   in place of signing: a rebuilt capsule either measures to the
   enrolled hash or fails by name with both hashes printed.

## Refusal semantics

Every failure is a distinct `SignError` naming the first check that
failed; nothing falls through to a generic error. Decoders refuse
trailing bytes, oversized counts, and lengths that disagree with the
buffer before any cryptography runs, so malformed input is rejected on
shape, cheaply, and signature verification only ever sees well-formed
regions. There are no warnings: a chain verifies or it does not.

## What is deliberately absent

No chain-of-trust walking beyond one certificate: a manifest binds to
exactly one cert, a cert to exactly one policy epoch. No online state:
revocation is carried in the policy, so verification is possible
offline and in the bootloader. No downgrade path: required algorithms
are fixed by the verifier, not negotiated from the artifact.
