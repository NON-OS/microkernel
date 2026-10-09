# nonos_hd

BIP39 mnemonics and BIP32/BIP44 key derivation for the NONOS keyring, `no_std`
and allocation-free. The keyring uses it to turn a 12-word phrase into the
wallet's account key: PBKDF2-HMAC-SHA512 with the full 2048 rounds over the
phrase with an empty passphrase, then `derive_eth_key` walks
`m/44'/60'/0'/0/0`. Secret intermediates are wiped with volatile writes on
every path. The hashes, HMAC and the English word list are in this crate or in
`nonos_hash`.

The one primitive it does not implement is secp256k1 scalar multiplication.
`derive_eth_key` takes the parent public key from a function the caller
passes: in the keyring that is `nonos_secp256k1`, in the host tests the `k256`
crate. It is a library with no capsule, service or capability word; its users
are `capsule_keyring`, `capsule_wallet_nonos` and `nonos_vault`. The custody
model is described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Tests

10 `#[test]` functions, among them `tests/vectors.rs`, which checks the crate
against the official BIP39 vectors in `tests/data/bip39_vectors.json`, the
three BIP32 test vectors from the BIP text and known hash and HMAC answers,
with the non-hardened public keys from `k256`.

```sh
cd userland/nonos_hd
cargo test --release
```

No CI job names this crate.
