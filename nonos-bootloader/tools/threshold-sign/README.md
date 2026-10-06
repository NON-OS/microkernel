# NØNOS Threshold Signing

FROST threshold signatures on the Edwards25519 curve. Multiple parties collaborate to produce a single 64-byte Schnorr signature under one group public key.

Status: a standalone tool. Nothing in the bootloader or the kernel checks its signatures, and its key generation uses a trusted dealer. The bootloader's host tools are described in [the bootloader page](../../../docs/handbook/bootloader.md).

---

## Why threshold signatures

Traditional multi-sig requires collecting multiple separate signatures and verifying each one. If you need 3-of-5 approval, you store and verify 3 signatures.

Threshold signatures are different. The 3-of-5 parties collaborate through a two-round protocol to produce a single 64-byte signature. Anyone verifying it sees one signature and one public key. That signature could only have been created if at least 3 of the 5 keyholders participated.

During signing no party holds the full private key. During setup one party does: `threshold-keygen` draws every polynomial coefficient in one process from one RNG, so whoever runs it holds the secret until the shares are handed out and that process ends.

---

## How it works

**Key generation** splits a secret into n shares with threshold t. Each participant gets one share. The shares are constructed so that any t of them can reconstruct the signing capability, but t-1 shares reveal nothing.

**Signing** happens in two rounds:
1. Each participant generates a random commitment and shares it
2. After seeing all commitments, each participant produces a signature share
3. Any t signature shares combine into the final signature

The pieces are Shamir secret sharing, Schnorr signatures and Lagrange interpolation.

---

## The tools

```
threshold-keygen          Generate t-of-n key shares (one-time setup)
threshold-round1          Generate signing commitment (each participant)
threshold-create-package  Bundle message + commitments (coordinator)
threshold-round2          Generate signature share (each participant)
threshold-aggregate       Combine shares into final signature
threshold-verify          Verify a signature
threshold-sign            All-in-one for testing (not for production)
```

---

## Build

```
cargo build --release
```

The tool is its own Cargo workspace; run that from this directory.

```
```

---

## Distributed signing

This is how it works when participants don't trust each other and run on separate machines.

### Setup (once)

Someone trusted generates the key shares:

```
threshold-keygen -t 3 -n 5 -o keys/
```

This creates `key_share_1.json` through `key_share_5.json` plus `public_key_package.json`. Distribute each share to its participant over a secure channel. Delete the shares after distribution; the coordinator should not keep copies.

### Round 1

Each participant runs independently:

```
threshold-round1 -k my_key_share.json -c my_commitment.json -n my_nonces.json
```

They send `my_commitment.json` to the coordinator and keep `my_nonces.json` secret.

### Package creation

Coordinator collects commitments and bundles them with the message:

```
threshold-create-package -m message.bin -c commit_1.json commit_2.json commit_3.json -o signing_package.json
```

Sends `signing_package.json` back to all participants.

### Round 2

Each participant generates their signature share:

```
threshold-round2 -p signing_package.json -n my_nonces.json -k my_key_share.json -o my_sig_share.json
```

They send `my_sig_share.json` to the coordinator and delete `my_nonces.json`; nonces must never be reused.

### Aggregation

Coordinator combines the shares:

```
threshold-aggregate -p signing_package.json -s sig_1.json sig_2.json sig_3.json -k public_key_package.json -o signature.bin
```

Output is a 64-byte signature, `R` then `s`.

### Verification

Anyone can verify with just the public key:

```
threshold-verify -m message.bin -s signature.bin -k public_key_package.json
```

---

## Testing locally

For development, you can run everything on one machine:

```
threshold-keygen -t 2 -n 3 -o test_keys/

threshold-sign \
  -m message.txt \
  -k test_keys/key_share_1.json test_keys/key_share_2.json \
  -p test_keys/public_key_package.json \
  -o signature.bin

threshold-verify -m message.txt -s signature.bin -k test_keys/public_key_package.json
```

This is for testing only. Production deployments must use the distributed workflow.

---

## Security

**What is protected:**
- Secret shares are zeroized in memory on drop
- Nonces are zeroized after signing
- No party reconstructs the full private key while signing; the dealer at key generation does hold it
- t-1 colluding parties learn nothing about the key

**What you must do:**
- Distribute key shares over secure channels
- Store key shares in protected storage (HSM if possible)
- Never reuse nonces; delete them after round 2
- Verify the coordinator isn't malicious (commitments are binding)

**Domain separation:**
```
NONOS:FROST:v1           Base domain
NONOS:FROST:COMMIT:v1    Binding factor
NONOS:FROST:CHALLENGE:v1 Schnorr challenge
```

---

## File formats

**Key share** (JSON, keep secret):
```json
{
  "participant_id": 1,
  "secret_share": "hex...",
  "public_share": "hex...",
  "group_public_key": "hex...",
  "verification_shares": [...],
  "config": {"threshold": 3, "total_signers": 5}
}
```

**Public key package** (JSON, share freely):
```json
{
  "group_public_key": "hex (32 bytes)",
  "verification_shares": {"1": "hex...", ...},
  "config": {"threshold": 3, "total_signers": 5}
}
```

**Signature** (binary, 64 bytes):
```
[R: 32 bytes][s: 32 bytes]
```

The layout is Ed25519's, but the challenge is SHA-512 over the domain `NONOS:FROST:CHALLENGE:v1`, then `R`, the group key and the message. A standard Ed25519 verifier hashes `R`, the key and the message with no domain, so it refuses these signatures; check them with `threshold-verify` or `verify_signature`.

---

## Bootloader integration

None yet. The loader's kernel check takes one Ed25519 and one ML-DSA-65 signature from `sign-kernel`. `KeystoreV2` in the loader has a `verify_multisig` function, but nothing calls it, and it does not read FROST signatures.

---

## References

- FROST paper: https://eprint.iacr.org/2020/852
- Ristretto: https://ristretto.group/
- Ed25519: RFC 8032

---

## License

AGPL-3.0
