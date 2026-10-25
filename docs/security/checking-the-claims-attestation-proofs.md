# Attestation proofs verify, and altered ones are refused

`nonos-proof-verify` repeats, off the machine, the decision the gates make at
boot and at spawn: is this kernel, loader or capsule a slot of the policy tree
whose root the gate holds? It checks a v4 trailer the way the gates do, with
the same two crates: `nonos-attest-path` folds the Merkle path, and
`nox_verify` checks the STARK proof of the same slot. It builds the context
itself from the image, so a trailer passes only for the bytes, capabilities
and epoch it was enrolled for. `nonos-stark-enroll` re-checks every trailer it
writes with this same check, so enrolling and verifying are one
implementation.

## Build

```sh
cd nonos-proof-verify && cargo build --release
# target/release/nonos-proof-verify
```

The same verifier runs in a browser: `nix build .#proof-verify-web` gives
the page and the module (`nonos-proof-verify/web`). The module runs the
command line's own code over the files the page is handed, and the page
shows its report. Nothing leaves the tab.

## Commands

| Command | The gate it repeats | Measurement |
|---|---|---|
| `capsule --root R --image ELF --caps HEX TRAILER...` | the capsule spawn gate | BLAKE3 of the ELF, the capability word granted, policy epoch 1 |
| `kernel --root R --image IMAGE TRAILER...` | the loader's check of the kernel | BLAKE3 of the image, boot epoch 1 |
| `bootloader --root R --image EFI TRAILER...` | the kernel's check of its loader | the PE Authenticode SHA-256 the firmware logs, boot epoch 1 |
| `transcript FILE [--root R] TRAILER...` | the same checks, slot by slot | taken from the transcript; no image needed |

`transcript` reads a `nonos-policy-transcript v2` file (the `.transcript` the
enroll tool writes beside every root). It rebuilds the root from the slots and
the pad seed, refuses a transcript whose slots do not give the root it
records, then checks each trailer against the slot its path opens. With
`--root`, the transcript must be for that root.

To check every capsule trailer committed in the tree against the release
capsule policy root:

```sh
nonos-proof-verify transcript \
  nonos-data/trust/policy/zk_capsule_policy_root.bin.transcript \
  nonos-data/trust/capsules/*.zk_trailer.bin
```

## What a pass and a failure print

```text
nonos-proof-verify 1.0.0
root a3189f36...b1a8  the release capsule policy root (nonos-data/trust/policy/zk_capsule_policy_root.bin)
transcript ...zk_capsule_policy_root.bin.transcript: 115 slots, root reproduced
VERIFIED      capsule slot 30  nonos-data/trust/capsules/about.zk_trailer.bin  (path and STARK)
...
verdict: VERIFIED, 115 of 115, release root, STARK proofs
```

For any root other than the three committed in `nonos-data/trust/policy`, the
root line reads `development root, not a release`, and so does the verdict.
A refusal names the stage and a stable code:

```text
FAILED        capsule slot 0  a.trailer  path: the path does not fold to the root for this measurement [302]
verdict: FAILED, 1 of 1 refused
```

| Code | Stage | Meaning |
|---|---|---|
| 300 | parse | not a v4 trailer of this kind |
| 301 | development | a path-only trailer, and `--allow-dev` was not given |
| 302 | path | the path does not fold to the root for this measurement |
| 303 | root | the root is not four canonical field words |
| 304 | publics | the statement has no public words for this kind |
| 305, 306 | slot | (transcript) the slot is padding, or of another kind |
| 401 to 407 | stark | `nox_verify` refused the proof (400 plus its refusal code) |

`--allow-dev` accepts a development image's trailers (the path alone, no
STARK). Each is reported as `path only ... never a release`, and the report is
never a release's.

## Exit status

| Code | Verdict | When |
|---|---|---|
| 0 | `verified` | every trailer passed |
| 1 | `failed` | a trailer, or the transcript, was refused |
| 2 | `could-not-check` | a file could not be read, an image could not be measured, a root is not 32 canonical bytes, or the command line was wrong |

A run that could not check something never exits 0 and never prints
`VERIFIED`, even when its other trailers passed. A refusal outranks it: one
refused trailer makes the run `failed`.

## The JSON report (v1)

`--json` prints one document instead of the text:

```json
{"format":"nonos-proof-verify","version":1,"tool_version":"1.0.0",
 "verdict":"verified","exit_code":0,"release":true,
 "root":{"hex":"a318...","release":true,"name":"capsule policy root",
         "file":"nonos-data/trust/policy/zk_capsule_policy_root.bin"},
 "transcript":{"file":"...","slots":115,"root_reproduced":true},
 "refusal":null,
 "results":[{"trailer":"...","image":null,"kind":"capsule","slot":30,
             "measurement":"...","caps":"0x0000000000000059",
             "verdict":"verified","proof":"stark",
             "stage":null,"code":null,"reason":null}]}
```

- `release` is true only for a verified run under a committed release root
  where every trailer is a STARK.
- A development root has `"release":false` and
  `"note":"development root, not a release"`.
- `refusal` is a reason that covers the whole run (an unreadable root, a
  transcript that does not reproduce its root), else null.
- A field that does not apply is null, never missing.
- A later format that changes a field's meaning will carry another `version`.

## What it does not check

It checks enrollment, the path and the STARK, and nothing else:
- not a capsule's signature or manifest (`nonos-verify trust-chain`);
- not the TCG log or the boot-root approval record (`nonos-boot-measure`);
- not whether a release root was approved: it only names the root.

## Evidence

`cargo test --release` in `nonos-proof-verify` (a test lane) covers:
- **The committed material.** All three committed transcripts reproduce their
  roots. Every committed capsule trailer verifies under the release root, each
  in a slot of its own.
- **Real fixture enrollments** (`tests/fixture`): a kernel, two capsules, and
  a development (path-only) enrollment, each with real proofs. Refused:
  - every header and path byte flipped, and proof bytes across the proof;
  - the wrong root; a swapped image; each of the 64 capability bits;
  - another epoch; every cut and extension; another kind;
  - a proof spliced from another slot onto a path that folds;
  - 600 random multi-byte mutants;
  - a tampered transcript.
- **The binary.** Each exit code, the JSON document (with file names that
  need escaping), and no unchecked run ever reading as verified.

`web/smoke.mjs` runs the browser module through the page's loader. It checks
the committed material, the fixture's cases and flipped trailers, and requires
each report to match the command line's byte for byte.

`nonos-proof-verify/fuzz` holds two cargo-fuzz targets. `trailer` runs the
whole check, STARK included, against a real enrollment and admits only the
honest bytes. `transcript` runs the transcript reader.
