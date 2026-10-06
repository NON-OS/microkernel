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

## The trailer

A v4 trailer carries the path and the proof side by side, little-endian, with nothing allowed after the proof (`MAGIC_V4`, `nonos-attest-path/src/v4/layout.rs:17-36`):

| Offset | Size | Field |
|---|---|---|
| 0 | 8 | `NATTV4` and two zero bytes |
| 8 | 1 | kind: 0 kernel, 1 capsule, 3 bootloader |
| 9 | 4 | n, the length of the path |
| 13 | n | the v3 path, starting with its own magic `NZKPATH1` |
| 13 + n | 4 | m, the length of the proof |
| 17 + n | m | the STARK proof, whose header names its parameter id |

The v3 path is `NZKPATH1`, one depth byte, one 32-byte sibling per level as four canonical little-endian words, then the direction bits from the leaf up, so a depth-8 path is 266 bytes (`parse`, `nonos-attest-path/src/trailer.rs:24-51`). The reader refuses a depth of 0 or above `MAX_DEPTH`, 32 (`nonos-attest-path/src/params.rs:31-34`), a depth byte other than the depth the gate expects, any length but the exact one, and set bits past the depth in the last direction byte (`parse`, `nonos-attest-path/src/trailer.rs:36-51`). `bytes_to_digest` refuses any word at or above the modulus, so each path has one encoding (`nonos-attest-path/src/trailer.rs:64-75`).

`parse_v4` refuses any other magic, a kind other than the one the caller expects, an inner path without its own magic, an empty proof or one over the caller's bound, and any byte past the proof (`nonos-attest-path/src/v4/parse.rs:29-55`). `MAX_PROOF_V4` caps any proof at 256 KiB (`nonos-attest-path/src/v4/layout.rs:28-32`), and `parse_v4` refuses outright when a caller passes a bound of 0 or one above that (`nonos-attest-path/src/v4/parse.rs:32-35`). The gates pass `ATTEST.max_proof_bytes`, the bound of the statement they check (`src/security/capsule_attest/path.rs:45`).
