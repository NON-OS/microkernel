# NONOS Kernel Signing Tool (Ed25519 and ML-DSA-65)

Command-line tool that signs a NØNOS kernel ELF with Ed25519 and ML-DSA-65 together. The Ed25519 half comes from a local 32-byte seed file or a HashiCorp Vault transit key; the ML-DSA-65 half always comes from a local seed file. The bootloader refuses a kernel unless both signatures verify. The formats and the loader's checks are described in [the signing page](../../../docs/handbook/trust/signing.md); this tool's place in the loader's host tools is in [the bootloader page](../../../docs/handbook/bootloader.md).

---

## What it does

This tool:
- Builds the signed message: the BLAKE3 hash of the kernel ELF, then the rollback index as a little-endian `u32` (36 bytes)
- Signs that message with Ed25519 (RFC 8032) and with ML-DSA-65
- Writes the kernel, then an `NKRSIG2` signature bundle, then a 64-byte `NONOSIMG` footer
- With `--verify`, reads the output back and checks both signatures
- Prints the BLAKE3 hashes of the kernel and the Ed25519 public key, and the Ed25519 public key as a Rust array

The make flow runs it with `--rollback-index $(NONOS_ROLLBACK_INDEX)` and `--verify`, then hands the result to `embed-trailer`, which appends the kernel's STARK trailer. The seal (`nix run .#seal`) runs the same two tools in the same order.

---

## Security model

- Algorithms: Ed25519 (RFC 8032) and ML-DSA-65, both required
- Key storage:
  - Ed25519: a 32-byte seed file, or a Vault transit key that never leaves Vault
  - ML-DSA-65: a `NONOSSK1` seed file and its `NONOSPK1` public file, as `capsule-sign keygen` writes them
- Signature binding: the bundle and footer follow the kernel bytes; the rollback index is inside the signed message
- Public keys: compiled into the bootloader by its `build.rs`, which checks the key ids in the bundle against them

Threat model:
- A stolen Ed25519 key alone cannot sign a kernel the loader accepts; both keys are needed
- Rotating either key means building a new bootloader

---

## Build and install

The tool is its own Cargo workspace. From this directory:
```
cargo build --release
```

Binary location:
```
target/release/sign-kernel
```

---

## CLI usage

### Local key signing

```
sign-kernel \
  --key signing_key_v1.bin \
  --mldsa65-key kernel_mldsa65.seed \
  --mldsa65-pub kernel_mldsa65.pub \
  --rollback-index 1 \
  --input target/x86_64-nonos/release/nonos-kernel \
  --output kernel_signed.bin \
  --verify
```

### Vault signing for the Ed25519 half

```
export VAULT_TOKEN="s.xxxxx"

sign-kernel \
  --vault-addr https://vault.example.com:8200 \
  --vault-key-name nonos-kernel-signing \
  --mldsa65-key kernel_mldsa65.seed \
  --mldsa65-pub kernel_mldsa65.pub \
  --rollback-index 1 \
  --input target/x86_64-nonos/release/nonos-kernel \
  --output kernel_signed.bin \
  --verify
```

### Options

| Flag | Description |
|------|-------------|
| `-k, --key FILE` | Path to the 32-byte Ed25519 seed file (conflicts with `--vault-addr`) |
| `--mldsa65-key FILE` | ML-DSA-65 seed file, required |
| `--mldsa65-pub FILE` | ML-DSA-65 public key file, required |
| `-i, --input FILE` | Kernel ELF to sign |
| `-o, --output FILE` | Output path for the signed image |
| `--rollback-index N` | Rollback index signed into the message (default 0) |
| `--vault-addr URL` | HashiCorp Vault address |
| `--vault-token TOKEN` | Vault token (or set VAULT_TOKEN env) |
| `--vault-key-name NAME` | Transit key name (default: nonos-kernel-signing) |
| `--verify` | Verify both signatures after signing |
| `-v, --verbose` | Print detailed output |

---

## Key management

### Development keys

The Ed25519 seed is 32 random bytes:
```
dd if=/dev/urandom bs=32 count=1 > signing_key_v1.bin
chmod 600 signing_key_v1.bin
```

The ML-DSA-65 pair comes from `capsule-sign keygen --alg mldsa65 --out <prefix>`. For the release keys, `tools/nonos-key-ceremony make` writes both under `nonos-bootloader/keys/`, which git ignores.

