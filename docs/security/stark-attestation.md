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

## Three trees

| Tree | Root file | Made by | Checked by |
|---|---|---|---|
| capsule policy | `zk_capsule_policy_root.bin` | `nonos-stark-enroll capsules` | the kernel, at every spawn |
| kernel | `kernel_attest_root.bin` | `nonos-stark-enroll kernel` | the loader, before the jump |
| bootloader | `bootloader_attest_root.bin` | `nonos-stark-enroll bootloader` | the kernel, at boot, under the boot-root record |

All three root files sit under `nonos-data/trust/policy/` (`ZK_CAPSULE_ROOT`, `mk/00-config.mk:82-106`). Each tree has depth 8, so 256 leaves (`DEPTH`, `nonos-stark-enroll/src/context.rs:19-22`).

The kernel compiles the capsule [policy root](../overview/glossary.md#policy-root) in as a static read through `black_box`, so it stays 32 contiguous bytes, and a file of another length fails the build (`ROOT`, `src/security/capsule_attest/policy_root.rs:17-28`). The loader compiles the kernel root in as `KERNEL_ATTEST_ROOT`. A loader built without `dev-mode` or `dev-qemu` fails to build when that root is not given or is all zero, and any loader build fails on a root file that is not 32 bytes (`generate_kernel_attest_root`, `nonos-bootloader/build.rs:245-276`). The comment above that function says a development build boots on signature trust alone; that is out of date, since `attest_kernel` refuses an unattested kernel in every mode. The kernel is enrolled from a frozen copy, `kernel.enrolled.elf`, so a relink in the middle of a build cannot change the bytes under the proof (`KERNEL_ATTEST_STAMP`, `mk/20-build.mk:637-648`).

The loader's tree cannot live in the loader, whose measurement would then depend on a root that depends on it. Its root is embedded in nothing; the release signs it into the [boot-root record](../overview/glossary.md#boot-root-record), as `bootloader` in the enroll tool explains (`nonos-stark-enroll/src/commands.rs:40-48`).

## Who makes the trailers

`nonos-stark-enroll` is the host tool that builds the trees and writes the trailers. It links `nonos-attest-path` with its `alloc` feature, `nonos-boot-measure`, and from the STARKs repository the prover `stark_proofs` with `fri8` and `parallel` and the verifier `nox_verify` (`nonos-stark-enroll/Cargo.toml:12-17`).

| Verb | What it does |
|---|---|
| `capsules <root.bin> <CAPS:image:trailer> ...` | one tree over every capsule given, each with its capability word in hex |
| `kernel <image> <root.bin> <trailer.bin>` | a tree with one slot, for the kernel's BLAKE3 |
| `bootloader <image.efi> <root.bin> <trailer.bin>` | a tree with one slot, for the loader's Authenticode digest |
| `verify`, `verify-kernel`, `verify-bootloader` | check trailers again with the gate's own check |
| `recompute <root.bin.transcript>` | rebuild a root from its transcript |
| `refusal-variants` | broken trailers for the booted refusal test |
| `authenticode <image.efi>` | print a loader's Authenticode digest |
| `selftest` | the enroll, trailer and gate loop, end to end |

The verbs are in `usage` and `main` (`nonos-stark-enroll/src/main.rs:37-68`).

`enroll` takes 1 to 256 slots and starts every leaf as a padding leaf from `pad_leaf`, drawn from a 32-byte pad seed, so no padding digest is known before the tree exists (`nonos-stark-enroll/src/policy.rs:36-41`, `nonos-attest-path/src/leaf.rs:84-96`). The slots take the first leaves in order, the tree is committed, and each slot's path is folded with the gate's own `verify` before anything else (`nonos-stark-enroll/src/policy.rs:42-56`). `emit` draws the pad seed with `os_random`, which reads `/dev/urandom` (`nonos-stark-enroll/src/io.rs:55-61`), and writes the root, each trailer, and a transcript beside the root (`nonos-stark-enroll/src/commands.rs:24-34`).

`prove_v4` builds the prover's statement, then `agree` compares the prover's public words with those of `nox_verify` and refuses on any difference, so a drift between the two is an enrollment error rather than a kernel that refuses its own boot (`nonos-stark-enroll/src/stark/prove.rs:28-67`). It proves with 64 bytes of fresh entropy, makes at most three such attempts when the zero-knowledge rank certificate is not reached, and passes the result through `check_v4`, the gate's full check, before it returns (`nonos-stark-enroll/src/stark/prove.rs:36-52`, `nonos-stark-enroll/src/stark/check.rs:23-43`). A trailer the boot would refuse never leaves the tool.

`prove_all` proves slots in parallel and hands the trailers back in slot order; one proof peaks near 3 GB, so the number of workers is bounded (`nonos-stark-enroll/src/stark/batch.rs:17-41`). It runs `NONOS_ENROLL_JOBS` at a time when that is set, and otherwise half the cores, at most 8, and no more than memory allows at 3.5 GB each (`nonos-stark-enroll/src/stark/batch.rs:82-99`).

The transcript, headed `nonos-policy-transcript v2`, records the depth, both epochs, the pad seed, every slot and the root, since nothing in a tree is secret (`render`, `nonos-stark-enroll/src/transcript.rs:17-40`). `recompute` rebuilds the root through the same `enroll`, so it proves every slot again unless `NONOS_ENROLL_PATHS` is 1, which builds the paths alone (`nonos-stark-enroll/src/transcript.rs:42-82`, `nonos-stark-enroll/src/policy.rs:57-73`). The make rules build the tool at `NONOS_STARK_ENROLL`, under `nonos-stark-enroll/target/` (`mk/00-config.mk:85`). To check a published capsule root from its transcript without proving:

```sh
NONOS_ENROLL_PATHS=1 nonos-stark-enroll recompute zk_capsule_policy_root.bin.transcript
```

Not tested in this release.

The make rule for `ZK_CAPSULE_ROOT` calls `capsules` once with every capsule in `NONOS_ENROLLED_CAPSULES`, so the whole set shares one root; with `NONOS_TRUST_REUSE` set to 1 it only checks that the committed root exists (`mk/20-build.mk:592-605`). The [seal](../overview/glossary.md#seal) runs `kernel` and then `verify-kernel`, `bootloader` and then `verify-bootloader` (`tools/nonos_seal/chain.py:46-62`).
