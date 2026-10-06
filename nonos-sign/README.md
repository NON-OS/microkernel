# nonos-sign

The NONOS trust-chain toolchain: the code that mints, signs, and
verifies every identity in the system. The `capsule-sign` binary drives
it from the command line; the `nonos_capsule_sign` library is linked by
`nonos-verify` in CI and mirrors the verification the kernel and
bootloader perform.

## What it produces

The trust chain is three artifacts per capsule plus one policy at the
root, all hybrid-signed with Ed25519 and ML-DSA-65:

| Artifact | Made by | Bound to |
|----------|---------|----------|
| trust-anchor policy | `mk-trust-policy` | the anchor's two public keys and an epoch |
| NONOS-ID certificate | `sign-id-cert` | a publisher's keys, namespace globs, a capability ceiling |
| capsule manifest | `sign-manifest` | the certificate and the exact BLAKE3 of one capsule ELF |

Verification is the same chain run backwards: `verify-manifest` checks
the certificate against the policy, the manifest against the
certificate, and, with `--elf`, that a given binary measures to the
manifest's enrolled payload hash. That last check is the reuse gate:
a build pipeline that holds no seeds proves a rebuilt capsule is the
enrolled capsule instead of re-signing it.

## Key handling

`keygen` writes a seed and a public key as self-tagged binary blobs.
Seeds never belong in a repository or a pipeline: publisher seeds and
the trust-anchor seed live offline with their owners, and everything a
verifier needs is public material. The custody and rotation ceremony
is documented in the trust keystore repository.

## Building

Plain cargo. `cargo build --release --bin capsule-sign` produces the
tool; the workspace tests cover encode and decode round-trips, the
signature chain, and refusal paths.

## License

AGPL-3.0, like the rest of NONOS.
