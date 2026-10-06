# NØNOS trust keystore — custody and production ceremony

This repository is the NØNOS trust keystore. It holds only **public** trust
material: the trust-anchor public key and policy, each capsule's publisher public
keys, signed identity certificates, signed manifests, the transparent capsule
attestation root and commitments, and the per-capsule attestation trailers.

It never holds a secret. Every private seed — the trust anchor seed, the
per-capsule publisher seeds, and the ZK enrollment/nonce seeds — lives outside
this repository and is git-ignored at the source tree (`.keys/`, `*.secret`,
`device_secrets.txt`). A verifier only ever needs the public material here plus
the baked anchor to check the whole chain; it never needs a secret. That is why
CI can verify the chain without holding any custody.

## What signs what

```text
trust anchor (ed25519 + ML-DSA-65 seed, offline)
    signs -> per-capsule NØNOS-ID certificate  (embeds the capsule publisher pubs)
per-capsule publisher key (ed25519 + ML-DSA-65 seed, offline)
    signs -> capsule manifest  (binds the ELF payload hash + capability mask + endpoints)
ZK capsule enrollment secret (offline)
    enrolls -> transparent capsule attestation root  (committed)
    proves  -> per-capsule zk_trailer  (committed; proves membership without revealing the secret)
```

The anchor and publisher signatures are hybrid: a classical Ed25519 signature and
a post-quantum ML-DSA-65 signature, both required. The attestation layer is a
transparent (no trusted setup) enrolled-secret proof.

## Custody classes

| Secret | Where it lives | Rotation impact |
|---|---|---|
| Trust anchor seed | offline signer only; pub baked into the bootloader | re-issues every capsule certificate; new anchor pub must be re-baked |
| Per-capsule publisher seed | offline signer only; pub committed here | re-signs that capsule's cert + manifest |
| ZK capsule enrollment seed | offline signer only; never committed | re-enrolls the attestation root; re-attests every trailer |
| ZK boot enrollment seed | offline signer only; never committed | re-enrolls the boot identity root |

## Production ceremony (offline, air-gapped)

Run this on a trusted machine that is not this one if you want custody that never
touched a shared environment. The public repo ships a developer-evaluation
keystore enrolled from a **public** seed so a clean checkout can boot; that public
seed is a known value and must not be used for a production release.

1. **Mint the anchor** (once; long-lived). Keep the seeds offline.

   ```sh
   capsule-sign keygen --alg ed25519 --out .keys/nonos_trust_anchor_ed25519
   capsule-sign keygen --alg mldsa65 --out .keys/nonos_trust_anchor_mldsa65
   ```

   Publish only the `.pub` files (anchor pub + policy). The bootloader bakes the
   anchor from the 32-byte public key via `NONOS_TRUST_ANCHOR_PUBKEY`; production
   build machines hold no anchor secret.

2. **Mint per-capsule publisher keys.** For each capsule bin name, generate a
   hybrid keypair; the seed stays in `.keys/`, the pub is committed here under
   `trust/keys/`.

   ```sh
   capsule-sign keygen --alg ed25519 --out .keys/<bin>_publisher_ed25519
   capsule-sign keygen --alg mldsa65 --out .keys/<bin>_publisher_mldsa65
   # move the .pub halves to nonos-data/trust/keys/
   ```

3. **Choose private enrollment secrets** with real entropy (256-bit). Never
   commit them. Example:

   ```sh
   printf 'ZK_CAPSULE_ENROLL_SEED=%s\n' "$(head -c 32 /dev/urandom | xxd -p -c 64)" >> .keys/custody/production-seeds.env
   printf 'ZK_CAPSULE_NONCE_SEED=%s\n'  "$(head -c 32 /dev/urandom | xxd -p -c 64)" >> .keys/custody/production-seeds.env
   printf 'ZK_BOOT_ENROLL_SEED=%s\n'    "$(head -c 32 /dev/urandom | xxd -p -c 64)" >> .keys/custody/production-seeds.env
   printf 'ZK_BOOT_NONCE_SEED=%s\n'     "$(head -c 32 /dev/urandom | xxd -p -c 64)" >> .keys/custody/production-seeds.env
   ```

4. **Sign, enroll, and attest the whole set** with the private seeds (no
   `NONOS_DEV`, which would substitute the public trapdoor):

   ```sh
   set -a; . .keys/custody/production-seeds.env; set +a
   make ZK_CAPSULE_ENROLL_SEED="$ZK_CAPSULE_ENROLL_SEED" \
        ZK_CAPSULE_NONCE_SEED="$ZK_CAPSULE_NONCE_SEED" \
        nonos-mk-all-capsules-attested
   ```

   This writes the committed public artifacts: certs, manifests, the attestation
   root + commitments, and every trailer. The secrets stay in `.keys/` and are
   git-ignored.

5. **Verify the chain** with no secret present, exactly as CI does:

   ```sh
   cargo run --release --manifest-path nonos-verify/Cargo.toml -- trust-chain
   cargo run --release --manifest-path nonos-verify/Cargo.toml -- attest
   ```

6. **Build the release image under the production policy.** Secure Boot and TPM
   measured boot are mandatory; a missing one halts boot.

   ```sh
   make BOOTLOADER_POLICY=production NONOS_TRUST_ANCHOR_PUBKEY=<anchor.pub raw 32B> nonos-mk-full-gui-prod
   ```

7. **Commit only public material.** Confirm `git status` in this repo shows only
   `trust/keys/*.pub`, `trust/capsules/*.{nonos_id_cert,manifest,zk_trailer}.bin`,
   `trust/policy/*.bin` — never a `.seed`, `.secret`, or `device_secrets.txt`.

## Rotation

- **Publisher key rotation** (routine): regenerate that capsule's keypair,
  re-run step 4 scoped to the capsule (`make nonos-mk-<slug>-sign`), commit the
  new pub + cert + manifest, and — because the manifest changed — re-attest.
- **Anchor rotation** (rare, high impact): mint a new anchor, re-issue every
  certificate, re-bake the bootloader anchor pub, and re-release. Treat as a
  planned trust-root migration with an epoch bump.
- **Enrollment-secret rotation**: pick new private seeds, re-run step 4 for the
  whole set (the root changes, so every trailer is re-attested), commit the new
  root + commitments + trailers.

## Assurance note

If a keystore was enrolled anywhere other than your trusted offline signer,
rotate the enrollment secrets (and, to be strict, the publisher seeds) on that
machine before a production release, then re-run steps 3–7. The anchor is the
one long-lived root; protect its seed accordingly.
