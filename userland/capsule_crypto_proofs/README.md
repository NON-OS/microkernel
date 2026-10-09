# capsule_crypto_proofs

Host test crate for the crypto pool's RSA scheme selection. It compiles the handler file
`capsule_crypto` serves through `#[path]` and checks it against a real directory authority
certificate and a SHA-1 PKCS#1 vector. It depends on the same `rsa`, `sha2`, `sha1` and
`base64ct` crates that `capsule_crypto` pins (`Cargo.toml`).

## What is under test

`src/lib.rs` mounts one capsule file:

- `../../capsule_crypto/src/server/handlers/rsa_scheme.rs` as `rsa_scheme`, which holds
  `digest_len(scheme, hashid)` and `verify(scheme, hashid, spki, sig, digest)`.

Scheme 0 is PKCS#1 v1.5 with a DigestInfo prefix, scheme 1 is PSS, and scheme 2 is PKCS#1
v1.5 over a bare digest. `hashid` 3 (20 byte SHA-1) is accepted only under schemes 0 and 2.

Local helpers: `src/cert/` (locate, PEM decode and parse the signed span, identity key and
signature of a certificate) and `src/spki.rs` (wrap a PKCS#1 key into the
SubjectPublicKeyInfo form `verify` takes). The certificate vector is
`vectors/anyone-authority-cert.txt`; per `Cargo.toml` it was fetched from an Anyone
authority on 2026-09-18. The SHA-1 vector is in `src/tests/sha1_vector.rs`.

## What the tests check

Test modules are listed in `src/tests.rs`:

- `anchor_tests.rs`: the vector's identity key hashes to the hardcoded v3 identity
  (`the_identity_key_hashes_to_the_hardcoded_v3_identity`).
- `unprefixed_tests.rs`: the certificate's own signature verifies under scheme 2, is refused
  under the prefixed schemes, and a tampered digest is refused
  (`a_real_authority_certificate_verifies_under_scheme_two`).
- `sha1_prefixed_tests.rs`: a SHA-1 DigestInfo signature, the shape an Alpine index uses,
  verifies under scheme 0; tampering is refused; PSS is never offered at SHA-1
  (`pss_is_never_offered_at_sha1`).
- `scheme_tests.rs`: digest lengths per scheme, unknown schemes refused, a bare PKCS#1 key
  refused as an argument (`an_unknown_scheme_is_refused`,
  `a_bare_pkcs1_key_is_refused_as_an_argument`).

## Running

```sh
cd userland/capsule_crypto_proofs
cargo test --release
```

At the time of writing this runs 13 tests. CI runs it: `capsule_crypto_proofs` is in the
proof crate matrix in `.github/workflows/verify.yml`, which runs `cargo test --release` and
`cargo clippy --release --all-targets -- -D warnings` with `RUST_TEST_THREADS=1`.

## Not covered

Only `rsa_scheme.rs` is mounted. The IPC framing, the other crypto pool handlers, and the
SHA-256/384/512 and PSS paths against real third-party signatures are not exercised here.
