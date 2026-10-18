# Randomness and cryptography: algorithms and tests

Part of [Randomness and cryptography](randomness-and-cryptography.md).

## Which algorithm is used where

| Algorithm | Used for | Code |
|---|---|---|
| Ed25519 and ML-DSA-65 | every [NONOS ID certificate](../overview/glossary-linux-personality.md#nonos-id-certificate) and capsule [manifest](../overview/glossary-linux-personality.md#manifest) at spawn, both required | `NONOS_PRODUCTION_POLICY`, `src/security/nonos_id_cert/policy.rs:30-32` |
| Ed25519 | the kernel's in-tree verifier | `verify`, `src/crypto/asymmetric/alg_id/verify.rs:32-39` |
| ML-DSA-65 | PQClean C code called through FFI; ML-DSA-44 and ML-DSA-87 ids are refused | `MlDsa44`, `src/crypto/asymmetric/alg_id/verify.rs:40-41` |
| BLAKE3 | the capability token MAC, two keyed hashes | `mac64`, `src/capabilities/token/material.rs:41-52` |
| SHA-256 | the kernel generator's seed hash | `sha256`, `src/crypto/util/rng/entropy/collect/pool.rs:65` |
| ChaCha20 | both generators | `chacha20_block`, `src/security/entropy_capsule/fast.rs:65`; `ChaChaRng`, `src/crypto/util/rng/csprng.rs:26` |
| ChaCha20-Poly1305 | each sector of the [data volume](../overview/glossary-claim-epoch.md#data-volume) | `aead_encrypt`, `src/fs/cryptoblock/seal.rs:41-42` |
| Keccak-256 | `CryptoKeccak256` | `keccak256`, `src/syscall/dispatch/crypto/keccak/handler.rs:42` |
| hashes, AEAD, X25519, HMAC, HKDF for capsules | `crypto_pool` | `dispatch`, `userland/capsule_crypto/src/server/dispatch.rs:28-50` |
| P-256, P-384, RSA | certificate checks in TLS, through `crypto_pool` | `crypto_status`, `userland/nonos_tls/src/verify_p256.rs:22` |

The loader's own Ed25519 and ML-DSA-65 checks of the kernel are on [Boot chain and signatures](boot-chain-and-signatures.md). ML-DSA-65 verification runs on a dedicated 256 KiB stack on x86_64 (`VERIFY_STACK_SIZE`, `src/crypto/pqc/ml_dsa_65/verify_stack.rs:24`). The kernel also carries in-tree P-256, P-384, RSA, AES-GCM and Curve25519 code under `src/crypto/asymmetric` and `src/crypto/symmetric`; the capsule's checks above use the crates instead.

## Post-quantum code in the kernel

`src/crypto/pqc` holds five algorithm modules and `quantum`, a dispatch layer over them, and only `kyber` is behind a feature (`src/crypto/pqc/mod.rs:17-28`).

| Module | In a release kernel | Origin | Row in `verification/ASSUMPTIONS.md` |
|---|---|---|---|
| `ml_dsa_65` | built, and called for every signature check | PQClean `crypto_sign/ml-dsa-65/clean` | `prim:crypto/pqc/ml_dsa_65` |
| `kyber`, ML-KEM | not built; its C is compiled | PQClean `crypto_kem/ml-kem-768/clean` | `prim:crypto/pqc/kyber` |
| `sphincs`, SPHINCS+-128s-simple | built, reached by no system call or capsule | in-tree Rust | `prim:crypto/pqc/sphincs` |
| `ntru`, NTRU-HPS-4096-821 | built, reached by no system call or capsule | in-tree Rust | `prim:crypto/pqc/ntru` |
| `mceliece`, McEliece348864 | built, reached by no system call or capsule | in-tree Rust | `prim:crypto/pqc/mceliece` |

`kyber` needs `mlkem512`, `mlkem768` or `mlkem1024`, and no [build profile](../overview/glossary-boot-root-record.md#build-profile) turns one on (`src/crypto/pqc/mod.rs:17-18`, `Cargo.toml:898-906` with `mlkem768`). `build.rs` still compiles PQClean's ML-KEM-768 C files into every kernel build, with no Rust code calling them unless a feature is on (`compile_pqclean_mlkem`, `build.rs:165-177`). It compiles ML-DSA-65 the same way unless `mldsa2` or `mldsa5` picks ML-DSA-44 or ML-DSA-87 (`compile_pqclean_mldsa`, `build.rs:246-258`). The three in-tree modules are called only by the dispatch layer `quantum`, by `src/security/quantum`, whose key operations nothing outside it calls, and, for SPHINCS+ and NTRU, by the round trips `kat_sphincs` and `kat_ntru` (`src/crypto/application/certification/kat.rs:42-76`). Nothing calls those two, and the boot self-test leaves them out of `CHECKS` on purpose (`src/crypto/application/certification/selftest.rs:32-35`). None of the three is reached by a system call or a capsule in this release.

PQClean is vendored in `third_party/pqclean`, and [PROVENANCE.md](../../third_party/pqclean/PROVENANCE.md) records why. Two of its statements do not match the tree: it lists all three ML-KEM sizes as compiled, while `build.rs` compiles one per build, and it says CI checks the pin on every push. `nonos-ci/check-pqclean-pin.sh` checks that `PROVENANCE.md` declares the pin, then hashes the committed listing of that directory, each file with its object id and `PROVENANCE.md` left out, and compares the hash with `EXPECTED` (`nonos-ci/check-pqclean-pin.sh:20-48`). No CI workflow, CI script or Make target runs it. Run on this commit, it reports drift and exits with status 1:

```sh
bash nonos-ci/check-pqclean-pin.sh
```

```text
pqclean tree drift:
  expected 2306482bedca390e1c9e83508a86a81f7f6a83a2
  actual   0fd27931b0d2587e95c0715c2d9065a07dcc7304
if intentional, update PQCLEAN_TREE_SHA in third_party/pqclean/PROVENANCE.md and this script
```

## What rests on trust

[verification/ASSUMPTIONS.md](../../verification/ASSUMPTIONS.md) lists what the security claims rest on without proof. The rows for this page are `hw:rdrand`, that RDRAND and RDSEED return unpredictable values, `stated:hash-collision` for BLAKE3, SHA-2 and SHA-3, `crate:blake3` and `crate:sha2` for the hashing crates linked into ring 0, `hw:tpm`, and one `prim:` row for each in-tree primitive, every one marked unproven. No row names PQClean, though `prim:crypto/pqc/ml_dsa_65` covers the module that calls its C code. There is no row for the two generators, and none for the crates `crypto_pool` links, since the crate rows cover only what is linked into ring 0 and into the loader. `python3 tools/nonos-assumptions` checks the register against the tree and reported `120 found, 10 stated, 0 unlisted, 0 stale` on this commit.

## Host tests

Each [proof crate](../overview/glossary-peer-list.md#proof-crate) below passed in `nix flake check` on this commit.

| Check | What it covers | Tests |
|---|---|---|
| `proofs-crypto_proofs` | the kernel's own SHA-256, SHA-512, SHA-3, BLAKE3, HMAC, HKDF, ChaCha20-Poly1305, AES-128-GCM, Ed25519, P-256, P-384 and RSA PKCS#1 v1.5 code, mounted by `#[path]` and run against RFC, NIST and BLAKE3 team vectors, RSA against a signature made with OpenSSL | 64 |
| `proofs-capsule_crypto_proofs` | `crypto_pool`'s RSA scheme choice, including SHA-1 and a real Anyone authority certificate | 13 |
| `proofs-nonos-sign` | the host signer, built from the same PQClean ML-DSA-65 sources, with a hybrid sign and verify round trip | 21 |
| `proofs-service_header_proofs` | the request header decode of `entropy_pool` and seven other services | 16 |
| `proofs-virtio_rng_proofs` | the virtio-rng driver capsule's queue and registers | 12 |

The `kernel-features-*` checks type-check the kernel with `mldsa3,mlkem768`, `mldsa2,mlkem512` and `mldsa5,mlkem1024` (`kernelFeatureSets`, `tools/nix/checks.nix:15`), and they passed too. No host test covers the two generators, the entropy capsule's sources, the kernel's ML-DSA-65 wrapper or the in-tree SPHINCS+, NTRU and McEliece code.

At boot, `run_selftest` checks SHA3-256, BLAKE3, ChaCha20-Poly1305 and Ed25519 against published answers on the machine itself (`CHECKS`, `src/crypto/application/certification/selftest.rs:34-39`). A failure is printed as `[CRYPTO-POST] FAIL` with the name, and the boot continues (`run_selftest`, `src/kernel_core/init/entry/microkernel_init.rs:58`). [Tests and proofs](../contributing/tests-and-proofs.md) explains how to run the proof crates.
