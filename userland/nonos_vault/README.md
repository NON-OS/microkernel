# nonos_vault

Sealing a record to this machine, so it can be written anywhere. A sealed
record is ChaCha20-Poly1305 (`nonos_seal`) over the caller's bytes, under a
key derived from the machine root the TPM produces from its owner seed and the
boot PCRs. The blob can sit on a disk somebody takes; it opens only on this
machine in this boot state. The keyring is its one user today, for the
wallet's account key, sealed under the record name `keyring.wallet.account`.
The vault path is described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md) and the
machine key in [docs/handbook/trust/tpm.md](../../docs/handbook/trust/tpm.md).

It is a `no_std` library with no capsule, service or capability word.

## How a record is sealed

- The caller derives the machine root under `ROOT_LABEL`,
  `nonos.vault.root.v1`.
- `subkey` runs HKDF-SHA256 with that label as the salt and
  `nonos.vault.record.v1:` plus the record name (1 to 64 bytes) as the info,
  so each record has its own key.
- `seal` refuses an all-zero nonce, writes a 12-byte header (`NONOSVLT`,
  version 2, the record name's length, a reserved byte), the nonce, the
  ciphertext and the tag. The header is also the associated data.
- `open` checks length, magic, version and the record name's length before any
  key is derived, and authenticates the record's own header bytes.

## What it does not do

- It does not survive a firmware, bootloader or kernel change: a moved PCR
  changes the root and every record stops opening.
- It does not stop rollback. An older blob opens as well as a newer one. That
  would need a TPM NV counter per record; the only NV counter in use is the
  boot floor the bootloader advances.
- It is not for anything meant to be unlinkable between boots.

## Tests

13 host tests under `src/tests/`: known keys, records, fixtures and refusals.

```sh
cd userland/nonos_vault
cargo test --release
```

No CI job names this crate directly.
