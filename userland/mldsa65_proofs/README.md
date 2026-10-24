# mldsa65_proofs

Host proofs for `nonos-mldsa65`. There are 6 tests.

- **The published known answer.** The reference project's metadata gives
  the SHA-256 of the first ML-DSA-65 known-answer record (`nistkat-sha256`
  in `META.yml`). The test rebuilds that record through the crate alone,
  using the NIST generator's seed and its AES-256 CTR DRBG (`src/drbg.rs`,
  over `nonos_aes`). The rebuilt record hashes to the published value.
- **Agreement with nonos-sign.** `nonos-sign` is the host build of the
  source the kernel and the loader compile. It wrote
  `vectors/nonos_sign.txt`, which holds:
  - a key pair made with zero randomness;
  - a random key pair;
  - signatures of four messages, from empty to 65537 bytes, under each.

  The crate reproduces every byte and verifies every signature.
- **Refusals.** A failed draw fails the key pair and the signature. A
  changed byte in the message, the signature or the key fails
  verification. The same randomness gives the same key.
- **Threads.** Eight threads at once take turns and all succeed.

## Mutants

| Mutant | Result |
|---|---|
| A failed draw not noted | caught |
| A failed draw not reported | caught |
| Freed slots never come back | caught: the C exits |
| Slots too small for the SHAKE state | caught: the C exits |
| `verify` accepting anything | caught |
| Returning a key pair the C failed | survives, equivalent: the C never fails a key pair |
| Returning a signature the C failed | survives, equivalent: the C never fails a signature |

What the kernel's own compiled copy does is shown by the shared source
and the published known answer, not by running the kernel here.

Run with `cargo test` from this directory.
