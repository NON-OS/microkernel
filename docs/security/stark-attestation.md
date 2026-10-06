# STARK attestation

NONOS admits a kernel, a loader or a [capsule](../overview/glossary.md#capsule) only when its [attestation trailer](../overview/glossary.md#attestation-trailer) shows that its measurement, and for a capsule the capabilities it is about to get, fill a slot of a tree whose root the checker already holds.

## The statement

Every gate checks one statement: under root R, the leaf built from context C with kind K is a slot of a 256-slot tree. It checks it twice, by folding a Merkle path with Poseidon and by verifying a STARK proof of the same slot, and admits only when both pass. The capsule gate's module comment names both halves, and says that the trusted root is always the kernel's own, never the trailer's (`verify_against`, `src/security/capsule_attest/path.rs:17-35`).

The gate builds the context itself from what it is about to run and grant, and never takes it from a trailer (`capsule_context`, `nonos-attest-path/src/context.rs:17-23`):

| Context | Bytes | Layout |
|---|---|---|
| `capsule_context` | 48 | BLAKE3 of the capsule ELF (32), the capability word (8, big-endian), the policy epoch (8, big-endian) |
| `boot_context` | 40 | the measurement (32), the boot epoch (8, big-endian) |

A kernel's measurement is BLAKE3 of its image; a loader's is the PE Authenticode SHA-256 the firmware records in the TCG log, so the kernel can rebuild it, as `boot_context` notes (`nonos-attest-path/src/context.rs:31-40`). The [capability word](../overview/glossary.md#capability-word) in a capsule's context is the manifest's `required_caps`, so a capsule enrolled with one set of rights does not open with another (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:70-73`).

The kind is `Kernel` 0, `Capsule` 1, `Pad` 2 or `Bootloader` 3 (`Kind`, `nonos-attest-path/src/leaf.rs:29-39`). `context_digest` is BLAKE3 over the domain `NONOS-ATTEST-PATH-LEAF-v3`, the context's length as a little-endian `u32`, and the context (`nonos-attest-path/src/leaf.rs:41-49`). `leaf_of` puts the domain word `NONOSLV3` in word 0 of a width-8 Poseidon state, the digest's four little-endian words in words 1 to 4 and the kind in word 5, permutes once and keeps the first four words; the kind has a word of its own so that a proof can pin it, and a capsule slot cannot open as a kernel (`nonos-attest-path/src/leaf.rs:51-71`).

The hash is Poseidon over the Goldilocks field, whose modulus `2^64 - 2^32 + 1` the code keeps as a `u64` constant (`nonos-attest-path/src/field.rs:17-18`). It has `WIDTH` 8, 32 full rounds, `x^7` on every word, a Cauchy MDS matrix and round constants from BLAKE3 of `NONOS-POSEIDON-GOLDILOCKS-RC` (`nonos-attest-path/src/params.rs:19-26`, `nonos-attest-path/src/poseidon.rs:20-56`). `verify` rebuilds the leaf, folds it up the path with `compress`, and compares the top with the root; a zero root and the padding kind admit nothing (`nonos-attest-path/src/verify.rs:22-50`).
