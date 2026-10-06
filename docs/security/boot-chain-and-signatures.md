# Boot chain and signatures

Each stage of a NONOS boot is checked: with Secure Boot on, the firmware checks the loader; the loader checks the kernel before it jumps; the kernel checks the loader that started it before any program runs; and the kernel checks every [capsule](../overview/glossary.md#capsule) before it spawns one.

## The chain

```mermaid
flowchart TD
    A[UEFI firmware] --> B[BOOTX64.EFI]
    B --> C[run_crypto_verification]
    C --> D[attest_kernel]
    D --> E[commit_rollback]
    E --> F[check_bootloader]
    F --> G[verify_id_cert]
    G --> H[verify_with_publisher]
    H --> I[attest_gate]
    I --> J[spawn_verified]
```

The UEFI firmware starts the loader, `BOOTX64.EFI`. With Secure Boot on, the firmware first checks the loader's signature against the keys enrolled in it. The loader's `run_verified_boot` reads the kernel, runs `run_crypto_verification` for the signature and the [rollback index](../overview/glossary.md#rollback-index), then `attest_kernel` for the kernel's STARK trailer, parses the ELF, runs `commit_rollback` to raise the TPM floor, and only then prepares the [handoff](../overview/glossary.md#handoff) (`nonos-bootloader/src/entry/pipeline.rs:30-50`).

Early in `microkernel_main`, right after the installer check and before any userspace, the kernel runs `check_bootloader`, its own check of the loader that started it; no userspace starts when that check fails (`src/kernel_core/init/entry/microkernel_main.rs:22-27`). Every capsule then passes `preflight::run`, which calls `verify_id_cert`, `verify_with_publisher` and `attest_gate` in that order (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:36-80`). `spawn_verified` asks `profile_gate::check` whether the boot mode lets the capsule start at all, and creates the process only after all three checks pass (`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:27-63`).

## Algorithms

Key files, certificates, manifests and the trust-anchor policy name the algorithm of each key and signature in one byte, `AlgId`: `0x01` Ed25519, `0x02` ML-DSA-44, `0x03` ML-DSA-65, `0x04` ML-DSA-87 (`nonos-sign/src/algs/alg_id.rs:19-26`). The kernel decodes the same four values, but its `verify` checks only Ed25519 and ML-DSA-65 and returns `Unsupported` for the other two (`src/crypto/asymmetric/alg_id/verify.rs:23-42`). The signer refuses them earlier: `parse_alg` knows only `ed25519` and `mldsa65` (`nonos-sign/src/algs/alg_id.rs:80-86`). A signed kernel image carries no such byte: its signature blob has the fixed layout shown below.

| Algorithm | Public key | Signature |
|---|---|---|
| Ed25519 | 32 bytes | 64 bytes |
| ML-DSA-65 | 1952 bytes | 3309 bytes |

The sizes are `ED25519_PUBKEY_BYTES` and its neighbours (`src/crypto/asymmetric/alg_id/lengths.rs:19-26`). ML-DSA-65 is one of the three parameter sets of [FIPS 204](https://csrc.nist.gov/pubs/fips/204/final). The kernel builds it from PQClean's portable `clean` code under `third_party/pqclean`, and `compile_pqclean_mldsa` picks ML-DSA-65 unless the `mldsa5` or `mldsa2` feature asks for another set (`build.rs:246-260`). For ML-DSA-65 the signer's secret-key file holds the whole 4032-byte secret key, because that code has no seeded key generation, as `seed_len` notes (`nonos-sign/src/algs/alg_id.rs:56-67`).

`capsule-sign keygen` writes key files that start with a magic: `NONOSSK1` for a secret key, `NONOSPK1` for a public key (`nonos-sign/src/keys/format.rs:17-18`). Certificates and manifests need one valid signature of each algorithm: `NONOS_PRODUCTION_POLICY` lists both as required (`src/security/nonos_id_cert/policy.rs:30-32`).