Never commit key files to git.

### Creating a Vault transit key

```
vault secrets enable transit

vault write transit/keys/nonos-kernel-signing \
  type=ed25519 \
  exportable=false \
  allow_plaintext_backup=false
```

---

## Vault integration

The tool uses Vault's transit secrets engine for secure signing:

1. Key stays in Vault (never exported)
2. Signing happens server-side via `transit/sign/:name`
3. Public key retrieved via `transit/keys/:name`

Required Vault policy:
```hcl
path "transit/sign/nonos-kernel-signing" {
  capabilities = ["update"]
}

path "transit/keys/nonos-kernel-signing" {
  capabilities = ["read"]
}
```

The tool has no flag for Vault namespaces; it always calls Vault without one.

---

## Signature format

The signed image:
```
+---------------------------+
| Kernel ELF                |  N bytes
+---------------------------+
| NKRSIG2 signature bundle  |  3445 bytes
+---------------------------+
| NONOSIMG footer           |  64 bytes
+---------------------------+
```

The bundle, in order: the magic `NKRSIG2\0` (8 bytes), the Ed25519 key id (32), the Ed25519 signature (64), the ML-DSA-65 key id (32), the ML-DSA-65 signature (3309). A key id is BLAKE3 in derive-key mode under `NONOS:KEYID:ED25519:v1` or `NONOS:KEYID:MLDSA65:v1` over the public key.

The footer, little-endian: magic `NONOSIMG`, version 1, flags 0, hash algorithm 1 (BLAKE3), signature algorithm 2 (Ed25519 plus ML-DSA-65), the total image size, the kernel's offset and size, the signature's offset and size, a zero proof offset and size, image version 1 and the rollback index. `embed-trailer` later rewrites the footer to point at the STARK trailer it appends.

Both signatures cover the 36-byte message, not the raw kernel bytes.

---

## Embedding the public keys

The tool prints the Ed25519 public key as a Rust array, but nothing pastes it anywhere. The bootloader's `build.rs` derives the Ed25519 key from the seed at `NONOS_SIGNING_KEY`, or reads the raw key file `NONOS_TRUST_ANCHOR_PUBKEY` names, and reads the ML-DSA-65 key from `NONOS_MLDSA65_PUBKEY`.

---

## Verification

The `--verify` flag reads back the signed file, checks the footer magic, and verifies both signatures over the message rebuilt from the kernel bytes and `--rollback-index`:

```
=== Verification ===
NONOSIMG footer: PRESENT
Signature verification: PASSED
ML-DSA-65 verification: PASSED
```

---

## Operational policy

- Keep both seeds out of version control.
- Sign release kernels on the machine that holds ek's keys, through the seal.
- Publish the hashes of the public keys with the release.

---

## Troubleshooting

**"Key file must be exactly 32 bytes"**
- Ed25519 seed must be 32 bytes. Generate with: `dd if=/dev/urandom bs=32 count=1 > key.bin`

**"vault connection failed"**
- Check VAULT_ADDR is reachable
- Verify network/firewall allows connection
- Check Vault is unsealed

**"permission denied" from Vault**
- Token lacks required capabilities
- Check policy includes transit/sign and transit/keys paths

**"key not found" from Vault**
- Transit key doesn't exist
- Check key name spelling
- Check that the transit engine is enabled

**"--mldsa65-key is required for kernel signing"** or **"--mldsa65-pub is required"**
- Both ML-DSA-65 files are mandatory; there is no Ed25519-only mode

**A verification failure**
- Key mismatch between signing and verification
- `--rollback-index` differs from the one signed

---

## Notes

**Two signature algorithms.** Ed25519 is small and fast; ML-DSA-65 is a post-quantum signature. The loader requires both, so breaking one algorithm is not enough to sign a kernel.

**The printed BLAKE3 hashes.** BLAKE3 is used throughout NØNOS for fingerprinting. The printed hashes let you check the kernel and public key across builds.

**More keys.** The tool signs with exactly one Ed25519 and one ML-DSA-65 signature per kernel. For key rotation, build a bootloader with the new public keys before deploying kernels signed with them.

---

## License

AGPL-3.0. See the repository LICENSE file.
