# NØNOS trust-chain ceremony — committed artifact set

This directory holds the public half of the NØNOS trust chain: the
trust-anchor policy the kernel bakes via `include_bytes!`, the
hybrid Ed25519 + ML-DSA-65 publisher pubkeys those policies
authorise, and the per-capsule cert + manifest the kernel verifier
checks at spawn time. Seeds never live here — they stay in
`.keys/` (gitignored) and only the host signer touches them.

## Custody status

**Scratch-ceremony.** Every key under `keys/` and every artifact
under `policy/` and `capsules/` was generated locally by
`nonos-sign capsule-sign keygen` (`ed25519` from getrandom,
`ml-dsa-65` from PQClean's reference impl) and signed by the
local Makefile pipeline. The seeds live on the developer's
disk, **not** an HSM. CI can verify the host signer ↔ kernel
verifier surface against this set; CI cannot and does not claim
production key custody.

A separate later slice will introduce HSM/offline-ceremony
custody (root key generation in an air-gapped environment,
multi-party rotation, signed log of every cert/manifest). Until
that slice lands, every artifact in this directory should be
treated as a development snapshot.

## Trust-anchor policy

Single sealed blob signed by both trust-anchor seeds; baked into
the kernel image via `src/security/nonos_trust_anchor/baked.rs`.

| Field           | Value                                |
| --------------- | ------------------------------------ |
| Algorithms      | Ed25519 + ML-DSA-65 (RequireAll)     |
| Epoch           | 1                                    |
| Valid from (ms) | 1767225600000 (2026-01-01T00:00:00Z) |
| Valid until (ms)| 1893456000000 (2030-01-01T00:00:00Z) |
| File            | `policy/nonos_trust_anchor.policy.bin` |

## Verified capsules

Every root-included `userland/*/Capsule.mk` entry ships with a
committed NØNOS-ID cert, CapsuleManifest v3, and Groth16 capsule
attestation trailer under `capsules/`. The current fleet contains
57 capsules. The verifier accepts a capsule only when the cert
decodes against the baked trust-anchor policy, the manifest verifies
under the capsule publisher keys, the manifest `payload_hash` matches
the ELF bytes, and the ZK trailer binds the capsule hash and exact
capability mask.

Per-capsule identity (handle, namespace, endpoints, caps, publisher
key paths, validity window) lives in each `userland/<capsule>/Capsule.mk`;
the shared build, sign, manifest, and attestation rules live in
`nonos-mk/capsule.mk`. The authoritative fleet list is produced by:

```
make nonos-mk-verify-capsule-attest
```

## Re-signing

Every recipe lives in the Makefile-driven pipeline:

```
make nonos-mk-trust-policy            # re-seal policy/nonos_trust_anchor.policy.bin
make nonos-mk-<slug>-sign             # re-sign one capsule's cert + manifest
make nonos-mk-host-trust-test         # decode + verify the entire baked set
```

Re-signing produces a new ML-DSA signature on every run (the
spec requires fresh per-signature randomness). The cert ID
changes; the manifest's `payload_hash` stays stable as long as
the underlying ELF bytes are identical.

## Stale-marker escape

A capsule whose committed manifest no longer matches the current
ELF can carry a `capsules/<bin>.STALE` marker file documenting
why. The host artifact test treats those capsules as
decode-only — the cryptographic chain is still validated, but
the ELF binding check is suspended until a fresh re-sign lands
and the marker is removed.

**STALE means "CI metadata temporarily stale," never
runtime-valid.** The kernel verifier never honours the marker —
at spawn time, a payload_hash mismatch is a hard fail. A capsule
with a STALE marker will not boot from a baked image until the
re-sign happens.

### Branch-protection rule

STALE markers are allowed only on dev / PR branches. On `main`,
the static-checks gate refuses to pass with any STALE marker
present:

| Branch              | STALE present | Outcome                       |
| ------------------- | ------------- | ----------------------------- |
| `main`              | none          | PASS                          |
| `main`              | any           | **FAIL** (`::error::` block)  |
| any other branch    | none          | PASS                          |
| any other branch    | any           | PASS with `::warning::`       |

The gate detects branch via `${GITHUB_REF}` in CI and via
`git rev-parse --abbrev-ref HEAD` locally. Merging a STALE
bundle to `main` would CI-pass the artifact test while the
kernel would still reject the spawn — the rule prevents that
mismatch from ever reaching the protected branch.

To clear a STALE marker:

```
make nonos-mk-<slug>-sign       # re-sign cert + manifest from current ELF
rm nonos-data/trust/capsules/<bin>.STALE
```

## Digest manifest

`MANIFEST.sha256` enumerates every committed file. Recreate it
with:

```
( cd nonos-data/trust && \
  shasum -a 256 policy/*.bin keys/*.pub capsules/*.bin \
  ) > nonos-data/trust/MANIFEST.sha256
```

## Forbidden

Seeds. `.seed` is gitignored repo-wide; the static-checks suite
also asserts `git ls-files '*.seed'` returns nothing. If a seed
ever appears here, treat it as a custody breach.
